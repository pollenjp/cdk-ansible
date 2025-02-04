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
pub struct ModuleSettings {
    pub output_dir: PathBuf,
    pub use_cache: bool,
    pub cache_dir: PathBuf,
    pub module_name: Option<String>,
}

impl ModuleSettings {
    pub fn resolve(args: ModuleArgs) -> Self {
        Self {
            output_dir: args.output_dir.unwrap_or_else(|| ".cdk-ansible.out".into()),
            use_cache: !args.no_cache,
            cache_dir: args
                .cache_dir
                .unwrap_or_else(|| ".cdk-ansible.cache.out".into()),
            module_name: args.module_name,
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
