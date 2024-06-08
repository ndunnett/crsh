use std::io::Write;

mod executing;
mod parsing;

use crate::system::ExecutionContext;

pub fn interpret(input: &str) -> i32 {
    let mut ctx = ExecutionContext::default();

    let ast = match parsing::parse(input) {
        Ok(ast) => {
            println!("{ast:#?}\n");
            ast
        }
        Err(e) => {
            let msg = format!("crsh: {e}\n");
            let _ = ctx.error.write_all(msg.as_bytes());
            return -1;
        }
    };

    executing::execute(&mut ctx, &ast)
}
