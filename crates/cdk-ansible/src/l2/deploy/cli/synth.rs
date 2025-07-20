use crate::{
    l2::deploy::{AppL2, cli::GlobalConfig},
    types::ExePlaybook,
    utils::{dump_json, json_to_yaml, playbook_dump},
};
use anyhow::Result;
use cdk_ansible_core::core::Playbook;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::task::JoinSet;

#[derive(Args, Debug, Clone)]
pub struct Synth {}

impl Synth {
    pub async fn run(self, app: &AppL2, global_config: Arc<GlobalConfig>) -> Result<()> {
        synth(app, &global_config).await?;
        Ok(())
    }
}

pub async fn synth(app: &AppL2, global_config: &Arc<GlobalConfig>) -> Result<()> {
    Ok(())
}
