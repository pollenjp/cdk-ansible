use crate::{
    ExePlayL2,
    l2::deploy::{AppL2, cli::GlobalConfig},
    types::StackName,
    utils::{dump_json, json_to_yaml},
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
        deploy(app, &global_config, &deploy_config).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DeployConfig {
    playbook_command: Vec<String>,
    max_procs: usize,
    stack_name: StackName,
}

impl DeployConfig {
    pub fn new(args: Deploy) -> Result<Self> {
        Ok(Self {
            playbook_command: ::shlex::split(&args.playbook_command)
                .with_context(|| "parsing playbook command")?,
            max_procs: args.max_procs,
            stack_name: StackName::from(args.stack_name.as_str()),
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
    let cmd_semaphore = Arc::new(Semaphore::new(deploy_config.max_procs));

    let stack = app
        .inner
        .stack_container
        .get_stack(&deploy_config.stack_name)
        .with_context(|| format!("getting stack: {}", deploy_config.stack_name))?;

    recursive_deploy(
        deploy_config
            .stack_name
            .to_string()
            .to_lowercase()
            .replace(' ', "_"),
        stack.exe_play().clone(),
        Arc::clone(&playbook_dir),
        Arc::clone(&inventory_dir),
        Arc::clone(deploy_config),
        Arc::clone(&cmd_semaphore),
    )
    .await?;
    Ok(())
}

fn recursive_deploy(
    name: String,
    exe_play_l2: ExePlayL2,
    playbook_dir: Arc<PathBuf>,
    inventory_dir: Arc<PathBuf>,
    deploy_config: Arc<DeployConfig>,
    cmd_semaphore: Arc<Semaphore>,
) -> BoxFuture<'static, Result<()>> {
    async move {
        match exe_play_l2 {
            ExePlayL2::Single(ep) => {
                // Run 'ansible-playbook' command

                let play_l2 = ep.create_play_l2().await?;
                let name = format!("{name}_{}", &play_l2.name);
                let inv_root = play_l2.hosts.to_inventory_root()?;
                let play = play_l2.try_play()?;

                // Create playbook
                let pb_path_j = playbook_dir.join(&name).with_extension("json");
                dump_json(pb_path_j.clone(), vec![play]).await?;
                json_to_yaml(pb_path_j.clone()).await?;

                // Create inventory
                let inv_path_j = inventory_dir.join(&name).with_extension("json");
                dump_json(inv_path_j.clone(), inv_root).await?;
                json_to_yaml(inv_path_j.clone()).await?;

                let cmd = deploy_config
                    .playbook_command
                    .first()
                    .with_context(|| "getting 1st playbook command")?;

                let _permit = cmd_semaphore.clone().acquire_owned().await?;
                let output = Command::new(cmd)
                    .args(deploy_config.playbook_command.get(1..).unwrap_or_default())
                    .args([
                        "-i",
                        inv_path_j
                            .with_extension("yaml")
                            .to_str()
                            .with_context(|| "stringifying path")?,
                        pb_path_j
                            .with_extension("yaml")
                            .to_str()
                            .with_context(|| "stringifying path")?,
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
                println!(
                    "{}\n{}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr),
                );
            }
            ExePlayL2::Sequential(eps) => {
                for (i, ep) in eps.into_iter().enumerate() {
                    recursive_deploy(
                        format!("{name}_seq{i}"),
                        ep,
                        Arc::clone(&playbook_dir),
                        Arc::clone(&inventory_dir),
                        Arc::clone(&deploy_config),
                        Arc::clone(&cmd_semaphore),
                    )
                    .await?;
                }
            }
            ExePlayL2::Parallel(eps) => {
                let mut set: JoinSet<Result<()>> = JoinSet::new();
                for (i, ep) in eps.into_iter().enumerate() {
                    set.spawn(recursive_deploy(
                        format!("{name}_par{i}"),
                        ep,
                        Arc::clone(&playbook_dir),
                        Arc::clone(&inventory_dir),
                        Arc::clone(&deploy_config),
                        Arc::clone(&cmd_semaphore),
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
