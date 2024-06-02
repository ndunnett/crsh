use std::io::Write;

use crate::builtin;
use crate::interpreter::{Executable, ExecutionContext};
use crate::system;

pub struct Which {
    keyword: String,
}

impl Which {
    pub fn build(args: &[&str]) -> Box<dyn Executable> {
        let keyword = args.first().map(|&s| s.to_string()).unwrap_or_default();
        Box::new(Self { keyword })
    }
}

impl Executable for Which {
    fn run(&self, mut ctx: ExecutionContext) -> Result<(), ()> {
        let msg = if builtin::get(&self.keyword).is_some() {
            format!("{}: shell builtin\n", self.keyword)
        } else if let Some(path) = system::find_on_path(&self.keyword) {
            format!("{}\n", path.display())
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
