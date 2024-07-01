use sysexits::ExitCode;

use super::ImplementedBuiltin;
use crate::{IOContext, Shell};

pub struct Exit {
    code: ExitCode,
}

impl ImplementedBuiltin for Exit {
    fn build(args: &[String]) -> Result<impl ImplementedBuiltin, String> {
        let code: ExitCode = args
            .first()
            .map(|arg| {
                arg.parse::<i32>()
                    .unwrap_or(0)
                    .try_into()
                    .unwrap_or(ExitCode::Ok)
            })
            .unwrap_or(ExitCode::Ok);

        Ok(Self { code })
    }

    fn run(&self, _sh: &mut Shell, _io: &mut IOContext) -> ExitCode {
        self.code.exit()
    }
}
