use sysexits::ExitCode;

use super::{Builtin, ImplementedBuiltin};
use crate::{IOContext, Shell};

pub struct Which {
    keyword: Option<String>,
}

impl ImplementedBuiltin for Which {
    fn build(args: &[String]) -> Result<impl ImplementedBuiltin, String> {
        Ok(Self {
            keyword: args.first().map(String::to_string),
        })
    }

    fn run(&self, sh: &mut Shell, io: &mut IOContext) -> ExitCode {
        if let Some(keyword) = &self.keyword {
            if Builtin::get(keyword).is_some() {
                io.println(format!("{}: shell builtin", keyword));
            } else if let Some(path) = sh.find_on_path(keyword) {
                io.println(path.display().to_string());
            } else {
                io.println(format!("{} not found", keyword));
            }
        }

        ExitCode::Ok
    }
}
