use crate::arg::ModuleArgs;
use crate::settings::{ModuleSettings, PkgUnit};
use anyhow::{bail, Context as _, Result};
use core::fmt;
use indexmap::IndexMap;
use quote::{format_ident, quote};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// FIXME: should be configurable
/// The name of the submodule
/// Any name is allowed because 'pub use <name>::*' is used in 'lib.rs'
static SUB_MOD_NAME: &str = "m";

///
/// # Arguments
///
/// * `args` - [`ModuleArgs`]
///
/// # Returns
///
/// Returns a `Result` with the result of the subcommand.
///
/// # Errors
///
/// * `CliError` - If the command line arguments are invalid.
/// * `IoError` - If the configuration file is not found or cannot be read.
#[expect(clippy::single_call_fn, reason = "better readability")]
pub fn module(args: ModuleArgs) -> Result<()> {
    let args = ModuleSettings::resolve(args);
    let ans_modu_names = match (args.module_name, args.module_name_regex) {
        (Some(modu_name), None) => {
            vec![AnsibleModuleName::new(&modu_name)
                .with_context(|| format!("failed to parse module name: {modu_name}"))?]
        }
        (None, Some(regex)) => match_module_name(&regex)?,
        (None, None) => match_module_name("*")?,
        (Some(_), Some(_)) => {
            // Already rejected at argument parsing
            bail!("failed to specify both module_name and module_name_regex");
        }
    };

    for ans_modu_name in ans_modu_names {
        println!("generate '{ans_modu_name}'");
        let module_json = get_module_json(&ans_modu_name, args.use_cache, &args.cache_dir)
            .with_context(|| format!("failed to get module json: {ans_modu_name}"))?;

        create_rust_package_project(
            args.pkg_unit.as_ref(),
            args.pkg_prefix.as_str(),
            args.output_dir.as_path(),
            &ans_modu_name,
            &module_json,
        )?;
    }
    Ok(())
}

///
/// # Arguments
///
/// * `name_regex` - e.g. '<namespace>\.<collection>\.*', '<namespace>\.*'
///
fn match_module_name(name_regex: &str) -> Result<Vec<AnsibleModuleName>> {
    // parse as regex
    let regex = Regex::new(format!("^{name_regex}$").as_str())?;
    let list_lines = get_ansible_modules_list()?;
    let ans_modu_names = list_lines
        .iter()
        .filter(|line| regex.is_match(line))
        .map(|line| {
            let am_name = AnsibleModuleName::new(line)
                .with_context(|| format!("failed to parse module name: {line}"))?;
            Ok(am_name)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(ans_modu_names)
}

/// Create a module file written by Rust from the module json
///
/// # Arguments
///
/// * `modu_path` - The path to the module file
/// * `module_json` - The module json
///
fn create_module_rs(modu_path: &Path, module_json: &AnsModuleJson) -> Result<()> {
    let content = generate_module_rs(module_json).with_context(|| {
        let module_json_str = serde_json::to_string(&module_json)
            .unwrap_or_else(|e| format!("failed to serialize module_json: {e}"));
        format!("failed to generate module: {module_json_str}")
    })?;

    let dir = modu_path.parent().map_or_else(
        || bail!("failed to get parent directory: {}", &modu_path.display()),
        |dir| Ok(dir.to_path_buf()),
    )?;
    fs::create_dir_all(&dir).with_context(|| {
        format!(
            "failed to create directory for saving '<module>.rs': {}",
            &dir.display()
        )
    })?;
    fs::write(modu_path, content)
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
    /// e.g. 'ansible' in 'ansible.builtin.debug'
    pub namespace: String,
    /// e.g. 'builtin' in 'ansible.builtin.debug'
    pub collection: String,
    /// e.g. 'debug' in 'ansible.builtin.debug'
    pub module: String,
}

impl AnsibleModuleName {
    /// parse '<namespace>.<collection>.<module>' into [`AnsibleModuleName`]
    pub fn new(modu_name: &str) -> Result<Self> {
        let parts = modu_name.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            bail!("Please specify like '<namespace>.<collection>.<module>': {modu_name}");
        }
        match (parts.get(0), parts.get(1), parts.get(2)) {
            (Some(&namespace), Some(&collection), Some(&module)) => Ok(Self {
                namespace: namespace.to_owned(),
                collection: collection.to_owned(),
                module: module.to_owned(),
            }),
            _ => bail!("failed to parse module name: {modu_name}"),
        }
    }

    /// '<namespace>.<collection>.<module>'
    pub fn fqdn(&self) -> String {
        format!("{}.{}.{}", self.namespace, self.collection, self.module)
    }
}

impl fmt::Display for AnsibleModuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fqdn())
    }
}

