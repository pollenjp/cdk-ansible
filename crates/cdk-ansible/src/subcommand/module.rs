use anyhow::{Context, Result};
use cdk_ansible_cli::ModuleArgs;
use indexmap::IndexMap;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;

pub(crate) fn module(args: &ModuleArgs) -> Result<()> {
    let module_names = if let Some(module_name) = &args.module_name {
        vec![module_name.clone()]
    } else {
        let list_lines = list_ansible_modules()?;
        list_lines
            .iter()
            .map(|line| {
                let Some(module_name) = line.split(' ').next() else {
                    return Err(anyhow::anyhow!("failed to split line: {}", line));
                };
                Ok(module_name.to_string())
            })
            .collect::<Result<Vec<_>>>()?
    };

    for module_name in module_names {
        println!("module_name: {}", module_name);
        let module_name_split = module_name
            .split('.')
            .map(|s| s.replace("-", "_"))
            .collect::<Vec<_>>();

        let module_json = ansible_doc_cmd(&module_name)
            .with_context(|| format!("failed to execute ansible-doc command: {}", module_name))?;
        let output_dir = args.output_dir.clone();

        let content = generate_module_rs(&module_json).with_context(|| {
            let module_json_str = if let Ok(module_json_str) = serde_json::to_string(&module_json) {
                module_json_str
            } else {
                dbg!(&module_json);
                "failed to serialize module_json".to_string()
            };
            format!("failed to generate module: {}", module_json_str)
        })?;

        // generate mod.rs for each directory
        //
        // ex) ansible.builtin.debug
        //     =>
        //     mod.rs                 -> pub mod ansible;
        //     ansible/mod.rs         -> pub mod builtin;
        //     ansible/builtin/mod.rs -> pub mod debug;
        //
        module_name_split
            .iter()
            .scan(output_dir.clone(), |acc, dir_name| {
                let path = acc.clone();
                *acc = acc.join(dir_name);
                Some((path.join("mod.rs"), dir_name.to_string()))
            })
            .map(|(mod_path, sub_mod_name)| create_mod_rs(&mod_path, &sub_mod_name))
            .collect::<Result<Vec<_>>>()?;

        let module_path = module_name_split
            .iter()
            .fold(output_dir, |acc, dir_name| acc.join(dir_name))
            .with_extension("rs");

        std::fs::create_dir_all(module_path.parent().unwrap()).with_context(|| {
            format!(
                "failed to create directory for saving '<module>.rs': {}",
                module_path.parent().unwrap().display()
            )
        })?;
        std::fs::write(&module_path, content)
            .with_context(|| format!("failed to write module file: {}", &module_path.display()))?;
    }
    Ok(())
}

// fn generate_module_rs(module_json: &ModuleJson) -> Result<String> {

/// If mod_path does not exist, create it and write 'pub mod <module_name>;' to it.
/// Otherwise, write 'pub mod <module_name>;' to it.
/// Finally, rustfmt.
fn create_mod_rs(mod_rs_path: &Path, sub_mod_name: &str) -> Result<()> {
    std::fs::create_dir_all(mod_rs_path.parent().unwrap()).with_context(|| {
        format!(
            "failed to create directory for saving '<module>.rs': {}",
            &mod_rs_path.display()
        )
    })?;

    // if mod.rs exists or not
    let mod_rs_content = if mod_rs_path.exists() {
        std::fs::read_to_string(mod_rs_path)
            .with_context(|| format!("failed to read mod.rs: {}", &mod_rs_path.display()))?
    } else {
        String::new()
    };

    let sub_mod_name_ident = format_ident!("{}", sub_mod_name);
    let ts = quote! {
        pub(crate) mod #sub_mod_name_ident;
    };
    let formatted_ts =
        format_code(ts.to_string().as_str()).with_context(|| "failed to format code")?;

    // check mod_rs_content has 'ts.to_string()' or not
    if !mod_rs_content.contains(formatted_ts.as_str()) {
        let formatted_mod_rs_content =
            format_code([mod_rs_content, formatted_ts].join("\n").as_str())
                .with_context(|| format!("failed to format mod.rs: {}", &mod_rs_path.display()))?;
        std::fs::write(mod_rs_path, formatted_mod_rs_content)
            .with_context(|| format!("failed to write to mod.rs: {}", &mod_rs_path.display()))?;
    }
    Ok(())
}

fn ansible_doc_cmd(module_name: &str) -> Result<ModuleJson> {
    // FIXME: ansible-doc command
    // let output = std::process::Command::new("uv")
    //     .args(["run", "ansible-doc", "--json", module_name])
    let output = std::process::Command::new("ansible-doc")
        .args(["--json", module_name])
        .output()
        .with_context(|| format!("failed to execute ansible-doc command: {}", module_name))?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let module_json: ModuleJson = serde_json::from_str(&output_str)
        .with_context(|| format!("failed to parse ansible-doc output: {:?}", output_str))?;
    Ok(module_json)
}

/// list all ansible modules accessible by ansible-doc
fn list_ansible_modules() -> Result<Vec<String>> {
    let output = std::process::Command::new("ansible-doc")
        .args(["--list"])
        .output()
        .with_context(|| "failed to execute ansible-doc command")?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.split('\n').map(|s| s.to_string()).collect())
}

