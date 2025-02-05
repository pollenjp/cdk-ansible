use anyhow::{bail, Context, Result};
use cargo_util_schemas::manifest::{
    InheritableDependency, InheritableField, InheritableSemverVersion, InheritableString,
    PackageName, TomlDependency, TomlDetailedDependency, TomlManifest, TomlPackage,
};
use cdk_ansible_cli::ModuleArgs;
use indexmap::IndexMap;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::settings::PkgUnit;

// FIXME: should be configurable
static SUB_MOD_NAME: &str = "m";

pub(crate) fn module(args: ModuleArgs) -> Result<()> {
    let args = crate::settings::ModuleSettings::resolve(args);
    let modu_names = if let Some(modu_name) = args.module_name {
        vec![modu_name]
    } else {
        let list_lines = get_ansible_modules_list()?;
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

    for modu_name in modu_names {
        println!("module_name: {}", modu_name);
        let am_name = AnsibleModuleName::new(&modu_name)
            .with_context(|| format!("failed to parse module name: {}", modu_name))?;

        // generate mod.rs for each directory
        //
        // ex) ansible.builtin.debug
        //     =>
        //     mod.rs                 -> pub mod ansible;
        //     ansible/mod.rs         -> pub mod builtin;
        //     ansible/builtin/mod.rs -> pub mod debug;
        //

        let module_json = get_module_json(GetModuleJsonArgs {
            name: &am_name,
            use_cache: args.use_cache,
            cache_dir: &args.cache_dir,
        })
        .with_context(|| format!("failed to get module json: {}", modu_name))?;

        create_rust_package_project(
            args.pkg_unit.as_ref(),
            args.pkg_prefix.as_str(),
            args.output_dir.as_path(),
            &am_name,
            &module_json,
        )?;
    }
    Ok(())
}

fn create_module_rs(modu_path: &Path, module_json: &ModuleJson) -> Result<()> {
    let content = generate_module_rs(module_json).with_context(|| {
        let module_json_str = if let Ok(module_json_str) = serde_json::to_string(&module_json) {
            module_json_str
        } else {
            dbg!(module_json);
            "failed to serialize module_json".to_string()
        };
        format!("failed to generate module: {}", module_json_str)
    })?;

    std::fs::create_dir_all(modu_path.parent().unwrap()).with_context(|| {
        format!(
            "failed to create directory for saving '<module>.rs': {}",
            modu_path.parent().unwrap().display()
        )
    })?;
    std::fs::write(modu_path, content)
        .with_context(|| format!("failed to write module file: {}", &modu_path.display()))?;
    Ok(())
}

/// '<namespace>.<collection>.<module>'
///
/// ex) ansible.builtin.debug
///   =>
///   namespace: ansible
///   collection: builtin
///   module: debug
struct AnsibleModuleName {
    pub namespace: String,
    pub collection: String,
    pub module: String,
}

impl AnsibleModuleName {
    pub fn new(modu_name: &str) -> Result<Self> {
        let parts = modu_name.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!(
                "invalid module name: {}\nPlease specify like '<namespace>.<collection>.<module>'",
                modu_name
            );
        }
        Ok(Self {
            namespace: parts[0].to_string(),
            collection: parts[1].to_string(),
            module: parts[2].to_string(),
        })
    }

    pub fn fqdn(&self) -> String {
        format!("{}.{}.{}", self.namespace, self.collection, self.module)
    }
}

