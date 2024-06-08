use std::env;
use std::io::Write;

use crate::builtin::Builtin;
use crate::system::ExecutionContext;

enum PwdOption {
    L,
    P,
    None,
}

pub struct Pwd {
    option: PwdOption,
}

impl Builtin for Pwd {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        let mut option = PwdOption::None;

        for arg in args {
            match *arg {
                "-L" => option = PwdOption::L,
                "-P" => option = PwdOption::P,
                _ => return Err(format!("pwd: bad argument: {arg}\n")),
            }
        }

        Ok(Box::new(Self { option }))
    }

    #[allow(clippy::match_single_binding)]
    fn run(&self, mut ctx: ExecutionContext) -> i32 {
        match &self.option {
            // -L and -P options not yet implemented
            // todo: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html
            _ => {
                if let Ok(path) = env::current_dir() {
                    let msg = format!("{}\n", path.display());
                    let _ = ctx.output.write_all(msg.as_bytes());
                    0
                } else {
                    let msg = "pwd: failed to read current directory\n";
                    let _ = ctx.error.write_all(msg.as_bytes());
                    -1
                }
            }
        }
    }
}
