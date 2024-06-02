use std::process::exit;

use crate::interpreter::{Executable, ExecutionContext};

pub struct Exit {
    code: i32,
}

impl Exit {
    pub fn build(args: &[&str]) -> Box<dyn Executable> {
        let code = args
            .first()
            .map(|arg| arg.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);

        Box::new(Self { code })
    }
}

impl Executable for Exit {
    fn run(&self, _ctx: ExecutionContext) -> Result<(), ()> {
        exit(self.code)
    }
}