type ModuleJson = IndexMap<String, ModuleItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub doc: ModuleDoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDoc {
    pub options: Option<IndexMap<String, ModuleDocOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDocOption {
    // #[serde(default)]
    // pub default: Option<serde_json::Value>,
    // #[serde(default)]
    // pub description: Vec<String>,
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
}

fn generate_module_rs(module_json: &ModuleJson) -> Result<String> {
    let module_name = module_json.keys().next().unwrap();

    let struct_attributes = module_json
        .get(module_name)
        .with_context(|| format!("module name not found: {}", module_name))?
        .doc
        .options
        .clone()
        // If no options, return empty IndexMap
        .unwrap_or_else(IndexMap::new)
        .iter()
        .map(|(key, value)| {
            let key_ident = format_ident!(
                "{}",
                escape_rust_reserved_keywords(key.as_str())
                    // TODO: configure variable name's replacement rules from optional args
                    .replace("-", "__")
                    .replace("+", "___")
            );
            let type_ident = syn::parse_str::<syn::Type>(
                match value
                    .type_
                    .clone()
                    // If type is not set, implicitly set "str"
                    .unwrap_or_else(|| "str".to_string())
                    .as_str()
                {
                    "str" | "string" => "OptionUnset<String>",
                    "path" => "OptionUnset<std::path::PathBuf>",
                    "int" | "integer" => "OptionUnset<i64>",
                    "bool" | "boolean" => "OptionUnset<bool>",
                    "list" => "OptionUnset<Vec<serde_json::Value>>",
                    "dict" => "OptionUnset<indexmap::IndexMap<String, serde_json::Value>>",
                    _ => "OptionUnset<String>", // FIXME: default should be set?
                },
            )
            .with_context(|| format!("failed to parse type: {:?}", value.type_))?;
            Ok(quote! {
                #[serde(
                    default = "OptionUnset::default",
                    skip_serializing_if = "OptionUnset::is_unset"
                )]
                pub #key_ident: #type_ident,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let token_streams = vec![quote! {
        use cdk_ansible_core::core::{OptionUnset, TaskModule};
        use serde::Serialize;

        #[derive(Clone, Debug, PartialEq, Serialize)]
        pub struct Module {
            #[serde(rename = #module_name)]
            pub module: Args,
        }

        impl TaskModule for Module {}

        #[derive(Clone, Debug, PartialEq, Serialize)]
        pub struct Args {
            #[serde(flatten)]
            pub options: Opt,
        }

        #[derive(Clone, Debug, PartialEq, Default, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub struct Opt {
            #(#struct_attributes)*
        }
    }];

    let content = quote! {
        #(#token_streams)*
    }
    .to_string();

    let formatted_code = format_code(&content).with_context(|| "failed to format code")?;
    Ok(formatted_code)
}

/// https://doc.rust-lang.org/book/appendix-01-keywords.html
fn escape_rust_reserved_keywords(s: &str) -> String {
    match s {
        // Keywords Currently in Use
        "as" => "as_".to_string(),
        "async" => "async_".to_string(),
        "await" => "await_".to_string(),
        "break" => "break_".to_string(),
        "const" => "const_".to_string(),
        "continue" => "continue_".to_string(),
        "crate" => "crate_".to_string(),
        "dyn" => "dyn_".to_string(),
        "else" => "else_".to_string(),
        "enum" => "enum_".to_string(),
        "extern" => "extern_".to_string(),
        "false" => "false_".to_string(),
        "fn" => "fn_".to_string(),
        "for" => "for_".to_string(),
        "if" => "if_".to_string(),
        "impl" => "impl_".to_string(),
        "in" => "in_".to_string(),
        "let" => "let_".to_string(),
        "loop" => "loop_".to_string(),
        "match" => "match_".to_string(),
        "mod" => "mod_".to_string(),
        "move" => "move_".to_string(),
        "mut" => "mut_".to_string(),
        "pub" => "pub_".to_string(),
        "ref" => "ref_".to_string(),
        "return" => "return_".to_string(),
        "Self" => "Self_".to_string(),
        "self" => "self_".to_string(),
        "static" => "static_".to_string(),
        "struct" => "struct_".to_string(),
        "super" => "super_".to_string(),
        "trait" => "trait_".to_string(),
        "true" => "true_".to_string(),
        "type" => "type_".to_string(),
        "unsafe" => "unsafe_".to_string(),
        "use" => "use_".to_string(),
        "where" => "where_".to_string(),
        "while" => "while_".to_string(),
        // Keywords Reserved for Future Use
        "abstract" => "abstract_".to_string(),
        "become" => "become_".to_string(),
        "box" => "box_".to_string(),
        "do" => "do_".to_string(),
        "final" => "final_".to_string(),
        "macro" => "macro_".to_string(),
        "override" => "override_".to_string(),
        "priv" => "priv_".to_string(),
        "try" => "try_".to_string(),
        "typeof" => "typeof_".to_string(),
        "unsized" => "unsized_".to_string(),
        "virtual" => "virtual_".to_string(),
        "yield" => "yield_".to_string(),
        _ => s.to_string(),
    }
}

/// format code by rustfmt (requires rustfmt)
fn format_code(code: &str) -> Result<String> {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .with_context(|| "failed to spawn rustfmt")?;

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(code.as_bytes())?;
    drop(stdin);

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(anyhow::anyhow!(
            "rustfmt failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
