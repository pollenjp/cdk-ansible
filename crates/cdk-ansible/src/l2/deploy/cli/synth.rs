use crate::l2::deploy::{AppL2, cli::GlobalConfig};
use anyhow::Result;
use clap::Args;
use std::sync::Arc;

#[derive(Args, Debug, Clone)]
pub struct Synth {}

impl Synth {
    pub async fn run(self, app: &AppL2, global_config: Arc<GlobalConfig>) -> Result<()> {
        synth(app, &global_config).await?;
        Ok(())
    }
}

pub async fn synth(_app: &AppL2, _global_config: &Arc<GlobalConfig>) -> Result<()> {
    Ok(())
}
