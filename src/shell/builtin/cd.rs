use std::env;
use std::path::Path;

use super::ImplementedBuiltin;
use crate::shell::{IOContext, Shell};

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

impl ImplementedBuiltin for Cd {
    fn build(args: &[&str]) -> Result<impl ImplementedBuiltin, String> {
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
                    Some(&arg) => return Err(format!("cd: bad argument: {arg}")),
                    None => {}
                };

                path = Some(args[1].to_string());
            }
            _ => return Err("cd: too many arguments".to_string()),
        }

        Ok(Cd { option, path })
    }

    fn run(&self, sh: &mut Shell, io: &mut IOContext) -> i32 {
        let path = match (&self.path, &self.option) {
            (None, CdOption::Back) => sh.env.oldpwd.to_string_lossy().to_string(),
            (None, _) => sh.env.home.to_string_lossy().to_string(),
            // -L and -P options not yet implemented
            // todo: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html
            (Some(s), _) => {
                if s.starts_with('~') {
                    s.replacen('~', &sh.env.home.to_string_lossy(), 1)
                } else {
                    s.into()
                }
            }
        };

        if !Path::new(&path).is_dir() {
            io.eprintln(format!(
                "cd: cannot access '{path}': No such file or directory"
            ));
            return -1;
        }

        if let Err(e) = env::set_current_dir(&path) {
            io.eprintln(format!("cd: cannot access '{path}': {e}"));
            -1
        } else {
            let pwd = env::current_dir().unwrap_or_else(|_| path.into());
            env::set_var("OLDPWD", &sh.env.pwd);
            sh.env.oldpwd.clone_from(&sh.env.pwd);
            sh.env.pwd = pwd;
            0
        }
    }
}
