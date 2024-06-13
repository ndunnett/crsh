use crate::shell::Shell;

mod executing;
mod parsing;

pub fn interpret(sh: &mut Shell, input: &str) -> i32 {
    match parsing::Parser::new(input).parse(0) {
        Ok(ast) => {
            sh.println(format!("{ast:#?}\n"));
            executing::execute(sh, &ast)
        }
        Err(e) => {
            sh.eprintln(format!("crsh: parsing error: {e}"));
            -1
        }
    }
}
