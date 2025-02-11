use clap::Parser;
mod version;
use anyhow::Context;
use anyhow::Result;
use clap::error::{ContextKind, ContextValue};
use serde::Deserialize;
use std::ffi::OsString;
use std::path::Path;

mod arg;
mod settings;
mod subcommand;
use arg::{Cli, Commands};

/// Options for the application.
#[derive(Debug, Clone, Default, Deserialize)]
struct Options {}

/// Load [`Options`] from a app config file
fn read_file(path: &Path) -> Result<Options> {
    let content =
        fs_err::read_to_string(path).context(format!("Failed to read file: {}", path.display()))?;
    println!("parsing function is not implemented yet:");
    println!("----------------------------------------");
    println!("{}", content);
    println!("----------------------------------------");
    let options: Options = Options::default();
    Ok(options)
}

#[derive(Debug, Clone)]
struct FilesystemOptions(Options);

impl FilesystemOptions {
    pub fn from_file(file: &Path) -> Result<Self> {
        let options = read_file(file)?;
        Ok(Self(options))
    }
}

pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(mut err) => {
            if let Some(ContextValue::String(subcommand)) = err.get(ContextKind::InvalidSubcommand)
            {
                match subcommand.as_str() {
                    "help" => {
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("module".to_string()),
                        );
                    }
                    "module" => {
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("module".to_string()),
                        );
                    }
                    "synth" => {
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("synth".to_string()),
                        );
                    }
                    _ => {}
                }
            }
            err.exit()
        }
    };

    let filesystem = if let Some(config_file) = cli.top_level.config_file.as_ref() {
        Some(FilesystemOptions::from_file(config_file)?)
    } else {
        None
    };

    if let Some(filesystem) = filesystem {
        // TODO: read as global settings
        println!("----------------------------------------");
        println!("Filesystem options: {:?}", filesystem);
        println!("----------------------------------------");
    }

    let global_settings = settings::GlobalSettings::resolve(*cli.top_level.global_args);

    println!("----------------------------------------");
    println!("Global settings: {:?}", global_settings);
    println!("{}", global_settings.quiet);
    println!("{}", global_settings.verbose);
    println!("----------------------------------------");

    let result = match *cli.command {
        Commands::Help(help_args) => {
            println!("----------------------------------------");
            println!("Help command: {:?}", help_args);
            println!("----------------------------------------");
            Ok(())
        }
        Commands::Module(module_args) => subcommand::module::module(module_args),
    };
    result.context("Failed to run command")
}
