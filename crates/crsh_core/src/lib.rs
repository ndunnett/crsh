use std::path::{Path, PathBuf};

mod builtin;
mod common_env;
mod config;
mod execution;
mod parsing;
mod shell_io;

pub use common_env::*;
pub use config::*;
pub use shell_io::*;

#[derive(Default)]
pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: i32,
    pub io: IOContext,
    pub config: ShellConfig,
}

impl Shell {
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
