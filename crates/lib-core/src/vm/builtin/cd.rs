use std::{env, path::Path};

use clap::Parser;
use sysexits::ExitCode;

use super::ImplementedBuiltin;
use crate::{IOContext, Shell};

#[derive(Parser)]
#[group(multiple = false)]
struct Cli {
    /// Pathname of the new working directory
    directory: Option<String>,

    /// Handle the operand dot-dot logically
    #[arg(short = 'L', group = "dotdot")]
    logical: bool,

    /// Handle the operand dot-dot physically
    #[arg(short = 'P', group = "dotdot")]
    physical: bool,
}

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
    fn build(args: &[String]) -> Result<impl ImplementedBuiltin, String> {
        match Cli::try_parse_from(["cd".to_string()].iter().chain(args)) {
            Ok(cli) => {
                let mut option = CdOption::None;
                let mut path = None;

                if cli.logical {
                    option = CdOption::L;
                }

                if cli.physical {
                    option = CdOption::P;
                }

                if let Some(dir) = &cli.directory {
                    if dir == "-" {
                        option = CdOption::Back;
                    } else {
                        path = cli.directory;
                    }
                }

                Ok(Cd { option, path })
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn run(&self, _shell: &mut Shell, io: &mut IOContext) -> ExitCode {
        let path = match (&self.path, &self.option) {
            (None, CdOption::Back) => env::var("OLDPWD").unwrap_or_default(),
            (None, _) => env::var("HOME").unwrap_or_default(),
            // -L and -P options not yet implemented
            // todo: https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html
            (Some(s), _) => {
                if s.starts_with('~') {
                    s.replacen('~', &env::var("HOME").unwrap_or_default(), 1)
                } else {
                    s.into()
                }
            }
        };

        if !Path::new(&path).is_dir() {
            io.eprintln(format!(
                "cd: cannot access '{path}': No such file or directory"
            ));
            return ExitCode::NoInput;
        }

        let pwd = env::current_dir().unwrap_or_default();

        if let Err(e) = env::set_current_dir(&path) {
            io.eprintln(format!("cd: cannot access '{path}': {e}"));
            ExitCode::NoInput
        } else {
            env::set_var("PWD", env::current_dir().unwrap_or_default());
            env::set_var("OLDPWD", pwd);
            ExitCode::Ok
        }
    }
}
