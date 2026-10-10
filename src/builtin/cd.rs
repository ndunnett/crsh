use std::{env, path::Path};

use argh::FromArgs;
use sysexits::ExitCode;

use crate::{Shell, builtin::Builtin, dir, io};

#[derive(FromArgs)]
/// Change the current working directory.
struct Cli {
    /// pathname of the new working directory
    #[argh(positional)]
    directory: Option<String>,

    /// handle the operand dot-dot logically
    #[argh(switch, short = 'L')]
    logical: bool,

    /// handle the operand dot-dot physically
    #[argh(switch, short = 'P')]
    physical: bool,
}

impl Builtin {
    pub(super) fn cd(shell: &mut Shell, io: &mut io::Context, args: &[&str]) -> ExitCode {
        let args = if args == ["-"] {
            &[shell.old_pwd.as_str()]
        } else {
            args
        };

        let cli = match Cli::from_args(&["cd"], args) {
            Ok(cli) => cli,
            Err(e) => {
                shell.io.eprintln(e.output);
                return ExitCode::Usage;
            }
        };

        if cli.logical || cli.physical {
            // https://pubs.opengroup.org/onlinepubs/9699919799/utilities/cd.html
            shell.io.eprintln("-L and -P options not yet implemented");
            return ExitCode::Usage;
        }

        let path = cli.directory.unwrap_or_else(dir::my_home);

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

            unsafe {
                env::set_var("OLDPWD", &shell.old_pwd);
                env::set_var("PWD", &shell.pwd);
            }

            ExitCode::Ok
        }
    }
}
