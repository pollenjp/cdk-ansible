use crate::{
    ExPlaybook,
    deploy::{
        DeployApp,
        cli::{GlobalArgs, synth::synth},
    },
};
use anyhow::Result;
use clap::Args;
use futures::future::{BoxFuture, FutureExt as _};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tokio::task::JoinSet;

#[derive(Args, Debug, Clone)]
pub struct Deploy {}

impl Deploy {
    pub async fn run(self, app: &DeployApp, global_args: &GlobalArgs) -> Result<()> {
        synth(app, global_args).await?;
        deploy(app, global_args).await?;
        Ok(())
    }
}

async fn deploy(app: &DeployApp, global_args: &GlobalArgs) -> Result<()> {
    let playbook_dir = global_args.app_dir.join("playbooks");
    let playbook_dir = Arc::new(playbook_dir);
    // FIXME: use stack name from args
    for (_, ex_playbook) in app.ex_playbooks.iter() {
        recursive_deploy(ex_playbook.clone(), Arc::clone(&playbook_dir)).await?;
    }
    Ok(())
}

fn recursive_deploy(
    ex_playbook: ExPlaybook,
    playbook_dir: Arc<PathBuf>,
) -> BoxFuture<'static, Result<()>> {
    async move {
        match ex_playbook {
            ExPlaybook::Single(_pb) => {
                // TODO: run 'ansible-playbook' command
                let output = Command::new("echo").arg("hello").arg("world").output();
                let output = output.await?;
                if !output.status.success() {
                    anyhow::bail!("command failed");
                }
                println!("{:?}", output.stdout);
            }
            ExPlaybook::Sequential(pbs) => {
                for pb in pbs {
                    recursive_deploy(pb, Arc::clone(&playbook_dir)).await?;
                }
            }
            ExPlaybook::Parallel(pbs) => {
                let mut set: JoinSet<Result<()>> = JoinSet::new();
                for pb in pbs {
                    set.spawn(recursive_deploy(pb, Arc::clone(&playbook_dir)));
                }
                while let Some(res) = set.join_next().await {
                    (res?)?;
                }
            }
        }
        Ok(())
    }
    .boxed()
}
