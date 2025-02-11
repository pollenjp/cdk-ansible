use clap::{command, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "custom cdk-ansible")]
#[command(about = ".")]
#[command(propagate_version = true)]
#[command(
    after_help = "Use `cdk-ansible help` for more details.",
    after_long_help = "",
    // disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Box<Commands>,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    #[command(next_help_heading = "Create Ansible playbooks from Rust code")]
    Synth(SynthArgs),
}

#[derive(Args, Debug, Clone)]
pub struct SynthArgs {
    #[arg(short, long, required = true)]
    #[arg(help = "Ansible project root directory")]
    pub output_dir: PathBuf,
    #[arg(short, long, required = false)]
    #[arg(help = "Ansible inventory directory. Default is '<output>/inventory/'")]
    pub inventory_dir: Option<PathBuf>,
    #[arg(short, long, required = false)]
    #[arg(help = "Ansible playbooks directory. Default is '<output>/playbooks/'")]
    pub playbooks_dir: Option<PathBuf>,
}
