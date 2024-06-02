use std::io::Write;

use crate::interpreter::{Executable, ExecutionContext};

pub struct Echo {
    message: String,
}

impl Echo {
    pub fn build(args: &[&str]) -> Box<dyn Executable> {
        Box::new(Self {
            message: format!("{}\n", args.join(" ")),
        })
    }
}

impl Executable for Echo {
    fn run(&self, mut ctx: ExecutionContext) -> Result<(), ()> {
        if ctx.output.write_all(self.message.as_bytes()).is_err() {
            Err(())
        } else {
            Ok(())
        }
    }
}
