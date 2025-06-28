use crate::deploy::{DeployApp, cli::GlobalArgs};
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Deploy {}

impl Deploy {
    pub async fn run(self, app: &DeployApp, global_args: GlobalArgs) -> Result<()> {
        dbg!(&app);
        dbg!(&global_args);
        Ok(())
    }
}
