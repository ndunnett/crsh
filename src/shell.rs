use std::env;
use std::io::{stdin, stdout, Write};

use crate::builtin::*;
use crate::system::dirs;
use crate::system::process::*;

type ShellResult = Result<bool, String>;

struct Shell {}

impl Shell {
    fn interpret(input: &str) -> ShellResult {
        let mut parts = input.split_whitespace();
        let command = parts.next();
        let args = parts.collect::<Vec<_>>();

        match command {
            Some("cd") => cd(&args),
            Some("exit") => Ok(true),
            Some(cmd) => Self::execute(cmd, &args),
            None => Ok(false),
        }
    }

    fn execute(command: &str, args: &[&str]) -> ShellResult {
        match fork() {
            Ok(ForkResult::Child) => {
                if let Err(e) = execvp(command, args) {
                    Err(e)
                } else {
                    Ok(false)
                }
            }
            Ok(ForkResult::Parent(pid)) => {
                if let Err(e) = waitpid(pid) {
                    Err(e)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn print_prefix() {
        if let Ok(pwd) = env::current_dir() {
            if let Some(dir) = pwd.as_os_str().to_str() {
                let home = dirs::home();
                print!(
                    "{} ",
                    if dir.starts_with(&home) {
                        dir.replacen(&home, "~", 1)
                    } else {
                        dir.to_string()
                    }
                );
            }
        }

        print!("$ ");
        stdout().flush().unwrap();
    }

    fn main_loop() {
        loop {
            Self::print_prefix();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            match Self::interpret(&input) {
                Ok(true) => return,
                Err(e) => eprintln!("Error: {}", e),
                _ => (),
            }
        }
    }
}

pub fn run() {
    Shell::main_loop()
}
