use std::process::ExitCode;

mod builtin;
mod interpreter;
mod prompt;
mod shell;

use crate::prompt::Prompt;
use crate::shell::Shell;

fn main() -> ExitCode {
    let mut shell = Shell::new();

    if Prompt::new(&mut shell).interactive_loop().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
