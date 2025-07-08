#![allow(clippy::all, reason = "implement later")]
#![allow(clippy::pedantic, reason = "implement later")]
#![allow(clippy::restriction, reason = "implement later")]
#![allow(dead_code, reason = "implement later")]
#![allow(unreachable_code, reason = "implement later")]

use anyhow::{Context as _, Result, bail};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct InitCmd {
    /// The directory to initialize the project.
    /// If not specified, the current directory will be used.
    /// Error if the directory is not empty.
    /// Only 'mise.toml' file will be ignored.
    #[arg(short = 'd', long, required = false)]
    pub dir: Option<PathBuf>,
}

impl InitCmd {
    pub async fn run(self) -> Result<()> {
        let config = InitConfig::new(self)?;
        dbg!(&config);
        todo!("implement later");
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct InitConfig {
    dir: PathBuf,
}

impl InitConfig {
    fn new(args: InitCmd) -> Result<Self> {
        let dir = args.dir.map_or_else(
            || std::env::current_dir().context("Failed to get current directory"),
            Ok,
        )?;
        if dir.is_file() {
            bail!("The directory is a file: {}", dir.display());
        }
        if dir.is_dir() {
            bail!("The directory is not empty: {}", dir.display());
        }
        Ok(Self { dir })
    }
}
