use std::process::ExitCode;

mod prompt;
mod shell;

fn main() -> ExitCode {
    crate::shell::Shell::new().main()
}
