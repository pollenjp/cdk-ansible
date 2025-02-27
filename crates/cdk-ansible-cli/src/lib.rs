//!
//! This is a crate for `cdk-ansible` command
//!
//! # Usage
//!
//! ```bash
//! cdk-ansible module --help
//! ```
//!

use anyhow::{Context as _, Result};
use clap::{
    Parser as _,
    error::{ContextKind, ContextValue},
};
use serde::Deserialize;
use std::ffi::OsString;
use std::path::Path;

/// CLI arguments
mod arg;
/// Settings layer
mod settings;
/// Define subcommands
mod subcommand;
/// Define version
mod version;

use arg::{Cli, Commands};

#[derive(Debug, Clone, Default, Deserialize)]
/// Options for the application.
///
/// Not implemented yet.
struct Options {
    #[expect(dead_code, reason = "not implemented yet")]
    /// Phantom data
    val1: String,
}

/// Load [`Options`] from a app config file
#[expect(clippy::single_call_fn, reason = "better readability")]
fn read_file(path: &Path) -> Result<Options> {
    let _content =
        fs_err::read_to_string(path).context(format!("Failed to read file: {}", path.display()))?;
    let options: Options = Options::default();
    Ok(options)
}

#[derive(Debug, Clone)]
/// Filesystem options
/// Not implemented yet.
#[expect(dead_code, reason = "not implemented yet")]
struct FilesystemOptions(Options);

impl FilesystemOptions {
    /// Load [`FilesystemOptions`] from a app config file
    #[expect(clippy::single_call_fn, reason = "better readability")]
    pub fn from_file(file: &Path) -> Result<Self> {
        let options: Options = read_file(file)?;
        Ok(Self(options))
    }
}

/// Main entry point of the application
///
/// This function is the main entry point of the application.
/// It parses the command line arguments, reads the configuration file,
/// and then executes the appropriate subcommand.
///
/// # Arguments
///
/// * `args` - The command line arguments to parse.
///
/// # Returns
///
/// Returns a `Result` with the result of the subcommand.
///
/// # Errors
///
/// Use [`anyhow::Result`] to handle errors.
///
#[inline]
pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(mut err) => {
            #[expect(clippy::single_match, reason = "better readability")]
            #[expect(clippy::pattern_type_mismatch, reason = "I don't know")]
            match err.get(ContextKind::InvalidSubcommand) {
                Some(ContextValue::String(subcommand)) => match subcommand.as_str() {
                    "help" => {
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("help".to_owned()),
                        );
                    }
                    "module" => {
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("module".to_owned()),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
            err.exit()
        }
    };

    let filesystem = if let Some(config_file) = cli.top_level.config_file.as_ref() {
        Some(FilesystemOptions::from_file(config_file)?)
    } else {
        None
    };

    #[expect(clippy::todo, reason = "implement later")]
    if let Some(_filesystem) = filesystem {
        todo!("TODO: read as global settings");
    }

    // TODO: implement later
    let _global_settings = settings::GlobalSettings::resolve(&cli.top_level.global_args);

    let result = match *cli.command {
        #[expect(clippy::todo, reason = "implement later")]
        Commands::Help(_help_args) => {
            todo!("TODO: implement help command");
        }
        Commands::Module(module_args) => subcommand::module::module(module_args),
    };
    result.context("Failed to run command")
}
