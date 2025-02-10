use std::{env, path::Path};

use clap::Parser;
use sysexits::ExitCode;

use lib_os::{dir, io};

use crate::{builtin::Builtin, Shell};

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
    pub(super) fn cd(shell: &mut Shell, io: &mut io::Context, args: &[&str]) -> ExitCode {
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
                shell.old_pwd.clone()
            } else {
                dir.to_string()
            }
        } else {
            dir::my_home()
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
            shell.old_pwd = shell.pwd.clone();
            shell.pwd = dir::current();
            env::set_var("OLDPWD", &shell.old_pwd);
            env::set_var("PWD", &shell.pwd);
            ExitCode::Ok
        }
    }
}
