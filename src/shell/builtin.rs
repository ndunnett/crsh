use super::{IOContext, Shell};

trait ImplementedBuiltin {
    fn build(args: &[&str]) -> Result<impl ImplementedBuiltin, String>;
    fn run(&self, sh: &mut super::Shell, io: &mut super::IOContext) -> i32;
}

mod cd;
mod exit;
mod which;

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

    pub fn run(&self, sh: &mut Shell, io: &mut IOContext, args: &[&str]) -> Result<i32, String> {
        macro_rules! run_builtin {
            ($mod:ty) => {
                Ok(<$mod>::build(args)?.run(sh, io))
            };
        }

        match self {
            Self::Cd => run_builtin!(cd::Cd),
            Self::Exit => run_builtin!(exit::Exit),
            Self::Which => run_builtin!(which::Which),
        }
    }
}
