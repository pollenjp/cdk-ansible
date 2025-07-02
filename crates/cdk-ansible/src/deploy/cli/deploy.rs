use crate::{
    ExPlaybook,
    deploy::{
        DeployApp,
        cli::{GlobalArgs, GlobalConfig, synth::synth},
    },
};
use anyhow::{Context as _, Result};
use clap::Args;
use futures::future::{BoxFuture, FutureExt as _};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tokio::task::JoinSet;

#[derive(Args, Debug, Clone)]
pub struct Deploy {
    /// The command to run the playbook. This string is parsed by shlex.
    ///
    /// The first argument is the command name, and the rest are arguments.
    ///
    /// The default is `ansible-playbook`.
    ///
    /// Example: If you want to run `uv run ansible-playbook -v some_playbook`, pass `uv run ansible-playbook -v` to [`Deploy::playbook_command`].
    #[arg(short, long, required = false, default_value = "ansible-playbook")]
    pub playbook_command: String,
}

impl Deploy {
    pub async fn run(self, app: &DeployApp, global_config: Arc<GlobalConfig>) -> Result<()> {
        let deploy_config = Arc::new(DeployConfig::new(self)?);
        synth(app, &global_config).await?;
        deploy(app, &global_config, &deploy_config).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DeployConfig {
    playbook_command: Vec<String>,
}

impl DeployConfig {
    pub fn new(args: Deploy) -> Result<Self> {
        Ok(Self {
            playbook_command: ::shlex::split(&args.playbook_command)
                .with_context(|| "parsing playbook command")?,
        })
    }
}

async fn deploy(
    app: &DeployApp,
    global_config: &Arc<GlobalConfig>,
    deploy_config: &Arc<DeployConfig>,
) -> Result<()> {
    let playbook_dir = Arc::new(global_config.playbook_dir.clone());
    // FIXME: use stack name from args
    for (_, ex_playbook) in app.ex_playbooks.iter() {
        recursive_deploy(
            ex_playbook.clone(),
            Arc::clone(&playbook_dir),
            Arc::clone(deploy_config),
        )
        .await?;
    }
    Ok(())
}

fn recursive_deploy(
    ex_playbook: ExPlaybook,
    playbook_dir: Arc<PathBuf>,
    deploy_config: Arc<DeployConfig>,
) -> BoxFuture<'static, Result<()>> {
    async move {
        match ex_playbook {
            ExPlaybook::Single(pb) => {
                // Run 'ansible-playbook' command

                let pb_path = playbook_dir.join(pb.name.clone()).with_extension("yaml");
                let cmd = deploy_config
                    .playbook_command
                    .first()
                    .with_context(|| "getting 1st playbook command")?;
                let output = Command::new(cmd)
                    .args(deploy_config.playbook_command.get(1..).unwrap_or_default())
                    .args([
                        "-i",
                        "localhost",
                        pb_path.to_str().with_context(|| "stringifying path")?,
                    ])
                    .output()
                    .await?;
                if !output.status.success() {
                    anyhow::bail!(
                        "running ansible-playbook: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            ExPlaybook::Sequential(pbs) => {
                for pb in pbs {
                    recursive_deploy(pb, Arc::clone(&playbook_dir), Arc::clone(&deploy_config))
                        .await?;
                }
            }
            ExPlaybook::Parallel(pbs) => {
                let mut set: JoinSet<Result<()>> = JoinSet::new();
                for pb in pbs {
                    set.spawn(recursive_deploy(
                        pb,
                        Arc::clone(&playbook_dir),
                        Arc::clone(&deploy_config),
                    ));
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
