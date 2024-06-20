use std::path::{Path, PathBuf};
use std::process::{exit, ExitCode};
use std::env;

mod builtin;
mod common_env;
mod config;
mod execution;
mod parsing;
mod shell_io;

use common_env::*;
use config::*;
use shell_io::*;

use crate::prompt::Prompt;

const TARGET: &str = env!("TARGET");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_REPO: &str = env!("CARGO_PKG_REPOSITORY");

const HELP_TEXT: &str = r#"Usage: crsh [<options>] [<argument> ...]

Special options:
  --help     show this message, then exit
  --version  show crsh version number, then exit
  -b         end option processing, like --
  -c         take first argument as a command to execute
  -o OPTION  set an option by name (not yet implemented)"#;

#[derive(Default)]
pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: i32,
    pub io: IOContext,
    pub config: ShellConfig,
}

impl Shell {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn main(&mut self) -> ExitCode {
        self.parse_args();

        match self.config.mode {
            ShellMode::Interactive => {
                if Prompt::new(self).interactive_loop().is_ok() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            ShellMode::Command => {
                if let Some(input) = self.config.args.clone().first() {
                    self.interpret(input.as_str());
                    ExitCode::SUCCESS
                } else {
                    eprintln!("crsh: string expected after -c");
                    ExitCode::FAILURE
                }
            }
            ShellMode::Script => {
                if let Some(path) = self.config.args.first() {
                    eprintln!("crsh: tried to call script \"{path}\": scripts not yet implemented");
                    ExitCode::FAILURE
                } else {
                    eprintln!("crsh: scripts not yet implemented");
                    ExitCode::FAILURE
                }
            }
        }
    }

    fn parse_args(&mut self) {
        let mut in_options = true;
        let mut args = env::args();
        self.config.start_path = args.next().unwrap_or_default();

        for arg in args {
            if !arg.starts_with('-') {
                in_options = false;
            }

            if !in_options {
                self.config.args.push(arg);
                continue;
            }

            match arg.as_str() {
                "-b" | "--" => in_options = false,
                "-c" => {
                    self.config.mode = ShellMode::Command;
                }
                "-o" => {
                    eprintln!("crsh: options not yet implemented");
                    exit(0);
                }
                "--version" => {
                    println!("{PKG_NAME} {PKG_VERSION} ({TARGET})\n{PKG_REPO}");
                    exit(0);
                }
                "--help" => {
                    println!("{HELP_TEXT}");
                    exit(0);
                }
                _ => {
                    eprintln!("crsh: no such option: {arg}");
                    exit(0);
                }
            };
        }

        if self.config.mode == ShellMode::Interactive && !self.config.args.is_empty() {
            self.config.mode = ShellMode::Script;
        }
    }

    pub fn interpret(&mut self, input: &str) {
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
    }

    pub fn find_on_path<P>(&self, keyword: P) -> Option<PathBuf>
    where
        P: AsRef<Path>,
    {
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
