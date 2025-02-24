use crate::version;
use cdk_ansible_static::EnvVars;
use clap::{command, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cdk-ansible", author, long_version = version::version().to_string())]
#[command(about = ".")]
#[command(propagate_version = true)]
#[command(
    after_help = "Use `cdk-ansible help` for more details.",
    after_long_help = "",
    // disable_help_flag = true,
    disable_help_subcommand = true,
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

#[derive(Parser)]
#[command(disable_version_flag = true)]
pub struct TopLevelArgs {
    #[arg(
        global = true,
        long,
        env = EnvVars::CDK_ANSIBLE_CONFIG_FILE,
    )]
    /// The configuration file (future use)
    pub config_file: Option<PathBuf>,
    #[command(flatten)]
    /// The global arguments.
    pub global_args: Box<GlobalArgs>,
    /// Display the version.
    #[arg(global = true, short = 'V', long, action = clap::ArgAction::Version)]
    /// Show version information.
    pub version: Option<bool>,
}

#[derive(Parser, Debug, Clone)]
#[command(next_help_heading = "Global options", next_display_order = 1000)]
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
pub enum Commands {
    /// Show help
    Help(HelpArgs),
    /// Create Rust code from ansible module
    ///
    /// Examples
    ///
    /// cdk-ansible module --module-name ansible.builtin.debug
    /// cdk-ansible module --module-name-regex 'ansible.builtin\..*'
    ///
    ///
    #[command(verbatim_doc_comment)]
    Module(ModuleArgs),
}

#[derive(Args, Debug)]
pub struct HelpArgs {
    /// Disable pager when printing help
    #[arg(long)]
    pub no_pager: bool,

    /// The command to show help for
    pub command: Option<Vec<String>>,
}

#[derive(Args, Debug, Clone)]
pub struct ModuleArgs {
    /// Prefix of the generated package name.
    ///
    /// The default value `cdkam` means CDK Ansible Module.
    #[arg(
        short,
        long,
        required = false,
        verbatim_doc_comment,
        default_value = "cdkam"
    )]
    pub pkg_prefix: String,
    /// Specifies the level at which Cargo packages are created.
    #[arg(
        long,
        required = false,
        value_enum,
        verbatim_doc_comment,
        default_value = "namespace"
    )]
    pub pkg_unit: PkgUnit,
    /// Directory to store generated Rust code.
    #[arg(
        long,
        required = false,
        verbatim_doc_comment,
        default_value = ".cdk-ansible.out"
    )]
    pub output_dir: PathBuf,
    /// Do not use cache of ansible-doc output.
    #[arg(long, required = false, verbatim_doc_comment)]
    pub no_cache: bool,
    /// Directory to store cache files of ansible-doc output.
    #[arg(
        long,
        required = false,
        verbatim_doc_comment,
        default_value = ".cdk-ansible.cache.out"
    )]
    pub cache_dir: PathBuf,
    /// Specify the ansible module name. (e.g. `ansible.builtin.debug`)
    /// If not specified, all modules accessible from your ansible environment will be generated.
    #[arg(
        long,
        required = false,
        verbatim_doc_comment,
        conflicts_with = "module_name_regex"
    )]
    pub module_name: Option<String>,
    /// Specify the ansible module name regex. (e.g. 'ansible\.builtin\..*')
    /// If not specified, all modules accessible from your ansible environment will be generated.
    #[arg(
        long,
        required = false,
        verbatim_doc_comment,
        conflicts_with = "module_name"
    )]
    pub module_name_regex: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Eq, PartialEq)]
pub enum PkgUnit {
    /// Create a package at the namespace level (package name will be 'cdkam_<namespace>')
    Namespace,
    /// Create a package at the collection level (package name will be 'cdkam_<namespace>_<collection>')
    Collection,
    /// Create a package at the module level (package name will be 'cdkam_<namespace>_<collection>_<module>')
    Module,
    /// Does not create package directory.
    /// Only create the module source code.
    None,
}
