use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod builtin;
mod common_env;
mod config;
mod execution;
mod parsing;
mod shell_io;

pub use common_env::*;
pub use config::*;
pub use shell_io::*;

pub use crate::prompt::Prompt;

#[derive(Clone, PartialEq)]
pub enum ShellMode {
    Interactive,
    Read,
    Command(String),
    Script(String),
}

#[derive(Default)]
pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: i32,
    pub io: IOContext,
    pub config: ShellConfig,
}

impl Shell {
    pub fn main(&mut self, mode: ShellMode) -> ExitCode {
        match mode {
            ShellMode::Interactive => {
                if Prompt::new(self).interactive_loop() == 0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            ShellMode::Read => {
                let mut input = String::new();

                if io::stdin().read_to_string(&mut input).is_ok() && self.interpret(&input) == 0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            ShellMode::Command(input) => {
                if self.interpret(&input) == 0 {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            ShellMode::Script(path) => match fs::read_to_string(&path) {
                Ok(script) => {
                    if self.interpret(&script) == 0 {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(e) => {
                    eprintln!("crsh: failed to run script at \"{path}\": {e}");
                    ExitCode::FAILURE
                }
            },
        }
    }

    pub fn interpret(&mut self, input: &str) -> i32 {
        self.exit_code = match parsing::Parser::new(input).parse(0) {
            Ok(ast) => {
                // self.io.println(format!("{ast:#?}\n"));
                self.execute(None, &ast)
            }
            Err(e) => {
                self.io.eprintln(format!("crsh: parsing error: {e}"));
                -1
            }
        };

        self.exit_code
    }

    pub fn find_on_path<P: AsRef<Path>>(&self, keyword: P) -> Option<PathBuf> {
        self.env
            .path
            .iter()
            .filter_map(|dir| {
                let path = dir.join(&keyword);

                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn config_filepath<S: AsRef<Path>>(&self, filename: S) -> PathBuf {
        let mut path = self.env.config.clone();
        path.push(filename);
        path
    }
}
