use std::env;
use std::io::{stdin, stdout, Write};

use crate::builtin::*;
use crate::system::dirs;
use crate::system::process;

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
                        dir.to_string()
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
        stdout().flush().unwrap();
    }

    fn main_loop(&mut self) {
        while !self.should_exit {
            self.print_prefix();
            let mut input = String::new();

            match stdin().read_line(&mut input) {
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
