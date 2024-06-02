use std::io::Write;

use crate::builtin;
use crate::interpreter::{Executable, ExecutionContext};
use crate::system;

pub struct Type {
    keyword: String,
}

impl Type {
    pub fn build(args: &[&str]) -> Box<dyn Executable> {
        let keyword = args.first().map(|&s| s.to_string()).unwrap_or_default();
        Box::new(Self { keyword })
    }
}

impl Executable for Type {
    fn run(&self, mut ctx: ExecutionContext) -> Result<(), ()> {
        let msg = if builtin::get(&self.keyword).is_some() {
            format!("{} is a shell builtin\n", self.keyword)
        } else if let Some(path) = system::find_on_path(&self.keyword) {
            format!("{} is {}\n", self.keyword, path.display())
        } else {
            format!("{} not found\n", self.keyword)
        };

        if ctx.output.write_all(msg.as_bytes()).is_err() {
            Err(())
        } else {
            Ok(())
        }
    }
}
