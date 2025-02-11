use crate::arg;
use std::path::PathBuf;

#[derive(Debug, Clone)]
/// CLI global settings
pub struct GlobalSettings {
    /// Whether to suppress output
    #[expect(dead_code, reason = "TODO: implement later")]
    pub quiet: bool,
    /// The verbosity level
    #[expect(dead_code, reason = "TODO: implement later")]
    pub verbose: u8,
}

impl GlobalSettings {
    /// Convert the command line arguments to the settings
    #[expect(clippy::single_call_fn, reason = "better readability")]
    pub const fn resolve(args: &arg::GlobalArgs) -> Self {
        Self {
            quiet: args.quiet,
            verbose: args.verbose,
        }
    }
}

#[derive(Debug, Clone)]
/// The unit of the package for settings
pub enum PkgUnit {
    /// The namespace of the package
    Namespace,
    /// The collection of the package
    Collection,
    /// The module of the package
    Module,
}

#[derive(Debug, Clone)]
/// Settings for the module command
pub struct ModuleSettings {
    /// The output directory
    pub output_dir: PathBuf,
    /// The prefix for the package name (default: "cdkam")
    pub pkg_prefix: String,
    /// The unit of the package
    pub pkg_unit: Option<PkgUnit>,
    /// Whether to use the cache
    pub use_cache: bool,
    /// The cache directory
    pub cache_dir: PathBuf,
    /// The name of the module
    pub module_name: Option<String>,
    /// The regex for the module name
    pub module_name_regex: Option<String>,
}

impl ModuleSettings {
    /// Convert the command line arguments to the settings
    #[expect(clippy::single_call_fn, reason = "better readability")]
    pub fn resolve(args: arg::ModuleArgs) -> Self {
        Self {
            output_dir: args.output_dir.unwrap_or_else(|| ".cdk-ansible.out".into()),
            pkg_prefix: args.pkg_prefix.unwrap_or_else(|| {
                // CDK Ansible Module
                "cdkam".to_owned()
            }),
            pkg_unit: match args.pkg_unit {
                Some(arg::PkgUnit::Namespace) | None => Some(PkgUnit::Namespace),
                Some(arg::PkgUnit::Collection) => Some(PkgUnit::Collection),
                Some(arg::PkgUnit::Module) => Some(PkgUnit::Module),
                Some(arg::PkgUnit::None) => None,
            },
            use_cache: !args.no_cache,
            cache_dir: args
                .cache_dir
                .unwrap_or_else(|| ".cdk-ansible.cache.out".into()),
            module_name: args.module_name,
            module_name_regex: args.module_name_regex,
        }
    }
}
