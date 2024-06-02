use crate::builtin;
use crate::system;

#[derive(Default, Clone)]
pub struct ExecutionContext {
    pub input: system::io::Input,
    pub output: system::io::Output,
    pub error: system::io::Error,
}

pub trait Executable {
    fn run(&self, ctx: ExecutionContext) -> Result<(), ()>;
}

#[derive(Default)]
pub struct Interpreter {
    ctx: ExecutionContext,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&self, input: &str) -> Result<(), ()> {
        let mut parts = input.split_whitespace();
        let keyword = parts.next().unwrap_or_default();
        let args = parts.collect::<Vec<_>>();
        let ctx = ExecutionContext { ..self.ctx.clone() };

        if let Some(builtin) = builtin::build(keyword, &args) {
            builtin.run(ctx)
        } else if let Some(path) = system::find_on_path(keyword) {
            system::Launch::new(path, args).run(ctx)
        } else {
            eprintln!("crsh: command not found: {keyword}");
            Err(())
        }
    }
}
