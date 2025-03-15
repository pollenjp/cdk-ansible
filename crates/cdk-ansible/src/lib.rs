use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use clap::error::{ContextKind, ContextValue};
use serde::Deserialize;
use std::ffi::OsString;
pub mod arg;
use arg::{Cli, Commands};
pub mod settings;
mod subcommand;
pub use cdk_ansible_core::core::*;
pub use subcommand::synth::Synthesizer;

// Re-export macros
pub use cdk_ansible_macro::FieldCount;

/// Options for the application.
#[derive(Debug, Clone, Default, Deserialize)]
struct Options {}

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

    let result = match *cli.command {
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