fn create_rust_package_project(
    pkg_unit: Option<&PkgUnit>,
    pkg_prefix: &str,
    base_dir: &Path,
    am_name: &AnsibleModuleName,
    module_json: &ModuleJson,
) -> Result<()> {
    let result: Result<()> = if let Some(pkg_unit) = pkg_unit {
        let pkg_name = match *pkg_unit {
            PkgUnit::Namespace => format!("{}_{}", pkg_prefix, am_name.namespace),
            PkgUnit::Collection => format!(
                "{}_{}_{}",
                pkg_prefix, am_name.namespace, am_name.collection
            ),
            PkgUnit::Module => format!(
                "{}_{}_{}_{}",
                pkg_prefix, am_name.namespace, am_name.collection, am_name.module
            ),
        };

        let pkg_dir = base_dir.join(&pkg_name);
        let src_dir = pkg_dir.join("src");
        let sub_mod_dir = src_dir.join(SUB_MOD_NAME);
        let lib_rs_path = src_dir.join("lib.rs");
        for (mod_path, sub_mod_name) in [
            (lib_rs_path.clone(), SUB_MOD_NAME.to_string()),
            (sub_mod_dir.join("mod.rs"), am_name.namespace.clone()),
            (
                sub_mod_dir.join(&am_name.namespace).join("mod.rs"),
                am_name.collection.clone(),
            ),
            (
                sub_mod_dir
                    .join(&am_name.namespace)
                    .join(&am_name.collection)
                    .join("mod.rs"),
                am_name.module.clone(),
            ),
        ] {
            create_mod_rs(&mod_path, &sub_mod_name)?;
        }

        create_lib_rs(&lib_rs_path, am_name, pkg_unit)?;

        std::fs::create_dir_all(&pkg_dir).with_context(|| {
            format!(
                "failed to create directory for saving '<module>.rs': {}",
                &pkg_dir.display()
            )
        })?;
        create_cargo_toml(&pkg_name, &pkg_dir)?;
        let modu_path = sub_mod_dir
            .join(&am_name.namespace)
            .join(&am_name.collection)
            .join(&am_name.module)
            .with_extension("rs");
        create_module_rs(&modu_path, module_json)?;
        Ok(())
    } else {
        let sub_mod_dir = base_dir;
        for (mod_path, sub_mod_name) in [
            (
                sub_mod_dir.join(&am_name.namespace).join("mod.rs"),
                am_name.collection.clone(),
            ),
            (
                sub_mod_dir
                    .join(&am_name.namespace)
                    .join(&am_name.collection)
                    .join("mod.rs"),
                am_name.module.clone(),
            ),
        ] {
            create_mod_rs(&mod_path, &sub_mod_name)?;
        }
        let modu_path = sub_mod_dir
            .join(&am_name.namespace)
            .join(&am_name.collection)
            .join(&am_name.module)
            .with_extension("rs");
        create_module_rs(&modu_path, module_json)?;
        Ok(())
    };
    result.with_context(|| format!("failed to create rust package project: {}", am_name.fqdn()))
}

///
/// pkg_name: ex) cdkam_ansible_builtin
/// pkg_dir: ex) /path/to/cdkam_ansible_builtin
///
fn create_cargo_toml(pkg_name: &str, pkg_dir: &Path) -> Result<()> {
    let cargo_toml_path = pkg_dir.join("Cargo.toml");
    let manifest = TomlManifest {
        package: Some(Box::new(TomlPackage {
            name: PackageName::new(pkg_name.to_string()).unwrap(),
            version: Some(InheritableSemverVersion::Value(semver::Version::new(
                0, 1, 0,
            ))),
            edition: Some(InheritableString::Value("2021".to_string())),
            rust_version: None,
            authors: None,
            build: None,
            metabuild: None,
            default_target: None,
            forced_target: None,
            links: None,
            exclude: None,
            include: None,
            publish: None,
            workspace: None,
            im_a_teapot: None,
            autolib: None,
            autobins: None,
            autoexamples: None,
            autotests: None,
            autobenches: None,
            default_run: None,
            description: None,
            homepage: None,
            documentation: None,
            readme: None,
            keywords: None,
            categories: None,
            license: None,
            license_file: None,
            repository: None,
            resolver: None,
            metadata: None,
            _invalid_cargo_features: None,
        })),
        dependencies: Some(BTreeMap::from([
            (
                PackageName::new("anyhow".to_string()).unwrap(),
                InheritableDependency::Value(TomlDependency::Simple("1.0.95".into())),
            ),
            (
                PackageName::new("indexmap".to_string()).unwrap(),
                InheritableDependency::Value(TomlDependency::Detailed(TomlDetailedDependency {
                    version: Some("2.7.1".to_string()),
                    features: Some(vec!["serde".to_string()]),
                    ..Default::default()
                })),
            ),
            (
                PackageName::new("serde".to_string()).unwrap(),
                InheritableDependency::Value(TomlDependency::Detailed(TomlDetailedDependency {
                    version: Some("1.0.217".to_string()),
                    features: Some(vec!["derive".to_string()]),
                    ..Default::default()
                })),
            ),
            (
                PackageName::new("serde_json".to_string()).unwrap(),
                InheritableDependency::Value(TomlDependency::Detailed(TomlDetailedDependency {
                    version: Some("1.0.138".to_string()),
                    features: Some(vec!["preserve_order".to_string()]),
                    ..Default::default()
                })),
            ),
        ])),
        ..Default::default()
    };
    std::fs::write(&cargo_toml_path, ::toml::to_string(&manifest)?)
        .with_context(|| format!("failed to write cargo.toml: {}", &cargo_toml_path.display()))?;
    Ok(())
}

