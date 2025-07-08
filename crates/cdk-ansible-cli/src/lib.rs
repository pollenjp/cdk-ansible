//!
//! This is a crate for `cdk-ansible` command
//!
//! # Usage
//!
//! ```bash
//! cdk-ansible module --help
//! ```
//!

use anyhow::Result;
use std::num::NonZero;

/// Define commands
mod cli;
use cli::Cli;
mod utils;
mod version;

#[inline]
pub fn run() -> Result<()> {
    let nprocs = std::thread::available_parallelism()
        .map(NonZero::get)
        .unwrap_or_default();
    let threads = nprocs; // TODO: use env var
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(threads)
        .build()?
        .block_on(Cli::run(std::env::args().collect()))
}
