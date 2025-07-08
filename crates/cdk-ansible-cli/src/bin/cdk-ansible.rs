#![expect(clippy::print_stderr, reason = "In main function")]
#![expect(clippy::use_debug, reason = "In main function")]

use cdk_ansible_cli::run;

/// Main entry point for end users
pub fn main() -> std::result::Result<(), i32> {
    if let Err(e) = run() {
        eprintln!("Error: {e:?}");
        return Err(1);
    }
    Ok(())
}