/// Create a rust package project
///
/// # Arguments
///
/// * `pkg_unit` - [`PkgUnit`]
/// * `pkg_prefix` - e.g. `cdkam_ansible`
/// * `base_dir` - e.g. `/path/to/cdkam_ansible`
/// * `am_name` - [`AnsibleModuleName`]
/// * `module_json` - [`ModuleJson`]
///
/// # Examples
///
/// ```txt
/// `base_dir`
/// |-- Cargo.toml
/// |-- src/
///      |-- lib.rs
///      |-- m/
///          |-- ansible/
///               |-- mod.rs
///               |-- <namespace>/
///                    |-- mod.rs
///                    |-- <collection>/
///                         |-- mod.rs
///                         |-- <module>.rs
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn create_rust_package_project(
    pkg_unit: Option<&PkgUnit>,
    pkg_prefix: &str,
    base_dir: &Path,
    am_name: &AnsibleModuleName,
    module_json: &AnsModuleJson,
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

        // generate mod.rs for each directory
        //
        // ex) ansible.builtin.debug
        //     =>
        //     mod.rs                 -> pub mod ansible;
        //     ansible/mod.rs         -> pub mod builtin;
        //     ansible/builtin/mod.rs -> pub mod debug;
        //
        for (mod_path, sub_mod_name) in [
            (lib_rs_path.clone(), SUB_MOD_NAME.to_owned()),
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

        fs::create_dir_all(&pkg_dir).with_context(|| {
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
/// # Arguments
///
/// * `pkg_name` - e.g. `cdkam_ansible_builtin`
/// * `pkg_dir` - e.g. `/path/to/cdkam_ansible_builtin`
///
/// # Returns
///
/// Returns a `Result` with the result of the subcommand.
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn create_cargo_toml(pkg_name: &str, pkg_dir: &Path) -> Result<()> {
    let cargo_toml_path = pkg_dir.join("Cargo.toml");

    // FIXME: some values should be configurable
    //
    // [package]
    // edition = "2021"
    // name = "cdkam_ansible"
    // version = "0.1.0"
    // [dependencies]
    // cdk-ansible.workspace = true
    // anyhow = "1.0.95"
    // indexmap = { version = "2.7.1", features = ["serde"] }
    // serde = { version = "1.0.217", features = ["derive"] }
    // serde_json = { version = "1.0.138", features = ["preserve_order"] }

    let mut manifest = cargo_toml::Manifest::from_str(
        r#"
        [package]
        name = "sample"
        version = "0.1.0"
        edition = "2021"
        rust-version = "1.83"
        "#,
    )?;
    if let Some(package) = manifest.package.as_mut() {
        pkg_name.clone_into(&mut package.name);
    }
    manifest.dependencies = vec![
        (
            "cdk-ansible".to_owned(),
            cargo_toml::Dependency::Inherited(cargo_toml::InheritedDependencyDetail {
                workspace: true,
                ..Default::default()
            }),
        ),
        (
            "anyhow".to_owned(),
            cargo_toml::Dependency::Simple("1.0.95".to_owned()),
        ),
        (
            "indexmap".to_owned(),
            cargo_toml::Dependency::Simple("2.7.1".to_owned()),
        ),
        (
            "serde".to_owned(),
            cargo_toml::Dependency::Simple("1.0.217".to_owned()),
        ),
        (
            "serde_json".to_owned(),
            cargo_toml::Dependency::Simple("1.0.138".to_owned()),
        ),
    ]
    .into_iter()
    .collect();

    fs::write(&cargo_toml_path, ::toml::to_string(&manifest)?)
        .with_context(|| format!("failed to write cargo.toml: {}", &cargo_toml_path.display()))?;
    Ok(())
}

/// Create a lib.rs file
///
/// # Arguments
///
/// * `lib_rs_path` - e.g. `/path/to/cdkam_ansible/src/lib.rs`
/// * `am_name` - [`AnsibleModuleName`]
/// * `pkg_unit` - [`PkgUnit`]
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn create_lib_rs(
    lib_rs_path: &Path,
    am_name: &AnsibleModuleName,
    pkg_unit: &PkgUnit,
) -> Result<()> {
    let sub_mod_path = syn::parse_str::<syn::Path>(SUB_MOD_NAME)
        .with_context(|| format!("failed to parse sub module path: {SUB_MOD_NAME}"))?;
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
    let lib_dir = lib_rs_path
        .parent()
        .with_context(|| format!("failed to get parent directory: {}", &lib_rs_path.display()))?;
    fs::create_dir_all(lib_dir).with_context(|| {
        format!(
            "failed to create directory for saving 'lib.rs': {}",
            &lib_rs_path.display()
        )
    })?;
    let formatted_content = format_code(&content.to_string())
        .with_context(|| format!("failed to format lib.rs: {}", &lib_rs_path.display()))?;
    fs::write(lib_rs_path, formatted_content)
        .with_context(|| format!("failed to write lib.rs: {}", &lib_rs_path.display()))?;
    Ok(())
}

/// If `mod_path` does not exist, create it and write `pub mod <module_name>;` to it.
/// Otherwise, write `pub mod <module_name>;` to it.
/// Finally, rustfmt.
///
/// # Arguments
///
/// * `mod_rs_path` - e.g. `/path/to/cdkam_ansible/src/m/ansible/mod.rs`
/// * `sub_mod_name` - e.g. 'ansible'
///
fn create_mod_rs(mod_rs_path: &Path, sub_mod_name: &str) -> Result<()> {
    let dir = mod_rs_path
        .parent()
        .with_context(|| format!("failed to get parent directory: {}", mod_rs_path.display()))?;
    fs::create_dir_all(dir).with_context(|| {
        format!(
            "failed to create directory for saving '<module>.rs': {}",
            dir.display()
        )
    })?;

    // if mod.rs exists or not
    let mod_rs_content = if mod_rs_path.exists() {
        fs::read_to_string(mod_rs_path)
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
        fs::write(mod_rs_path, formatted_mod_rs_content)
            .with_context(|| format!("failed to write to mod.rs: {}", &mod_rs_path.display()))?;
    }
    Ok(())
}

/// Get module json
///
/// # Arguments
///
/// * `name` - [`AnsibleModuleName`]
/// * `use_cache` - Whether to use cache
/// * `cache_dir` - Cache directory
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn get_module_json(
    name: &AnsibleModuleName,
    use_cache: bool,
    cache_dir: &PathBuf,
) -> Result<AnsModuleJson> {
    let name = name.fqdn();
    let cache_file_path = cache_dir.join(&name);
    let output_str = if use_cache && cache_file_path.exists() {
        fs::read_to_string(&cache_file_path)
            .with_context(|| format!("failed to read cache file: {}", &cache_file_path.display()))?
    } else {
        let output = Command::new("ansible-doc")
            .args(["--json", name.as_str()])
            .output()
            .with_context(|| format!("failed to execute 'ansible-doc --json {name}'"))?;
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        if use_cache {
            // If use_cache is False, do not save to cache
            fs::create_dir_all(cache_dir).with_context(|| {
                format!("failed to create cache directory: {}", &cache_dir.display())
            })?;
            fs::write(cache_file_path.with_extension("json"), &output_str).with_context(|| {
                format!("failed to write cache file: {}", &cache_file_path.display())
            })?;
        }

        output_str
    };
    let module_json: AnsModuleJson = serde_json::from_str(&output_str)
        .with_context(|| format!("failed to parse ansible-doc output: {output_str}"))?;
    Ok(module_json)
}

/// list all ansible module names accessible by ansible-doc
#[expect(clippy::single_call_fn, reason = "better readability")]
fn get_ansible_modules_list() -> Result<Vec<String>> {
    let output = Command::new("ansible-doc")
        .args(["--list"])
        .output()
        .with_context(|| "failed to execute 'ansible-doc --list'")?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let names = output_str
        .split('\n')
        .map(String::from)
        .filter(|s| !s.is_empty())
        .map(|line| {
            let Some(module_name) = line.split(' ').next() else {
                bail!("failed to split line: {line}")
            };
            Ok(module_name.to_owned())
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(names)
}

/// Ansible Module Json Type (`TypeAlias`)
type AnsModuleJson = IndexMap<String, AnsModuleItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// module field
pub struct AnsModuleItem {
    /// 'doc' field
    pub doc: AnsModuleDoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// doc field
pub struct AnsModuleDoc {
    /// 'options' field
    pub options: Option<IndexMap<String, AnsModuleDocOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// doc option field
pub struct AnsModuleDocOption {
    // TODO: add description field
    // #[serde(default)]
    // pub description: Vec<String>,
    /// 'type' field
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
}

/// generate module rs
///
/// # Arguments
///
/// * `module_json` - [`ModuleJson`]
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn generate_module_rs(module_json: &AnsModuleJson) -> Result<String> {
    let Some(module_name) = module_json.keys().next() else {
        bail!("module_json does not have any key: {module_json:?}")
    };

    let struct_attributes = module_json
        .get(module_name)
        .with_context(|| format!("module name not found: {module_name}"))?
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
                    .replace('-', "_x_")
                    .replace('+', "_xx_")
            );
            let type_ident = syn::parse_str::<syn::Type>(
                match value
                    .type_
                    .clone()
                    // If type is not set, implicitly set "str"
                    .unwrap_or_else(|| "str".to_owned())
                    .as_str()
                {
                    "path" => "OptU<std::path::PathBuf>",
                    "int" | "integer" => "OptU<i64>",
                    "bool" | "boolean" => "OptU<bool>",
                    "list" => "OptU<Vec<serde_json::Value>>",
                    "dict" => "OptU<indexmap::IndexMap<String, serde_json::Value>>",
                    // `"str" | "string"` or default should be [`OptU<String>`]
                    _ => "OptU<String>",
                },
            )
            .with_context(|| format!("failed to parse type: {:?}", value.type_))?;
            Ok(quote! {
                #[serde(
                    default = "OptU::default",
                    skip_serializing_if = "OptU::is_unset"
                )]
                pub #key_ident: #type_ident,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let token_streams = vec![quote! {
        #[allow(unused_imports, reason = "Some modules may have empty `options` field")]
        use cdk_ansible::OptU;
        use cdk_ansible::TaskModule;
        use serde::Serialize;

        #[derive(Clone, Debug, Serialize)]
        pub struct Module {
            #[serde(rename = #module_name)]
            pub module: Args,
        }

        impl TaskModule for Module {}

        #[derive(Clone, Debug, Serialize)]
        pub struct Args {
            #[serde(flatten)]
            pub options: Opt,
        }

        #[derive(Clone, Debug, Default, Serialize)]
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

/// Escape rust reserved keywords
///
/// <https://doc.rust-lang.org/book/appendix-01-keywords.html>
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn escape_rust_reserved_keywords(s: &str) -> String {
    match s {
        // Keywords Currently in Use
        "as" => "as_".to_owned(),
        "async" => "async_".to_owned(),
        "await" => "await_".to_owned(),
        "break" => "break_".to_owned(),
        "const" => "const_".to_owned(),
        "continue" => "continue_".to_owned(),
        "crate" => "crate_".to_owned(),
        "dyn" => "dyn_".to_owned(),
        "else" => "else_".to_owned(),
        "enum" => "enum_".to_owned(),
        "extern" => "extern_".to_owned(),
        "false" => "false_".to_owned(),
        "fn" => "fn_".to_owned(),
        "for" => "for_".to_owned(),
        "if" => "if_".to_owned(),
        "impl" => "impl_".to_owned(),
        "in" => "in_".to_owned(),
        "let" => "let_".to_owned(),
        "loop" => "loop_".to_owned(),
        "match" => "match_".to_owned(),
        "mod" => "mod_".to_owned(),
        "move" => "move_".to_owned(),
        "mut" => "mut_".to_owned(),
        "pub" => "pub_".to_owned(),
        "ref" => "ref_".to_owned(),
        "return" => "return_".to_owned(),
        "Self" => "Self_".to_owned(),
        "self" => "self_".to_owned(),
        "static" => "static_".to_owned(),
        "struct" => "struct_".to_owned(),
        "super" => "super_".to_owned(),
        "trait" => "trait_".to_owned(),
        "true" => "true_".to_owned(),
        "type" => "type_".to_owned(),
        "unsafe" => "unsafe_".to_owned(),
        "use" => "use_".to_owned(),
        "where" => "where_".to_owned(),
        "while" => "while_".to_owned(),
        // Keywords Reserved for Future Use
        "abstract" => "abstract_".to_owned(),
        "become" => "become_".to_owned(),
        "box" => "box_".to_owned(),
        "do" => "do_".to_owned(),
        "final" => "final_".to_owned(),
        "macro" => "macro_".to_owned(),
        "override" => "override_".to_owned(),
        "priv" => "priv_".to_owned(),
        "try" => "try_".to_owned(),
        "typeof" => "typeof_".to_owned(),
        "unsized" => "unsized_".to_owned(),
        "virtual" => "virtual_".to_owned(),
        "yield" => "yield_".to_owned(),
        _ => s.to_owned(),
    }
}

/// format code by rustfmt (requires rustfmt)
fn format_code(code: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "failed to spawn rustfmt")?;

    let Some(mut stdin) = child.stdin.take() else {
        bail!("failed to take stdin: rustfmt");
    };
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
