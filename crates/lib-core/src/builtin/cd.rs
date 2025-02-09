use std::{env, path::Path};

use clap::Parser;
use sysexits::ExitCode;

use crate::{builtin::Builtin, io::IOContext, Shell};

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

impl Builtin {
    pub(super) fn cd(shell: &mut Shell, io: &mut IOContext, args: &[&str]) -> ExitCode {
        let cli = match Cli::try_parse_from(["cd"].iter().chain(args)) {
            Ok(cli) => cli,
            Err(e) => {
                shell.io.eprintln(e.to_string());
                return ExitCode::Usage;
            }
        };

        if cli.logical || cli.physical {
            // https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html
            shell.io.eprintln("-L and -P options not yet implemented");
            return ExitCode::Usage;
        }

        let path = if let Some(dir) = &cli.directory {
            if dir == "-" {
                env::var("OLDPWD").unwrap_or_default()
            } else {
                dir.to_string()
            }
        } else {
            env::var("HOME").unwrap_or_default()
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
