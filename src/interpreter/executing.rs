use crate::builtin;
use crate::interpreter::parsing::Command;
use crate::shell::Shell;

pub fn execute(sh: &mut Shell, ast: &Command) -> i32 {
    match ast {
        Command::Empty => 0,
        Command::Simple { keyword, args } => execute_simple(sh, keyword, args),
        _ => {
            sh.eprintln("crsh: unimplemented functionality");
            -1
        }
    }
}

fn execute_simple(sh: &mut Shell, keyword: &str, args: &[&str]) -> i32 {
    if let Some(builder) = builtin::get_builder(keyword) {
        match builder(args) {
            Ok(builtin) => builtin.run(sh),
            Err(e) => {
                sh.eprintln(e);
                -1
            }
        }
    } else if sh.find_on_path(keyword).is_some() {
        sh.launch(keyword, args)
    } else {
        sh.eprintln(format!("crsh: command not found: {keyword}"));
        -1
    }
}
