use std::process;

use super::parsing::Command;
use super::{IOContext, Shell};

impl Shell {
    pub fn execute(&mut self, io: &mut IOContext, ast: &Command) -> i32 {
        match ast {
            Command::Empty => 0,
            Command::Simple { keyword, args } => self.execute_simple(io, keyword, args),
            Command::And { left, right } => self.execute_and(io, left, right),
            Command::Or { left, right } => self.execute_or(io, left, right),
            Command::Pipeline { left, right } => self.execute_pipeline(io, left, right),
            // _ => {
            //     sh.eprintln("crsh: unimplemented functionality");
            //     -1
            // }
        }
    }

    fn execute_simple(&mut self, io: &mut IOContext, keyword: &str, args: &[&str]) -> i32 {
        if let Some(builder) = Self::get_builtin_builder(keyword) {
            match builder(args) {
                Ok(builtin) => builtin.run(self, io),
                Err(e) => {
                    io.eprintln(e);
                    -1
                }
            }
        } else if self.find_on_path(keyword).is_some() {
            let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let mut cmd = process::Command::new(keyword);

            match cmd
                .stdin(io.input.clone())
                .stdout(io.output.clone())
                .stderr(io.error.clone())
                .args(&args)
                .spawn()
            {
                Ok(mut c) => match c.wait() {
                    Ok(status) => status.code().unwrap_or(-1),
                    Err(e) => {
                        io.eprintln(format!("crsh: {e}"));
                        -1
                    }
                },
                Err(e) => {
                    io.eprintln(format!("crsh: {e}"));
                    -1
                }
            }
        } else {
            io.eprintln(format!("crsh: command not found: {keyword}"));
            -1
        }
    }

    fn execute_and(&mut self, io: &mut IOContext, left: &Command, right: &Command) -> i32 {
        let left_result = self.execute(io, left);

        if left_result == 0 {
            self.execute(io, right)
        } else {
            left_result
        }
    }

    fn execute_or(&mut self, io: &mut IOContext, left: &Command, right: &Command) -> i32 {
        let left_result = self.execute(io, left);

        if left_result != 0 {
            self.execute(io, right)
        } else {
            left_result
        }
    }

    fn execute_pipeline(&mut self, io: &mut IOContext, left: &Command, right: &Command) -> i32 {
        let mut left_io = IOContext {
            input: io.input.clone(),
            output: io.output.clone(),
            error: io.error.clone(),
        };

        let mut right_io = IOContext {
            input: io.input.clone(),
            output: io.output.clone(),
            error: io.error.clone(),
        };

        let _ = self.execute(&mut left_io, left);
        self.execute(&mut right_io, right)
    }
}
