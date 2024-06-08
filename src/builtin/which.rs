use std::io::Write;

use crate::builtin::{self, Builtin};
use crate::system::ExecutionContext;
use crate::system;

pub struct Which {
    keyword: String,
}

impl Builtin for Which {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        let keyword = args.first().map(|&s| s.to_string()).unwrap_or_default();
        Ok(Box::new(Self { keyword }))
    }

    fn run(&self, mut ctx: ExecutionContext) -> i32 {
        let msg = if builtin::get_builder(&self.keyword).is_some() {
            format!("{}: shell builtin\n", self.keyword)
        } else if let Some(path) = system::find_on_path(&self.keyword) {
            format!("{}\n", path.display())
        } else {
            format!("{} not found\n", self.keyword)
        };

        if ctx.output.write_all(msg.as_bytes()).is_err() {
            -1
        } else {
            0
        }
    }
}
