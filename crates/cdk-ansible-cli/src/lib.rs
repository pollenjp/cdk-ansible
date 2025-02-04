use cdk_ansible_static::EnvVars;
use clap::{command, Args, Parser, Subcommand};
use std::path::PathBuf;

pub mod version;

#[derive(Parser)]
#[command(name = "cdk-ansible", author, long_version = crate::version::version().to_string())]
#[command(about = ".")]
#[command(propagate_version = true)]
#[command(
    after_help = "Use `cdk-ansible help` for more details.",
    after_long_help = "",
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Box<Commands>,
    #[command(flatten)]
    pub top_level: TopLevelArgs,
}

#[derive(Parser)]
#[command(disable_help_flag = true, disable_version_flag = true)]
pub struct TopLevelArgs {
    #[command(flatten)]
    pub global_args: Box<GlobalArgs>,

    #[arg(
        global = true,
        long,
        env = EnvVars::CDK_ANSIBLE_CONFIG_FILE,
        help_heading = "Global options"
    )]
    pub config_file: Option<PathBuf>,

    /// Display the version.
    #[arg(global = true, short = 'V', long, action = clap::ArgAction::Version, help_heading = "Global options")]
    version: Option<bool>,
}

#[derive(Parser, Debug, Clone)]
#[command(next_help_heading = "Global options", next_display_order = 1000)]
#[allow(clippy::struct_excessive_bools)]
pub struct GlobalArgs {
    /// Do not print any output.
    #[arg(global = true, long, short, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Use verbose output.
    ///
    /// You can configure fine-grained logging using the `RUST_LOG` environment variable.
    /// (<https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives>)
    #[arg(global = true, action = clap::ArgAction::Count, long, short, conflicts_with = "quiet")]
    pub verbose: u8,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    #[command(next_help_heading = "Show help")]
    Help(HelpArgs),
    #[command(next_help_heading = "Create Rust code from ansible module")]
    Module(ModuleArgs),
    #[command(next_help_heading = "Create Ansible playbooks from Rust code")]
    Synth(SynthArgs),
}

#[derive(Args, Debug)]
pub struct HelpArgs {
    /// Disable pager when printing help
    #[arg(long)]
    pub no_pager: bool,

    pub command: Option<Vec<String>>,
}

#[derive(Args, Debug, Clone)]
pub struct ModuleArgs {
    #[arg(short, long, required = false)]
    pub output_dir: Option<PathBuf>,
    #[arg(short, long, required = false)]
    pub no_cache: bool,
    #[arg(short, long, required = false)]
    pub cache_dir: Option<PathBuf>,
    #[arg(
        short,
        long,
        required = false,
        help = "Specify the ansible module name. If not specified, all modules will be generated."
    )]
    pub module_name: Option<String>,
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
