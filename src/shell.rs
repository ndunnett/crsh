use std::env;
use std::io::{self, Write};
use std::str::FromStr;

use crate::builtin::*;
use crate::system::dirs;

struct Shell {
    last_result: Result<(), String>,
    should_exit: bool,
}

impl Shell {
    fn new() -> Self {
        Self {
            last_result: Ok(()),
            should_exit: false,
        }
    }

    fn print_prefix(&self) {
        if let Ok(pwd) = env::current_dir() {
            if let Some(dir) = pwd.as_os_str().to_str() {
                let home = dirs::home();
                print!(
                    "\x1b[2m{}\x1b[m ",
                    if dir.starts_with(&home) {
                        dir.replacen(&home, "~", 1)
                    } else {
                        dir.into()
                    }
                );
            }
        }

        let prompt_colour = if self.last_result.is_ok() {
            "\x1b[32m"
        } else {
            "\x1b[31m"
        };

        print!("\x1b[1m{}$\x1b[m ", prompt_colour);
        io::stdout().flush().unwrap();
    }

    fn main_loop(&mut self) {
        while !self.should_exit {
            self.print_prefix();
            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!();
                    break;
                }
                Ok(_) => self.last_result = self.interpret(&input),
                Err(e) => self.last_result = Err(e.to_string()),
            }

            if let Err(e) = self.last_result.as_ref() {
                eprintln!("error: {}", e);
            }
        }
    }

    fn interpret(&mut self, input: &str) -> Result<(), String> {
        let commands = input.trim().split(" | ");

        for command in commands {
            let args = command.split_whitespace().collect::<Vec<_>>();

            if args.is_empty() {
                continue;
            }

            match Builtin::from_str(args[0]) {
                Ok(Builtin::Exit) => {
                    self.should_exit = true;
                    return Ok(());
                }
                Ok(builtin) => Command::new(builtin, &args[1..]),
                Err(_) => Command::new(Builtin::Exec, &args),
            }
            .execute()?;
        }

        Ok(())
    }
}

pub fn run() {
    let mut shell = Shell::new();
    shell.main_loop()
}
