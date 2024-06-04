use std::process::ExitCode;

mod builtin;
mod interpreter;
mod prompt;
mod system;

use crate::prompt::Prompt;

fn main() -> ExitCode {
    if Prompt::new().interactive_loop().is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
