use anyhow::Context;
use anyhow::Result;
use cdk_ansible_cli::{Cli, Commands};
use cdk_ansible_core::core::{Inventory, Playbook};
use clap::error::{ContextKind, ContextValue};
use clap::Parser;
use serde::Deserialize;
use std::ffi::OsString;
use std::path::Path;

pub mod settings;

mod subcommand;

pub use subcommand::synth::Synthesizer;

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

pub fn run<I, T>(args: I, commander: impl Synthesizer) -> Result<()>
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
                            ContextValue::String("help".to_string()),
                        );
                    }
                    "some1" => {
                        // TODO: remove later
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("some1".to_string()),
                        );
                    }
                    "some2" => {
                        // TODO: remove later
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("some2".to_string()),
                        );
                    }
                    "some3" => {
                        // TODO: remove later
                        err.insert(
                            ContextKind::InvalidSubcommand,
                            ContextValue::String("some3".to_string()),
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
    println!("----------------------------------------");

    let result = match *cli.command {
        Commands::Help(help_args) => {
            println!("----------------------------------------");
            println!("Help command: {:?}", help_args);
            println!("----------------------------------------");
            Ok(())
        }
        Commands::Module(module_args) => subcommand::module::module(module_args),
        Commands::Synth(synth_args) => subcommand::synth::synth(&commander, synth_args),
    };
    result.context("Failed to run command")
}

pub fn playbook_dump(args: &settings::SynthSettings, playbook: &Playbook) -> Result<()> {
    let file_path = args.playbook_dir.join(format!("{}.json", playbook.name));
    std::fs::create_dir_all(file_path.parent().unwrap())?;
    std::fs::write(file_path, serde_json::to_string_pretty(&playbook.plays)?)?;
    Ok(())
}
