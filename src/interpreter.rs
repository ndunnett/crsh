use std::io::Write;

use crate::builtin;
use crate::system;

#[derive(Default, Clone)]
pub struct ExecutionContext {
    pub input: system::io::Input,
    pub output: system::io::Output,
    pub error: system::io::Error,
}

pub fn execute(input: &str) -> Result<(), ()> {
    let mut parts = input.split_whitespace();
    let keyword = parts.next().unwrap_or_default();
    let args = parts.collect::<Vec<_>>();
    let mut ctx = ExecutionContext::default();

    if let Some(builder) = builtin::get_builder(keyword) {
        match builder(&args) {
            Ok(builtin) => builtin.run(ctx),
            Err(e) => {
                let _ = ctx.error.write_all(e.as_bytes());
                Err(())
            }
        }
    } else if system::find_on_path(keyword).is_some() {
        system::launch(keyword, args, ctx)
    } else {
        let msg = format!("crsh: command not found: {keyword}\n");
        let _ = ctx.error.write_all(msg.as_bytes());
        Err(())
    }
}
