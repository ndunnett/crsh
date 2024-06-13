use std::env;

use crate::builtin::Builtin;
use crate::shell::Shell;

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
                _ => return Err(format!("pwd: bad argument: {arg}")),
            }
        }

        Ok(Box::new(Self { option }))
    }

    #[allow(clippy::match_single_binding)]
    fn run(&self, sh: &mut Shell) -> i32 {
        match &self.option {
            // -L and -P options not yet implemented
            // todo: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/pwd.html
            _ => {
                if let Ok(path) = env::current_dir() {
                    sh.println(path.display().to_string());
                    0
                } else {
                    sh.eprintln("pwd: failed to read current directory");
                    -1
                }
            }
        }
    }
}
