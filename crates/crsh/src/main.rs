use std::process::ExitCode;

use crsh_core::Shell;

fn main() -> ExitCode {
    Shell::new().main()
}
