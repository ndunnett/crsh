mod cd;
mod echo;
mod launch;

pub enum Call<'a> {
    Cd(Vec<&'a str>),
    Echo(Vec<&'a str>),
    Exit,
    Launch(&'a str, Vec<&'a str>),
}

impl<'a> Call<'a> {
    pub fn parse(s: &'a str) -> Self {
        let mut parts = s.split_whitespace();

        match parts.next().unwrap_or_default() {
            "cd" => Self::Cd(parts.collect()),
            "echo" => Self::Echo(parts.collect()),
            "exit" => Self::Exit,
            keyword => Self::Launch(keyword, parts.collect()),
        }
    }

    pub fn execute(&self) -> Result<(), ()> {
        match self {
            Call::Cd(args) => cd::cd(args),
            Call::Echo(args) => echo::echo(args),
            Call::Launch(keyword, args) => launch::launch(keyword, args),
            Call::Exit => {
                eprintln!("error: failed to exit");
                Err(())
            }
        }
    }
}
