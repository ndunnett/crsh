use std::path::{Path, PathBuf};

mod builtin;
mod common_env;
mod execution;
mod parsing;
mod shell_io;

use common_env::*;
use shell_io::*;

#[derive(Default)]
pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: i32,
    pub io: IOContext,
}

impl Shell {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn interpret(&mut self, input: &str) {
        self.exit_code = match parsing::Parser::new(input).parse(0) {
            Ok(ast) => {
                self.io.println(format!("{ast:#?}\n"));
                self.execute(&mut self.io.clone(), &ast)
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
}
