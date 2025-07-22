use crate::l2::deploy::AppL2;
use anyhow::{Context as _, Result};
use clap::{Args, Parser, Subcommand, command};
use std::path::{PathBuf, absolute};
use std::sync::Arc;

mod deploy;
mod synth;

#[derive(Parser)]
#[command(name = "cargo run --package your-app --")]
#[command(about = "about")]
#[command(
    // after_help = "Use `help` subcommand for more details.",
    // after_long_help = "",
    // disable_help_flag = true,
    // disable_help_subcommand = true,
    // disable_version_flag = true
)]
pub struct Cli {
    #[command(flatten)]
    pub global_args: GlobalArgs,

    #[command(subcommand)]
    pub command: Option<Box<Commands>>,
}

#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    #[arg(short, long, required = false, default_value = ".cdka")]
    #[arg(help = "A directory saving generated files. Default is '.cdka' in current directory.")]
    pub app_dir: PathBuf,
    #[arg(short, required = false)]
    pub uv_project: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    // pub app_dir: PathBuf,
    pub playbook_dir: PathBuf,
    pub inventory_dir: PathBuf,
}

impl GlobalConfig {
    pub fn from_args(args: &GlobalArgs) -> Result<Self> {
        let app_dir = absolute(&args.app_dir).with_context(|| "absolute path of app_dir")?;
        let playbook_dir = app_dir.join("playbooks");
        let inventory_dir = app_dir.join("inventory");
        Ok(Self {
            // app_dir,
            playbook_dir,
            inventory_dir,
        })
    }
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    #[command(next_help_heading = "Create Ansible playbooks from Rust code")]
    Synth(synth::Synth),
    #[command(next_help_heading = "Deploy Ansible playbooks")]
    Deploy(deploy::Deploy),
}

impl Cli {
    pub async fn run(app: &AppL2) -> Result<()> {
        let cli = Cli::parse_from(app.inner.args.clone());
        let global_config = Arc::new(GlobalConfig::from_args(&cli.global_args)?);
        if let Some(command) = cli.command {
            match *command {
                Commands::Synth(cmd) => {
                    cmd.run(app, Arc::clone(&global_config)).await?;
                }
                Commands::Deploy(cmd) => {
                    cmd.run(app, Arc::clone(&global_config)).await?;
                }
            }
        } else {
            dbg!("no command");
        }
        Ok(())
    }
}
