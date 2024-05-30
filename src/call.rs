use crate::builtin;
use crate::system::process;

#[derive(PartialEq)]
pub enum Call<'a> {
    Cd(Vec<&'a str>),
    Echo(Vec<&'a str>),
    Exec(Vec<&'a str>),
    Exit,
}

impl<'a> Call<'a> {
    pub fn parse(s: &'a str) -> Self {
        let args: Vec<&'a str> = s.split_whitespace().collect();

        match args[0] {
            "cd" => Self::Cd(args[1..].into()),
            "echo" => Self::Echo(args[1..].into()),
            "exec" => Self::Exec(args[1..].into()),
            "exit" => Self::Exit,
            _ => Self::Exec(args),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        match self {
            Call::Cd(args) => builtin::cd(args),
            Call::Echo(args) => builtin::echo(args),
            Call::Exec(args) => process::execute(args),
            Call::Exit => Err("failed to exit".into()),
        }
    }
}
