use std::env;
use std::io::{self, Write};
use std::process::exit;

use crate::interpreter;
use crate::system;

struct PromptStyle<'a> {
    path_decoration: &'a str,
    prompt_decoration: &'a str,
    colour_success: &'a str,
    colour_fail: &'a str,
    regular_prompt: &'a str,
    continue_prompt: &'a str,
}

impl Default for PromptStyle<'_> {
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

struct PromptContext {
    current_path: String,
    last_result: Result<(), ()>,
}

impl Default for PromptContext {
    fn default() -> Self {
        Self {
            current_path: Self::get_path(),
            last_result: Ok(()),
        }
    }
}

impl PromptContext {
    fn get_path() -> String {
        let pwd_buf = env::current_dir().unwrap_or_default();
        let pwd = pwd_buf.as_os_str().to_str().unwrap_or_default();
        let home = system::home();

        if pwd.starts_with(&home) {
            pwd.replacen(&home, "~", 1)
        } else {
            pwd.into()
        }
    }

    fn update_path(&mut self) {
        self.current_path = Self::get_path();
    }
}

#[derive(Default)]
pub struct Prompt<'a> {
    style: PromptStyle<'a>,
    ctx: PromptContext,
}

impl<'a> Prompt<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interactive_loop(&mut self) {
        self.ctx.last_result = Ok(());

        loop {
            self.ctx.update_path();
            self.print_ps1();
            let mut input = String::new();
            let read_status;

            loop {
                match io::stdin().read_line(&mut input) {
                    Ok(0) => {
                        println!();
                        exit(0);
                    }
                    Ok(_) => {
                        if input.trim_end().ends_with('\\') {
                            input = input
                                .trim_end()
                                .strip_suffix('\\')
                                .unwrap_or(&input)
                                .to_string();

                            self.print_ps2();
                            continue;
                        } else {
                            read_status = Ok(());
                            break;
                        }
                    }
                    Err(e) => {
                        read_status = Err(e.to_string());
                        break;
                    }
                }
            }

            self.ctx.last_result = match read_status {
                Ok(_) => interpreter::execute(&input),
                Err(e) => {
                    eprintln!("crsh: {e}");
                    Err(())
                }
            };
        }
    }

    pub fn print_ps1(&self) {
        print!(
            "{}{}\x1b[m {}{}{}\x1b[m ",
            self.style.path_decoration,
            self.ctx.current_path,
            self.style.prompt_decoration,
            if self.ctx.last_result.is_ok() {
                self.style.colour_success
            } else {
                self.style.colour_fail
            },
            self.style.regular_prompt
        );

        io::stdout().flush().unwrap();
    }

    pub fn print_ps2(&self) {
        print!(
            "{}{}\x1b[m ",
            self.style.path_decoration, self.style.continue_prompt
        );
        io::stdout().flush().unwrap();
    }
}
