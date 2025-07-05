use sysexits::ExitCode;

use lib_os::{dir, io};

use crate::{builtin::Builtin, Shell};

impl Builtin {
    pub(super) fn which(_shell: &mut Shell, io: &mut io::Context, args: &[&str]) -> ExitCode {
        if let Some(keyword) = args.first() {
            if Builtin::get(keyword).is_some() {
                io.println(format!("{keyword}: shell builtin"));
            } else if let Some(path) = dir::find_on_path(keyword) {
                io.println(path.display().to_string());
            } else {
                io.println(format!("{keyword} not found"));
            }
        }

        ExitCode::Ok
    }
}
