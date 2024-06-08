use std::io::Write;

use crate::builtin;
use crate::system::{find_on_path, launch, ExecutionContext};

use super::parsing::*;

fn execute_simple(ctx: &mut ExecutionContext, keyword: &str, args: &[&str]) -> i32 {
    if let Some(builder) = builtin::get_builder(keyword) {
        match builder(args) {
            Ok(builtin) => builtin.run(ctx.clone()),
            Err(e) => {
                let _ = ctx.error.write_all(e.as_bytes());
                -1
            }
        }
    } else if find_on_path(keyword).is_some() {
        launch(keyword, args, ctx.clone())
    } else {
        let msg = format!("crsh: command not found: {keyword}\n");
        let _ = ctx.error.write_all(msg.as_bytes());
        -1
    }
}

pub fn execute(ctx: &mut ExecutionContext, ast: &Command) -> i32 {
    match ast {
        Command::Empty => 0,
        Command::Simple { keyword, args } => execute_simple(ctx, keyword, args),
        _ => {
            let msg = "crsh: error: unimplemented functionality\n";
            let _ = ctx.error.write_all(msg.as_bytes());
            -1
        }
    }
}
