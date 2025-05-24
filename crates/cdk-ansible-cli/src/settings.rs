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
    /// The regex for the module name to exclude
    pub module_name_exclude: Option<Vec<String>>,
}

impl ModuleSettings {
    /// Convert the command line arguments to the settings
    pub fn resolve(args: arg::ModuleArgs) -> Self {
        Self {
            output_dir: args.output_dir,
            pkg_prefix: args.pkg_prefix,
            pkg_unit: match args.pkg_unit {
                arg::PkgUnit::Namespace => Some(PkgUnit::Namespace),
                arg::PkgUnit::Collection => Some(PkgUnit::Collection),
                arg::PkgUnit::Module => Some(PkgUnit::Module),
                arg::PkgUnit::None => None,
            },
            use_cache: !args.no_cache,
            cache_dir: args.cache_dir,
            module_name: args.module_name,
            module_name_regex: args.module_name_regex,
            module_name_exclude: args.module_name_exclude,
        }
    }
}
