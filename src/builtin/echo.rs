use std::io::Write;

use crate::builtin::Builtin;
use crate::system::ExecutionContext;

pub struct Echo {
    message: String,
}

impl Builtin for Echo {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        Ok(Box::new(Self {
            message: format!("{}\n", args.join(" ")),
        }))
    }

    fn run(&self, mut ctx: ExecutionContext) -> i32 {
        if ctx.output.write_all(self.message.as_bytes()).is_err() {
            -1
        } else {
            0
        }
    }
}
