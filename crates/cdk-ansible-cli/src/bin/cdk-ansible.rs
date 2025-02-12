use anyhow::Result;
use cdk_ansible_cli::run;
use std::env;

fn main() -> Result<()> {
    run(env::args_os())?;
    Ok(())
}
