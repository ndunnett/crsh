use sysexits::ExitCode;

use crate::{builtin::Builtin, io::IOContext, Shell};

impl Builtin {
    pub(super) fn exit(shell: &mut Shell, _io: &mut IOContext, args: &[&str]) -> ExitCode {
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
