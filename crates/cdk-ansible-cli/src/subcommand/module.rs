use crate::arg::ModuleArgs;
use crate::settings::{ModuleSettings, PkgUnit};
use anyhow::{Context as _, Result, bail};
use core::fmt;
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
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
pub fn module(args: ModuleArgs) -> Result<()> {
    let args = ModuleSettings::resolve(args);
    let ans_modu_names = match (args.module_name, args.module_name_regex) {
        (Some(modu_name), None) => {
            vec![
                AnsibleModuleName::new(&modu_name)
                    .with_context(|| format!("failed to parse module name: {modu_name}"))?,
            ]
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

    /// e.g.
    /// - `<pkg_prefix>_<namespace>`,
    /// - `<pkg_prefix>_<namespace>_<collection>`,
    /// - `<pkg_prefix>_<namespace>_<collection>_<module>`
    pub fn pkg_name(&self, pkg_prefix: &str, pkg_unit: Option<&PkgUnit>) -> String {
        match pkg_unit {
            None => pkg_prefix.to_owned(),
            Some(&PkgUnit::Namespace) => format!("{}_{}", pkg_prefix, self.namespace),
            Some(&PkgUnit::Collection) => {
                format!("{}_{}_{}", pkg_prefix, self.namespace, self.collection)
            }
            Some(&PkgUnit::Module) => format!(
                "{}_{}_{}_{}",
                pkg_prefix, self.namespace, self.collection, self.module
            ),
        }
    }

    /// e.g. 'ansible-builtin-debug'
    pub fn feature_name(&self, pkg_unit: &PkgUnit) -> String {
        match *pkg_unit {
            PkgUnit::Namespace => self.namespace.clone(),
            PkgUnit::Collection => format!("{}-{}", self.namespace, self.collection),
            PkgUnit::Module => format!("{}-{}-{}", self.namespace, self.collection, self.module),
        }
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
///          |-- <namespace>/
///               |-- mod.rs
///               |-- <collection>/
///                    |-- mod.rs
///                    |-- <module>.rs
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn create_rust_package_project(
    pkg_unit: Option<&PkgUnit>,
    pkg_prefix: &str,
    base_dir: &Path,
    am_name: &AnsibleModuleName,
    module_json: &AnsModuleJson,
) -> Result<()> {
    let result: Result<()> = {
        let pkg_name = am_name.pkg_name(pkg_prefix, pkg_unit);
        let pkg_dir = base_dir.join(&pkg_name);
        let src_dir = pkg_dir.join("src");
        let lib_rs_path = src_dir.join("lib.rs");
        let sub_mod_dir = src_dir.join(SUB_MOD_NAME);

        // generate mod.rs for each directory
        //
        // ex) ansible.builtin.debug
        //     =>
        //     mod.rs                 -> pub mod ansible;
        //     ansible/mod.rs         -> pub mod builtin;
        //     ansible/builtin/mod.rs -> pub mod debug;
        //

        // Add 'pub use m::<namespace>::*' to root/src/lib.rs
        create_lib_rs(&lib_rs_path, am_name, pkg_unit)?;
        // Add 'pub mod <namespace>' to root/src/m/mod.rs
        create_mod_rs(
            &sub_mod_dir.join("mod.rs"),
            &am_name.namespace.clone(),
            None,
        )?;
        // Add 'pub mod <collection>' to root/src/m/<namespace>/mod.rs
        create_mod_rs(
            &sub_mod_dir.join(&am_name.namespace).join("mod.rs"),
            &am_name.collection,
            None,
        )?;
        // Add 'pub mod <module>' to root/src/m/<namespace>/<collection>/mod.rs
        create_mod_rs(
            &sub_mod_dir
                .join(&am_name.namespace)
                .join(&am_name.collection)
                .join("mod.rs"),
            &am_name.module,
            Some(CfgAttr {
                feature: am_name.feature_name(&PkgUnit::Module),
            }),
        )?;

        fs::create_dir_all(&pkg_dir).with_context(|| {
            format!(
                "failed to create directory for saving '<module>.rs': {}",
                &pkg_dir.display()
            )
        })?;
        create_or_edit_cargo_toml(am_name, &pkg_dir, &pkg_name)?;

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
fn create_or_edit_cargo_toml(
    am_name: &AnsibleModuleName,
    pkg_dir: &Path,
    pkg_name: &str,
) -> Result<()> {
    let cargo_toml_path = pkg_dir.join("Cargo.toml");

    // FIXME: some values should be configurable
    //
    // ```Cargo.toml
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
    // ```

    if !cargo_toml_path.exists() {
        let mut manifest = ::cargo_toml::Manifest::from_str(&format!(
            "[package]
name = \"{pkg_name}\"
version = \"0.1.0\"
edition = \"2024\"
rust-version = \"1.85\"
"
        ))?;
        if let Some(package) = manifest.package.as_mut() {
            pkg_name.clone_into(&mut package.name);
        }
        manifest.dependencies = vec![
            (
                "cdk-ansible".to_owned(),
                ::cargo_toml::Dependency::Inherited(::cargo_toml::InheritedDependencyDetail {
                    workspace: true,
                    ..Default::default()
                }),
            ),
            (
                "anyhow".to_owned(),
                ::cargo_toml::Dependency::Simple("1.0.95".to_owned()),
            ),
            (
                "indexmap".to_owned(),
                ::cargo_toml::Dependency::Simple("2.7.1".to_owned()),
            ),
            (
                "serde".to_owned(),
                ::cargo_toml::Dependency::Simple("1.0.217".to_owned()),
            ),
            (
                "serde_json".to_owned(),
                ::cargo_toml::Dependency::Simple("1.0.138".to_owned()),
            ),
        ]
        .into_iter()
        .collect();

        fs::write(&cargo_toml_path, ::toml::to_string(&manifest)?).with_context(|| {
            format!("failed to write Cargo.toml: {}", &cargo_toml_path.display())
        })?;
    }

    let toml_text = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("failed to read Cargo.toml: {}", &cargo_toml_path.display()))?;
    let mut override_toml = toml_text
        .parse::<::toml_edit::DocumentMut>()
        .with_context(|| {
            format!("Failed to parse toml as toml_edit::DocumentMut: {toml_text:?}",)
        })?;

    // Set '[features]' as table
    let mut features_table: ::toml_edit::Table = override_toml.get("features").map_or_else(
        || Ok(::toml_edit::Table::new()),
        |v| {
            v.as_table()
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("failed to get table from toml_edit::Item: {v:?}"))
        },
    )?;

    #[expect(clippy::indexing_slicing, reason = "toml_edit convention")]
    {
        features_table["default"].or_insert(::toml_edit::array());

        // ```rs
        // [features]
        // default = []
        // ansible = ["ansible-builtin"]
        // ansible-builtin = ["ansible-builtin-debug", ...]
        // ansible-builtin-debug = []
        // ansible-builtin-<...> = []
        // ...
        // ```

        let ns_feat_name = am_name.feature_name(&PkgUnit::Namespace);
        let coll_feat_name = am_name.feature_name(&PkgUnit::Collection);
        let modu_feat_name = am_name.feature_name(&PkgUnit::Module);
        let ns_feat = add_str_and_sort_array_without_duplication(
            features_table.get(&ns_feat_name),
            vec![coll_feat_name.clone()],
        )?;
        features_table[&ns_feat_name] = ns_feat;
        let coll_feat = add_str_and_sort_array_without_duplication(
            features_table.get(&coll_feat_name),
            vec![modu_feat_name.clone()],
        )?;
        features_table[&coll_feat_name] = coll_feat;
        let modu_feat = add_str_and_sort_array_without_duplication(
            features_table.get(&modu_feat_name),
            vec![],
        )?;
        features_table[&modu_feat_name] = modu_feat;

        override_toml["features"] = ::toml_edit::Item::Table(features_table);
    };

    fs::write(&cargo_toml_path, override_toml.to_string())
        .with_context(|| format!("failed to write Cargo.toml: {}", &cargo_toml_path.display()))?;

    Ok(())
}

fn add_str_and_sort_array_without_duplication(
    arr_item: Option<&::toml_edit::Item>,
    values: Vec<String>,
) -> Result<::toml_edit::Item> {
    let arr_item = arr_item
        .cloned()
        .unwrap_or_else(|| ::toml_edit::value(::toml_edit::Array::new()));
    let orig_arr = arr_item
        .as_array()
        .cloned()
        .with_context(|| "failed to as_array_mut from toml_edit::Item")?;
    // sort and remove duplicates
    let mut set = BTreeSet::<String>::new();
    for v in &orig_arr {
        let s = v
            .as_str()
            .with_context(|| "failed to as_str from toml_edit::Value")?
            .to_owned();
        set.insert(s);
    }
    for v in values {
        set.insert(v);
    }
    // Construct sorted array
    let mut ret_arr = ::toml_edit::Array::new();
    for v in set {
        ret_arr.push(v);
    }
    Ok(::toml_edit::value(ret_arr))
}

/// Create a lib.rs file, such as:
///
/// ```rs
/// pub mod m;
/// pub use m::*;
/// // or
/// pub use m::<namespace>::*;
/// // or
/// pub use m::<namespace>::<collection>::*;
/// // or
/// pub use m::<namespace>::<collection>::<module>::*;
/// ```
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
    pkg_unit: Option<&PkgUnit>,
) -> Result<()> {
    let sub_mod_path = syn::parse_str::<syn::Path>(SUB_MOD_NAME)
        .with_context(|| format!("failed to parse sub module path: {SUB_MOD_NAME}"))?;
    let pub_use_target_path = match pkg_unit {
        // pub use m::*;
        None => syn::parse_str::<syn::Path>(format!("crate::{SUB_MOD_NAME}").as_str()),
        // pub use m::<namespace>::*;
        Some(&PkgUnit::Namespace) => syn::parse_str::<syn::Path>(
            format!("crate::{}::{}", SUB_MOD_NAME, am_name.namespace).as_str(),
        ),
        // pub use m::<namespace>::<collection>::*;
        Some(&PkgUnit::Collection) => syn::parse_str::<syn::Path>(
            format!(
                "crate::{}::{}::{}",
                SUB_MOD_NAME, am_name.namespace, am_name.collection
            )
            .as_str(),
        ),
        // pub use m::<namespace>::<collection>::<module>::*;
        Some(&PkgUnit::Module) => syn::parse_str::<syn::Path>(
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

struct CfgAttr {
    pub feature: String,
}

impl CfgAttr {
    pub fn to_token(&self) -> TokenStream {
        let feature = &self.feature;
        quote! {
            #[cfg(feature = #feature)]
        }
    }
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
fn create_mod_rs(mod_rs_path: &Path, sub_mod_name: &str, cfg_attr: Option<CfgAttr>) -> Result<()> {
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
    let cfg_attr = cfg_attr.map_or_else(|| quote! {}, |cfg_attr| cfg_attr.to_token());
    let ts = quote! {
        #cfg_attr
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
    let module_json: AnsModuleJson = ::serde_json::from_str(&output_str)
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
struct AnsModuleItem {
    /// 'doc' field
    pub doc: AnsModuleDoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// doc field
struct AnsModuleDoc {
    /// 'options' field
    pub options: Option<IndexMap<String, AnsModuleDocOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// doc option field
struct AnsModuleDocOption {
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
                    .replace('-', "_xx_")
                    .replace('+', "_xxx_")
            );
            let type_ident = syn::parse_str::<syn::Type>(
                match value
                    .type_
                    .clone()
                    // If type is not set, implicitly set "str"
                    .unwrap_or_else(|| "str".to_owned())
                    .as_str()
                {
                    // FIXME: always include "string" because ansible can use template.
                    "path" => "OptU<std::path::PathBuf>",
                    "int" | "integer" => "OptU<i64>",
                    "bool" | "boolean" => "OptU<bool>",
                    "list" => "OptU<Vec<::serde_json::Value>>",
                    "dict" => "OptU<indexmap::IndexMap<String, ::serde_json::Value>>",
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
/// <https://doc.rust-lang.org/reference/keywords.html>
///
#[expect(clippy::single_call_fn, reason = "better readability")]
fn escape_rust_reserved_keywords(s: &str) -> String {
    match s {
        // Strict keywords
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
        | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super" | "trait"
        | "true" | "type" | "unsafe" | "use" | "where" | "while" | "async" | "await" | "dyn"
        // Reserved keywords
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
        | "typeof" | "unsized" | "virtual" | "yield" | "try" | "gen"
        // Weak keywords
        | "macro_rules" | "union"  | "safe" | "raw" => {
            s.to_owned() + "_x_"
        }
        // Weak keywords
        "'static" => "x_static_x_".to_owned(),
        _ => s.to_owned(), // do nothing
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
