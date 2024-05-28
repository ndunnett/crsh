use std::env;
use std::io::{stdin, stdout, Write};

use crate::builtin::*;
use crate::system::dirs;
use crate::system::process;

#[derive(PartialEq)]
enum LastStatus {
    Success,
    Error,
}

struct Shell {
    last_status: LastStatus,
    should_exit: bool,
}

impl Shell {
    fn new() -> Self {
        Self {
            last_status: LastStatus::Success,
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
                        dir.to_string()
                    }
                );
            }
        }

        let prompt_colour = if self.last_status == LastStatus::Success {
            "\x1b[32m"
        } else {
            "\x1b[31m"
        };

        print!("\x1b[1m{}$\x1b[m ", prompt_colour);
        stdout().flush().unwrap();
    }

    fn main_loop(&mut self) {
        while !self.should_exit {
            self.print_prefix();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if let Err(e) = self.interpret(&input) {
                self.last_status = LastStatus::Error;
                eprintln!("error: {}", e);
            } else {
                self.last_status = LastStatus::Success;
            }
        }
    }

    fn interpret(&mut self, input: &str) -> Result<(), String> {
        let args = input.split_whitespace().collect::<Vec<_>>();

        match args.first() {
            Some(&"exit") => {
                self.should_exit = true;
                Ok(())
            }
            Some(&"cd") => cd(&args[1..]),
            Some(_) => process::execute(&args),
            None => Ok(()),
        }
    }
}

pub fn run() {
    let mut shell = Shell::new();
    shell.main_loop()
}
