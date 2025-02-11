use crate::arg::SynthArgs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PkgUnit {
    Namespace,
    Collection,
    Module,
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
