use std::process::exit;

use crate::builtin::Builtin;
use crate::system::ExecutionContext;

pub struct Exit {
    code: i32,
}

impl Builtin for Exit {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        let code = args
            .first()
            .map(|arg| arg.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);

        Ok(Box::new(Self { code }))
    }

    fn run(&self, _ctx: ExecutionContext) -> i32 {
        exit(self.code)
    }
}
