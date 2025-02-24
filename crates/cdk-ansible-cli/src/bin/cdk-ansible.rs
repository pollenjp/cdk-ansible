use anyhow::Result;
use cdk_ansible_cli::run;

fn main() -> Result<()> {
    run(std::env::args_os())?;
    Ok(())
}
