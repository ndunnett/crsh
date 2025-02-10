use sysexits::ExitCode;

use lib_os::io;

use crate::{builtin::Builtin, Shell};

impl Builtin {
    pub(super) fn exit(shell: &mut Shell, _io: &mut io::Context, args: &[&str]) -> ExitCode {
        shell.should_exit = true;

        args.first()
            .map(|arg| {
                arg.parse::<i32>()
                    .unwrap_or(0)
                    .try_into()
                    .unwrap_or(ExitCode::Ok)
            })
            .unwrap_or(ExitCode::Ok)
    }
}
