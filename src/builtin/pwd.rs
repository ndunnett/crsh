use std::env;
use std::io::Write;

use crate::interpreter::{Executable, ExecutionContext};

enum PwdOption {
    L,
    P,
    None,
    Bad(String),
}

pub struct Pwd {
    option: PwdOption,
}

impl Pwd {
    pub fn build(args: &[&str]) -> Box<dyn Executable> {
        let mut option = PwdOption::None;

        for arg in args {
            match *arg {
                "-L" => option = PwdOption::L,
                "-P" => option = PwdOption::P,
                _ => {
                    option = PwdOption::Bad(arg.to_string());
                    break;
                }
            }
        }

        Box::new(Self { option })
    }
}

impl Executable for Pwd {
    fn run(&self, mut ctx: ExecutionContext) -> Result<(), ()> {
        match &self.option {
            PwdOption::Bad(arg) => {
                let msg = format!("pwd: bad argument: {arg}\n");
                let _ = ctx.error.write_all(msg.as_bytes());
                Err(())
            }
            // -L and -P options not yet implemented
            // todo: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html
            _ => {
                if let Ok(path) = env::current_dir() {
                    let msg = format!("{}\n", path.display());
                    let _ = ctx.output.write_all(msg.as_bytes());
                    Ok(())
                } else {
                    let msg = "pwd: failed to read current directory";
                    let _ = ctx.error.write_all(msg.as_bytes());
                    Err(())
                }
            }
        }
    }
}
