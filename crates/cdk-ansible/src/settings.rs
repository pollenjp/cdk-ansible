use cdk_ansible_cli::{GlobalArgs, ModuleArgs, SynthArgs};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GlobalSettings {
    pub quiet: bool,
    pub verbose: u8,
}

impl GlobalSettings {
    pub fn resolve(args: GlobalArgs) -> Self {
        Self {
            quiet: args.quiet,
            verbose: args.verbose,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PkgUnit {
    Namespace,
    Collection,
    Module,
}

#[derive(Debug, Clone)]
pub struct ModuleSettings {
    pub output_dir: PathBuf,
    pub pkg_prefix: String,
    pub pkg_unit: Option<PkgUnit>,
    pub use_cache: bool,
    pub cache_dir: PathBuf,
    pub module_name: Option<String>,
    pub module_name_regex: Option<String>,
}

impl ModuleSettings {
    pub fn resolve(args: ModuleArgs) -> Self {
        Self {
            output_dir: args.output_dir.unwrap_or_else(|| ".cdk-ansible.out".into()),
            pkg_prefix: args.pkg_prefix.unwrap_or_else(|| {
                // CDK Ansible Module
                "cdkam".to_string()
            }),
            pkg_unit: match args.pkg_unit {
                Some(cdk_ansible_cli::PkgUnit::Namespace) => Some(PkgUnit::Namespace),
                Some(cdk_ansible_cli::PkgUnit::Collection) => Some(PkgUnit::Collection),
                Some(cdk_ansible_cli::PkgUnit::Module) => Some(PkgUnit::Module),
                Some(cdk_ansible_cli::PkgUnit::None) => None,
                None => Some(PkgUnit::Namespace), // default value
            },
            use_cache: !args.no_cache,
            cache_dir: args
                .cache_dir
                .unwrap_or_else(|| ".cdk-ansible.cache.out".into()),
            module_name: args.module_name,
            module_name_regex: args.module_name_regex,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SynthSettings {
    pub output_dir: PathBuf,
    pub playbook_dir: PathBuf,
    pub inventory_dir: PathBuf,
}

impl SynthSettings {
    pub fn resolve(args: SynthArgs) -> Self {
        Self {
            output_dir: args.output_dir.clone(),
            playbook_dir: args
                .playbooks_dir
                .unwrap_or_else(|| args.output_dir.join("playbooks")),
            inventory_dir: args
                .inventory_dir
                .unwrap_or_else(|| args.output_dir.join("inventory")),
        }
    }
}
