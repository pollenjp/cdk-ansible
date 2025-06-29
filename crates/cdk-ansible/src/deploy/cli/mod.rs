use crate::deploy::DeployApp;
use anyhow::Result;
use clap::{Args, Parser, Subcommand, command};
use std::path::PathBuf;

mod deploy;
mod synth;

#[derive(Parser)]
#[command(name = "Personalized cdk-ansible command")]
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
    pub async fn run(app: &DeployApp) -> Result<()> {
        let cli = Cli::parse_from(app.args.clone());
        if let Some(command) = cli.command {
            match *command {
                Commands::Synth(cmd) => {
                    cmd.run(app, &cli.global_args).await?;
                }
                Commands::Deploy(cmd) => {
                    cmd.run(app, &cli.global_args).await?;
                }
            }
        } else {
            dbg!("no command");
        }
        Ok(())
    }
}
