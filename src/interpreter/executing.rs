use crate::builtin;
use crate::shell::Shell;

use super::parsing::*;

fn execute_simple(shell: &mut Shell, keyword: &str, args: &[&str]) -> i32 {
    if let Some(builder) = builtin::get_builder(keyword) {
        match builder(args) {
            Ok(builtin) => builtin.run(shell),
            Err(e) => {
                shell.eprintln(e);
                -1
            }
        }
    } else if shell.find_on_path(keyword).is_some() {
        shell.launch(keyword, args)
    } else {
        shell.eprintln(format!("crsh: command not found: {keyword}"));
        -1
    }
}

pub fn execute(shell: &mut Shell, ast: &Command) -> i32 {
    match ast {
        Command::Empty => 0,
        Command::Simple { keyword, args } => execute_simple(shell, keyword, args),
        _ => {
            shell.eprintln("crsh: unimplemented functionality");
            -1
        }
    }
}