fn create_lib_rs(
    lib_rs_path: &Path,
    am_name: &AnsibleModuleName,
    pkg_unit: &PkgUnit,
) -> Result<()> {
    let sub_mod_path = syn::parse_str::<syn::Path>(SUB_MOD_NAME)
        .with_context(|| format!("failed to parse sub module path: {}", SUB_MOD_NAME))?;
    let pub_use_target_path = match *pkg_unit {
        PkgUnit::Namespace => syn::parse_str::<syn::Path>(
            format!("crate::{}::{}", SUB_MOD_NAME, am_name.namespace).as_str(),
        ),
        PkgUnit::Collection => syn::parse_str::<syn::Path>(
            format!(
                "crate::{}::{}::{}",
                SUB_MOD_NAME, am_name.namespace, am_name.collection
            )
            .as_str(),
        ),
        PkgUnit::Module => syn::parse_str::<syn::Path>(
            format!(
                "crate::{}::{}::{}::{}",
                SUB_MOD_NAME, am_name.namespace, am_name.collection, am_name.module
            )
            .as_str(),
        ),
    }
    .context("failed to parse pub use target path")?;
    let content = quote! {
        pub mod #sub_mod_path;
        pub use #pub_use_target_path::*;
    };
    std::fs::create_dir_all(lib_rs_path.parent().unwrap()).with_context(|| {
        format!(
            "failed to create directory for saving 'lib.rs': {}",
            &lib_rs_path.display()
        )
    })?;
    let formatted_content = format_code(&content.to_string())
        .with_context(|| format!("failed to format lib.rs: {}", &lib_rs_path.display()))?;
    std::fs::write(lib_rs_path, formatted_content)
        .with_context(|| format!("failed to write lib.rs: {}", &lib_rs_path.display()))?;
    Ok(())
}

/// If mod_path does not exist, create it and write 'pub mod <module_name>;' to it.
/// Otherwise, write 'pub mod <module_name>;' to it.
/// Finally, rustfmt.
fn create_mod_rs(mod_rs_path: &Path, sub_mod_name: &str) -> Result<()> {
    std::fs::create_dir_all(
        mod_rs_path
            .parent()
            .expect("failed to get parent directory"),
    )
    .with_context(|| {
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
        pub mod #sub_mod_name_ident;
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

struct GetModuleJsonArgs<'a> {
    name: &'a AnsibleModuleName,
    use_cache: bool,
    cache_dir: &'a PathBuf,
}

fn get_module_json(args: GetModuleJsonArgs) -> Result<ModuleJson> {
    let name = &args.name.fqdn();
    let cache_file_path = args.cache_dir.join(name);
    let output_str = if args.use_cache && cache_file_path.exists() {
        std::fs::read_to_string(&cache_file_path)
            .with_context(|| format!("failed to read cache file: {}", &cache_file_path.display()))?
            .to_string()
    } else {
        let output = std::process::Command::new("ansible-doc")
            .args(["--json", name])
            .output()
            .with_context(|| format!("failed to execute ansible-doc command: {}", name))?;
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        if args.use_cache {
            // If use_cache is False, do not save to cache
            std::fs::create_dir_all(args.cache_dir).with_context(|| {
                format!(
                    "failed to create cache directory: {}",
                    &args.cache_dir.display()
                )
            })?;
            std::fs::write(cache_file_path.with_extension("json"), &output_str).with_context(
                || format!("failed to write cache file: {}", &cache_file_path.display()),
            )?;
        }

        output_str
    };
    let module_json: ModuleJson = serde_json::from_str(&output_str)
        .with_context(|| format!("failed to parse ansible-doc output: {:?}", output_str))?;
    Ok(module_json)
}

/// list all ansible modules accessible by ansible-doc
fn get_ansible_modules_list() -> Result<Vec<String>> {
    let output = std::process::Command::new("ansible-doc")
        .args(["--list"])
        .output()
        .with_context(|| "failed to execute ansible-doc command")?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect())
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
