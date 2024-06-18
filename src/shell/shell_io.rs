use std::io::{self, Write};

mod input;
mod output;

pub use input::*;
pub use output::*;

#[derive(Clone)]
pub struct IOContext {
    pub input: Input,
    pub output: Output,
    pub error: Output,
}

impl Default for IOContext {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
            error: Output::Stderr(io::stderr()),
        }
    }
}

impl IOContext {
    pub fn print<S: AsRef<str>>(&mut self, msg: S) {
        let _ = self.output.write_all(msg.as_ref().as_bytes());
    }

    pub fn println<S: AsRef<str>>(&mut self, msg: S) {
        self.print(format!("{}\n", msg.as_ref()));
    }

    pub fn eprint<S: AsRef<str>>(&mut self, msg: S) {
        let _ = self.error.write_all(msg.as_ref().as_bytes());
    }

    pub fn eprintln<S: AsRef<str>>(&mut self, msg: S) {
        self.eprint(format!("{}\n", msg.as_ref()));
    }
}
