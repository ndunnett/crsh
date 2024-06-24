use std::path::{Path, PathBuf};

use sysexits::ExitCode;

mod builtin;
mod common_env;
mod config;
mod execution;
mod parsing;
mod shell_io;

pub use common_env::*;
pub use config::*;
pub use shell_io::*;

pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: ExitCode,
    pub io: IOContext,
    pub config: ShellConfig,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            env: CommonEnv::default(),
            exit_code: ExitCode::Ok,
            io: IOContext::default(),
            config: ShellConfig::default(),
        }
    }
}

impl Shell {
    pub fn interpret(&mut self, input: &str) -> ExitCode {
        self.exit_code = match parsing::Parser::new(input).parse(0) {
            Ok(ast) => {
                // self.io.println(format!("{ast:#?}\n"));
                self.execute(None, &ast)
            }
            Err(e) => {
                self.io.eprintln(format!("crsh: parsing error: {e}"));
                ExitCode::DataErr
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
