use std::io;
use std::process::exit;

use crate::interpreter::Interpreter;
use crate::prompt::Prompt;

pub struct Shell<'a> {
    prompt: Prompt<'a>,
    interpreter: Interpreter,
    last_result: Result<(), ()>,
}

impl<'a> Shell<'a> {
    pub fn new() -> Self {
        Self {
            prompt: Prompt::new(),
            interpreter: Interpreter::new(),
            last_result: Ok(()),
        }
    }

    pub fn interactive_loop(&mut self) {
        loop {
            self.prompt.print(self.last_result);
            self.last_result = match self.read_input() {
                Ok(input) => self.interpreter.execute(&input),
                Err(e) => {
                    eprintln!("crsh: {e}");
                    Err(())
                }
            };
        }
    }

    fn read_input(&self) -> Result<String, String> {
        let mut input = String::new();

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

                        self.prompt.print_continuation();
                        continue;
                    } else {
                        return Ok(input);
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
    }
}
