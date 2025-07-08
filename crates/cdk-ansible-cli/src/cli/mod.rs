//! Subcommands

use crate::version;
use anyhow::Result;
use clap::{Parser, Subcommand, command};

mod init;
mod module;

#[derive(Parser)]
#[command(name = "cdk-ansible", author, long_version = version::version().to_string())]
#[command(about = ".")]
#[command(propagate_version = true)]
#[command(
    after_help = "Use `cdk-ansible help` for more details.",
    after_long_help = "",
    // disable_help_flag = true,
    // disable_help_subcommand = true,
    disable_version_flag = true
)]
/// The main CLI structure.
pub struct Cli {
    #[command(subcommand)]
    /// The command to execute.
    pub command: Box<Commands>,
    #[command(flatten)]
    /// The top-level arguments.
    pub top_level: TopLevelArgs,
}

impl Cli {
    pub async fn run(args: Vec<String>) -> Result<()> {
        let cli = Self::parse_from(args);

        match *cli.command {
            Commands::Init(cmd) => cmd.run().await,
            Commands::Module(cmd) => cmd.run().await,
        }
    }
}

#[derive(Parser)]
#[command(disable_version_flag = true)]
pub struct TopLevelArgs {
    /// Display the version.
    #[arg(global = true, short = 'V', long, action = clap::ArgAction::Version)]
    /// Show version information.
    pub version: Option<bool>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a cdk-ansible project
    #[command(verbatim_doc_comment)]
    Init(init::InitCmd),
    /// Create Rust code from ansible module
    ///
    /// Examples
    ///
    /// cdk-ansible module --module-name ansible.builtin.debug
    /// cdk-ansible module --module-name-regex 'ansible.builtin\..*'
    ///
    ///
    #[command(verbatim_doc_comment)]
    Module(module::ModuleCmd),
}
