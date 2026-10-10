use sysexits::ExitCode;

use crate::{Shell, builtin::Builtin, io};

impl Builtin {
    pub(super) fn exit(shell: &mut Shell, _io: &mut io::Context, args: &[&str]) -> ExitCode {
        shell.should_exit = true;

        args.first().map_or(ExitCode::Ok, |arg| {
            arg.parse::<i32>()
                .unwrap_or(0)
                .try_into()
                .unwrap_or(ExitCode::Ok)
        })
    }
}
