use std::env;
use std::io::{self, Write};

use crate::call::Call;
use crate::system::dirs;

pub struct Shell {
    last_result: Result<(), String>,
    should_exit: bool,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            last_result: Ok(()),
            should_exit: false,
        }
    }

    pub fn main_loop(&mut self) {
        while !self.should_exit {
            self.print_prompt();
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
        let parts = input.trim().split(" | ");

        for part in parts {
            if part.is_empty() {
                continue;
            }

            let call = Call::parse(part);

            if call == Call::Exit {
                self.should_exit = true;
                return Ok(());
            } else {
                call.execute()?;
            }
        }

        Ok(())
    }

    fn print_prompt(&self) {
        let pwd_buf = env::current_dir().unwrap_or_default();
        let pwd = pwd_buf.as_os_str().to_str().unwrap_or_default();
        let home = dirs::home();

        let path = if pwd.starts_with(&home) {
            pwd.replacen(&home, "~", 1)
        } else {
            pwd.into()
        };

        const PATH_DECORATION: &str = "\x1b[2m";
        const PROMPT_DECORATION: &str = "\x1b[1m";
        const COLOUR_SUCCESS: &str = "\x1b[32m";
        const COLOUR_FAIL: &str = "\x1b[31m";
        const PROMPT: &str = "$";

        let prompt_colour = if self.last_result.is_ok() {
            COLOUR_SUCCESS
        } else {
            COLOUR_FAIL
        };

        print!(
            "{}{}\x1b[m {}{}{}\x1b[m ",
            PATH_DECORATION, path, PROMPT_DECORATION, prompt_colour, PROMPT
        );
        io::stdout().flush().unwrap();
    }
}
