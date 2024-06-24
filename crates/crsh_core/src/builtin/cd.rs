use std::env;
use std::path::Path;

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
    fn build(args: &[&str]) -> Result<impl ImplementedBuiltin, String> {
        match Cli::try_parse_from(["cd"].iter().chain(args)) {
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

    fn run(&self, sh: &mut Shell, io: &mut IOContext) -> ExitCode {
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
            return ExitCode::NoInput;
        }

        if let Err(e) = env::set_current_dir(&path) {
            io.eprintln(format!("cd: cannot access '{path}': {e}"));
            ExitCode::NoInput
        } else {
            let pwd = env::current_dir().unwrap_or_else(|_| path.into());
            env::set_var("OLDPWD", &sh.env.pwd);
            sh.env.oldpwd.clone_from(&sh.env.pwd);
            sh.env.pwd = pwd;
            ExitCode::Ok
        }
    }
}
