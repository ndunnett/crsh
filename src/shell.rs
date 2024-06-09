use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

mod io_descriptors;
mod common_env;

pub use io_descriptors::*;
pub use common_env::*;

#[derive(Default)]
pub struct Shell {
    pub common_env: CommonEnv,
    pub exit_code: i32,
    pub input: Input,
    pub output: Output,
    pub error: Error,
}

impl Shell {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn eprint<S: AsRef<str>>(&mut self, msg: S) {
        let _ = self.error.write_all(msg.as_ref().as_bytes());
    }

    pub fn eprintln<S: AsRef<str>>(&mut self, msg: S) {
        self.eprint(format!("{}\n", msg.as_ref()));
    }

    pub fn print<S: AsRef<str>>(&mut self, msg: S) {
        let _ = self.output.write_all(msg.as_ref().as_bytes());
    }

    pub fn println<S: AsRef<str>>(&mut self, msg: S) {
        self.print(format!("{}\n", msg.as_ref()));
    }

    pub fn launch(&mut self, keyword: &str, args: &[&str]) -> i32 {
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

        match Command::new(keyword)
            .stdin(self.input.clone())
            .stdout(self.output.clone())
            .stderr(self.error.clone())
            .args(&args)
            .spawn()
        {
            Ok(mut c) => match c.wait() {
                Ok(status) => status.code().unwrap_or(-1),
                Err(e) => {
                    self.eprintln(format!("crsh: {e}"));
                    -1
                }
            },
            Err(e) => {
                self.eprintln(format!("crsh: {e}"));
                -1
            }
        }
    }

    pub fn find_on_path<P>(&self, keyword: P) -> Option<PathBuf>
    where
        P: AsRef<Path>,
    {
        self.common_env
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
