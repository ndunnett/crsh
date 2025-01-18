use std::io::{Read, Write};

use super::{input::Input, output::Output};

#[derive(Debug)]
pub struct IOContext {
    pub input: Input,
    pub output: Output,
    pub error: Output,
}

impl Default for IOContext {
    fn default() -> Self {
        Self {
            input: Input::Stdin(std::io::stdin()),
            output: Output::Stdout(std::io::stdout()),
            error: Output::Stderr(std::io::stderr()),
        }
    }
}

impl IOContext {
    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self {
            input: self.input.try_clone()?,
            output: self.output.try_clone()?,
            error: self.error.try_clone()?,
        })
    }

    pub fn _null() -> Self {
        Self {
            input: Input::Null,
            output: Output::Null,
            error: Output::Null,
        }
    }

    pub fn _read(&mut self) -> std::io::Result<String> {
        let mut buffer = String::new();
        self.input.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

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
