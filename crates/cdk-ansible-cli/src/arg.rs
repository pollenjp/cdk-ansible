use cdk_ansible_static::EnvVars;
use clap::{command, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cdk-ansible", author, long_version = crate::version::version().to_string())]
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
    #[command(flatten)]
    pub top_level: TopLevelArgs,
}

#[derive(Parser)]
#[command(disable_version_flag = true)]
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
    /// Default value is defined at `cdk_ansible::settings::ModuleSettings`
    #[arg(short, long, required = false)]
    pub pkg_prefix: Option<String>,
    /// Specifies the level at which Cargo packages are created:
    /// - 'namespace': Creates a package at the namespace level
    /// - 'collection': Creates a package at the collection level
    /// - 'module': Creates a package at the module level
    /// - 'None': Does not create any packages
    ///
    /// Package names follow this pattern:
    /// - namespace: 'cdkam_<namespace>'
    /// - collection: 'cdkam_<namespace>_<collection>'
    /// - module: 'cdkam_<namespace>_<collection>_<module>'
    #[arg(long, required = false, value_enum)]
    pub pkg_unit: Option<PkgUnit>,
    /// Default value is defined at `cdk_ansible::settings::ModuleSettings`
    #[arg(long, required = false)]
    pub output_dir: Option<PathBuf>,
    /// Default value is defined at `cdk_ansible::settings::ModuleSettings`
    #[arg(long, required = false)]
    pub no_cache: bool,
    /// Default value is defined at `cdk_ansible::settings::ModuleSettings`
    #[arg(long, required = false)]
    pub cache_dir: Option<PathBuf>,
    /// Specify the ansible module name. (e.g. 'ansible.builtin.debug')
    /// If not specified, all modules accessible from your ansible environment will be generated.
    #[arg(long, required = false, conflicts_with = "module_name_regex")]
    pub module_name: Option<String>,
    /// Specify the ansible module name regex. (e.g. 'ansible\.builtin\..*')
    /// If not specified, all modules accessible from your ansible environment will be generated.
    #[arg(long, required = false, conflicts_with = "module_name")]
    pub module_name_regex: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
pub enum PkgUnit {
    Namespace,
    Collection,
    Module,
    None,
}
