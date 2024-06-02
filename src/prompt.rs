use std::env;
use std::io::{self, Write};

use crate::system;

pub struct Prompt<'a> {
    path_decoration: &'a str,
    prompt_decoration: &'a str,
    colour_success: &'a str,
    colour_fail: &'a str,
    regular_prompt: &'a str,
    continue_prompt: &'a str,
}

impl Default for Prompt<'_> {
    fn default() -> Self {
        Self {
            path_decoration: "\x1b[2m",
            prompt_decoration: "\x1b[1m",
            colour_success: "\x1b[32m",
            colour_fail: "\x1b[31m",
            regular_prompt: "$",
            continue_prompt: ">",
        }
    }
}

impl<'a> Prompt<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_path(&self) -> String {
        let pwd_buf = env::current_dir().unwrap_or_default();
        let pwd = pwd_buf.as_os_str().to_str().unwrap_or_default();
        let home = system::home();

        if pwd.starts_with(&home) {
            pwd.replacen(&home, "~", 1)
        } else {
            pwd.into()
        }
    }

    pub fn print(&self, last_result: Result<(), ()>) {
        print!(
            "{}{}\x1b[m {}{}{}\x1b[m ",
            self.path_decoration,
            self.get_path(),
            self.prompt_decoration,
            if last_result.is_ok() {
                self.colour_success
            } else {
                self.colour_fail
            },
            self.regular_prompt
        );

        io::stdout().flush().unwrap();
    }

    pub fn print_continuation(&self) {
        print!("{}{}\x1b[m ", self.path_decoration, self.continue_prompt);
        io::stdout().flush().unwrap();
    }
}
