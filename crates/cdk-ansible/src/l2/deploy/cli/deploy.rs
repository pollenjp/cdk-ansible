use crate::{
    ExePlaybook,
    l2::deploy::{
        AppL2,
        cli::{GlobalConfig, synth::synth},
    },
    types::StackName,
};
use anyhow::{Context as _, Result};
use clap::Args;
use futures::future::{BoxFuture, FutureExt as _};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Semaphore;
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
    #[arg(
        short = 'c',
        long,
        required = false,
        default_value = "ansible-playbook"
    )]
    pub playbook_command: String,
    /// Inventory name.
    ///
    /// The candidates are inventories added to the [`crate::l2::deploy::App`] ([`crate::l2::deploy::App::add_inventory`])
    #[arg(short = 'i', long, required = true)]
    pub inventory: String,
    /// The maximum number of playbook processes.
    #[arg(short = 'P', long, required = false, default_value = "2")]
    pub max_procs: usize,
    // The stack name to deploy.
    #[arg(required = true)]
    pub stack_name: String,
}

impl Deploy {
    pub async fn run(self, app: &AppL2, global_config: Arc<GlobalConfig>) -> Result<()> {
        let deploy_config = Arc::new(DeployConfig::new(self)?);
        synth(app, &global_config).await?;

        deploy(app, &global_config, &deploy_config).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DeployConfig {
    playbook_command: Vec<String>,
    inventory: String,
    max_procs: usize,
    stack_name: String,
}

impl DeployConfig {
    pub fn new(args: Deploy) -> Result<Self> {
        Ok(Self {
            playbook_command: ::shlex::split(&args.playbook_command)
                .with_context(|| "parsing playbook command")?,
            inventory: args.inventory,
            max_procs: args.max_procs,
            stack_name: args.stack_name,
        })
    }
}

async fn deploy(
    app: &AppL2,
    global_config: &Arc<GlobalConfig>,
    deploy_config: &Arc<DeployConfig>,
) -> Result<()> {
    let playbook_dir = Arc::new(global_config.playbook_dir.clone());
    let inventory_dir = Arc::new(global_config.inventory_dir.clone());

    // Semaphore for limiting the number of concurrent ansible-playbook processes
    let pb_semaphore = Arc::new(Semaphore::new(deploy_config.max_procs));

    let exe_playbook = app
        .inner
        .stack_container
        .exe_playbooks()
        .get(&StackName::from(deploy_config.stack_name.as_str()))
        .with_context(|| "getting exe_playbook")?;
    recursive_deploy(
        exe_playbook.clone(),
        Arc::clone(&playbook_dir),
        Arc::clone(&inventory_dir),
        Arc::clone(deploy_config),
        Arc::clone(&pb_semaphore),
    )
    .await?;
    Ok(())
}

fn recursive_deploy(
    exe_playbook: ExePlaybook,
    playbook_dir: Arc<PathBuf>,
    inventory_dir: Arc<PathBuf>,
    deploy_config: Arc<DeployConfig>,
    pb_semaphore: Arc<Semaphore>,
) -> BoxFuture<'static, Result<()>> {
    async move {
        match exe_playbook {
            ExePlaybook::Single(pb) => {
                // Run 'ansible-playbook' command

                let pb_path = playbook_dir.join(pb.name.clone()).with_extension("yaml");
                if !pb_path.exists() {
                    anyhow::bail!("playbook file not found: {}", pb_path.display());
                }

                let inventory_path = inventory_dir
                    .join(deploy_config.inventory.clone())
                    .with_extension("yaml");
                if !inventory_path.exists() {
                    anyhow::bail!("inventory file not found: {}", inventory_path.display());
                }

                let cmd = deploy_config
                    .playbook_command
                    .first()
                    .with_context(|| "getting 1st playbook command")?;

                let _permit = pb_semaphore.clone().acquire_owned().await?;
                let output = Command::new(cmd)
                    .args(deploy_config.playbook_command.get(1..).unwrap_or_default())
                    .args([
                        "-i",
                        inventory_path
                            .to_str()
                            .with_context(|| "stringifying path")?,
                        pb_path.to_str().with_context(|| "stringifying path")?,
                    ])
                    .output()
                    .await?;
                if !output.status.success() {
                    anyhow::bail!(
                        "running ansible-playbook:\n{}\n{}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            ExePlaybook::Sequential(pbs) => {
                for pb in pbs {
                    recursive_deploy(
                        pb,
                        Arc::clone(&playbook_dir),
                        Arc::clone(&inventory_dir),
                        Arc::clone(&deploy_config),
                        Arc::clone(&pb_semaphore),
                    )
                    .await?;
                }
            }
            ExePlaybook::Parallel(pbs) => {
                let mut set: JoinSet<Result<()>> = JoinSet::new();
                for pb in pbs {
                    set.spawn(recursive_deploy(
                        pb,
                        Arc::clone(&playbook_dir),
                        Arc::clone(&inventory_dir),
                        Arc::clone(&deploy_config),
                        Arc::clone(&pb_semaphore),
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
