use sysexits::ExitCode;

use crate::{builtin::Builtin, io::IOContext, Shell};

impl Builtin {
    pub(super) fn which(shell: &mut Shell, io: &mut IOContext, args: &[&str]) -> ExitCode {
        if let Some(keyword) = args.first() {
            if Builtin::get(keyword).is_some() {
                io.println(format!("{}: shell builtin", keyword));
            } else if let Some(path) = shell.find_on_path(keyword) {
                io.println(path.display().to_string());
            } else {
                io.println(format!("{} not found", keyword));
            }
        }

        ExitCode::Ok
    }
}
