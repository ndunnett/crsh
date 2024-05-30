use std::str::FromStr;

mod cd;
mod echo;

use crate::system::process;

pub enum Builtin {
    Cd,
    Echo,
    Exec,
    Exit,
}

impl FromStr for Builtin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cd" => Ok(Self::Cd),
            "echo" => Ok(Self::Echo),
            "exec" => Ok(Self::Exec),
            "exit" => Ok(Self::Exit),
            _ => Err(()),
        }
    }
}

pub struct Command<'a> {
    builtin: Builtin,
    args: &'a [&'a str],
}

impl<'a> Command<'a> {
    pub fn new(builtin: Builtin, args: &'a [&'a str]) -> Self {
        Self { builtin, args }
    }

    pub fn execute(&self) -> Result<(), String> {
        let func = match self.builtin {
            Builtin::Cd => cd::cd,
            Builtin::Echo => echo::echo,
            Builtin::Exec => process::execute,
            Builtin::Exit => return Err("failed to exit".into()),
        };

        func(self.args)
    }
}
