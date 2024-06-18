use std::io::Write;
use std::path::{Path, PathBuf};

mod builtin;
mod common_env;
mod executing;
mod io_descriptors;
mod parsing;

pub use common_env::*;
pub use io_descriptors::*;

#[derive(Default, Clone)]
pub struct IOContext {
    pub input: Input,
    pub output: Output,
    pub error: Error,
}

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
                self.println(format!("{ast:#?}\n"));
                executing::execute(self, &self.io.clone(), &ast)
            }
            Err(e) => {
                self.eprintln(format!("crsh: parsing error: {e}"));
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

    pub fn eprint<S: AsRef<str>>(&mut self, msg: S) {
        let _ = self.io.error.write_all(msg.as_ref().as_bytes());
    }

    pub fn eprintln<S: AsRef<str>>(&mut self, msg: S) {
        self.eprint(format!("{}\n", msg.as_ref()));
    }

    pub fn print<S: AsRef<str>>(&mut self, msg: S) {
        let _ = self.io.output.write_all(msg.as_ref().as_bytes());
    }

    pub fn println<S: AsRef<str>>(&mut self, msg: S) {
        self.print(format!("{}\n", msg.as_ref()));
    }
}
