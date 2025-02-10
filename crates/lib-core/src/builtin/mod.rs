use sysexits::ExitCode;

use lib_os::io;

use crate::Shell;

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

    pub fn run(&self, shell: &mut Shell, io: &mut io::Context, args: &[&str]) -> ExitCode {
        let f = match self {
            Self::Cd => Self::cd,
            Self::Exit => Self::exit,
            Self::Which => Self::which,
        };

        f(shell, io, args)
    }
}
