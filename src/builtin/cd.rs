use std::env;
use std::io::Write;
use std::path::Path;

use crate::interpreter::{Executable, ExecutionContext};
use crate::system;

enum CdOption {
    L,
    P,
    Back,
    None,
    Bad(String),
}

pub struct Cd {
    option: CdOption,
    path: Option<String>,
}

impl Cd {
    pub fn build(args: &[&str]) -> Box<dyn Executable> {
        let mut option = CdOption::None;
        let mut path = None;

        match args.len() {
            0 => {}
            1 => match args.first() {
                Some(&"-") => option = CdOption::Back,
                Some(&"-L") => option = CdOption::L,
                Some(&"-P") => option = CdOption::P,
                Some(&arg) => path = Some(arg.to_string()),
                None => {}
            },
            2 => {
                match args.first() {
                    Some(&"-L") => option = CdOption::L,
                    Some(&"-P") => option = CdOption::P,
                    Some(&arg) => option = CdOption::Bad(format!("bad argument: {}", arg)),
                    None => {}
                };

                path = Some(args[1].to_string());
            }
            _ => option = CdOption::Bad("too many arguments".to_string()),
        }

        Box::new(Self { option, path })
    }
}

impl Executable for Cd {
    fn run(&self, mut ctx: ExecutionContext) -> Result<(), ()> {
        let path = match (&self.path, &self.option) {
            (None, CdOption::Back) => {
                if let Some(oldpwd) = env::var_os("OLDPWD") {
                    oldpwd.into_string().unwrap_or("".into())
                } else if let Ok(pwd) = env::current_dir() {
                    format!("{}", pwd.display())
                } else {
                    "".to_string()
                }
            }
            (None, _) => system::home(),
            (_, CdOption::Bad(arg)) => {
                let msg = format!("cd: {arg}\n");
                let _ = ctx.error.write_all(msg.as_bytes());
                return Err(());
            }
            // -L and -P options not yet implemented
            // todo: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html
            (Some(s), _) => {
                if s.starts_with('~') {
                    s.replacen('~', &system::home(), 1)
                } else {
                    s.to_string()
                }
            }
        };

        if !Path::new(&path).is_dir() {
            let msg = format!("cd: {}: no such file or directory\n", path);
            let _ = ctx.error.write_all(msg.as_bytes());
            Err(())
        } else {
            if let Ok(pwd) = env::current_dir() {
                env::set_var("OLDPWD", pwd);
            }

            if let Err(e) = env::set_current_dir(&path) {
                let msg = format!("cd: {e}\n");
                let _ = ctx.error.write_all(msg.as_bytes());
                Err(())
            } else {
                Ok(())
            }
        }
    }
}
