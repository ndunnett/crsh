use sysexits::ExitCode;

use crate::{IOContext, Shell};

mod cd;
mod exit;
mod which;

trait ImplementedBuiltin {
    fn build(args: &[String]) -> Result<impl ImplementedBuiltin, String>;
    fn run(&self, shell: &mut Shell, io: &mut IOContext) -> ExitCode;
}

pub enum Builtin {
    Cd,
    Exit,
    Which,
}

impl Builtin {
    pub fn get(keyword: &str) -> Option<Self> {
        match keyword {
            "cd" => Some(Self::Cd),
            "exit" => Some(Self::Exit),
            "which" => Some(Self::Which),
            _ => None,
        }
    }

    pub fn run(
        &self,
        shell: &mut Shell,
        io: &mut IOContext,
        args: &[String],
    ) -> Result<ExitCode, String> {
        macro_rules! run_builtin {
            ($mod:ty) => {
                Ok(<$mod>::build(args)?.run(shell, io))
            };
        }

        match self {
            Self::Cd => run_builtin!(cd::Cd),
            Self::Exit => run_builtin!(exit::Exit),
            Self::Which => run_builtin!(which::Which),
        }
    }
}
