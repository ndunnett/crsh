use std::env;
use std::io::Write;
use std::path::Path;

use crate::builtin::Builtin;
use crate::interpreter::ExecutionContext;
use crate::system;

enum CdOption {
    L,
    P,
    Back,
    None,
}

pub struct Cd {
    option: CdOption,
    path: Option<String>,
}

impl Builtin for Cd {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
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
                    Some(&arg) => return Err(format!("cd: bad argument: {}\n", arg)),
                    None => {}
                };

                path = Some(args[1].to_string());
            }
            _ => return Err("cd: too many arguments\n".to_string()),
        }

        Ok(Box::new(Self { option, path }))
    }

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
            let msg = format!("cd: cannot access '{path}': No such file or directory\n");
            let _ = ctx.error.write_all(msg.as_bytes());
            Err(())
        } else {
            if let Ok(pwd) = env::current_dir() {
                env::set_var("OLDPWD", pwd);
            }

            if let Err(e) = env::set_current_dir(&path) {
                let msg = format!("cd: cannot access '{path}': {e}\n");
                let _ = ctx.error.write_all(msg.as_bytes());
                Err(())
            } else {
                Ok(())
            }
        }
    }
}
