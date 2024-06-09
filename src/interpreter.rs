use std::io::Write;

mod executing;
mod parsing;

use crate::shell::Shell;

pub fn interpret(shell: &mut Shell, input: &str) -> i32 {
    let ast = match parsing::parse(input) {
        Ok(ast) => {
            println!("{ast:#?}\n");
            ast
        }
        Err(e) => {
            let msg = format!("crsh: {e}\n");
            let _ = shell.error.write_all(msg.as_bytes());
            return -1;
        }
    };

    executing::execute(shell, &ast)
}
