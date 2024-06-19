use std::io;
use std::process;

use super::parsing::Command;
use super::{IOContext, Shell};

impl Shell {
    pub fn execute(&mut self, ctx: Option<IOContext>, ast: &Command) -> i32 {
        match ast {
            Command::Empty => 0,
            Command::Simple { keyword, args } => self.execute_simple(ctx, keyword, args),
            Command::Logical { and, left, right } => self.execute_logical(ctx, *and, left, right),
            Command::Pipeline { cmds } => self.execute_pipeline(ctx, cmds),
            Command::List { cmds } => self.execute_list(ctx, cmds),
            // _ => {
            //     sh.eprintln("crsh: unimplemented functionality");
            //     -1
            // }
        }
    }

    fn execute_simple(&mut self, ctx: Option<IOContext>, keyword: &str, args: &[&str]) -> i32 {
        let mut io = match ctx {
            Some(ctx) => ctx,
            None => self.io.clone(),
        };

        if let Some(builder) = Self::get_builtin_builder(keyword) {
            match builder(args) {
                Ok(builtin) => builtin.run(self, &mut io),
                Err(e) => {
                    io.eprintln(e);
                    -1
                }
            }
        } else if self.find_on_path(keyword).is_some() {
            let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            let mut cmd = process::Command::new(keyword);

            let child = cmd
                .stdin(io.input.clone())
                .stdout(io.output.clone())
                .stderr(io.error.clone())
                .args(&args)
                .spawn();

            drop(io);
            drop(cmd);

            match child {
                Ok(mut child) => match child.wait() {
                    Ok(status) => status.code().unwrap_or(-1),
                    Err(e) => {
                        self.io.eprintln(format!("crsh: {e}"));
                        -1
                    }
                },
                Err(e) => {
                    self.io.eprintln(format!("crsh: {e}"));
                    -1
                }
            }
        } else {
            self.io
                .eprintln(format!("crsh: command not found: {keyword}"));
            -1
        }
    }

    fn execute_logical(
        &mut self,
        ctx: Option<IOContext>,
        and: bool,
        left: &Command,
        right: &Command,
    ) -> i32 {
        let left_result = self.execute(ctx.clone(), left);

        if (left_result == 0) == and {
            self.execute(ctx, right)
        } else {
            left_result
        }
    }

    fn execute_pipeline(&mut self, ctx: Option<IOContext>, cmds: &[Command]) -> i32 {
        let io = match ctx {
            Some(ctx) => ctx,
            None => self.io.clone(),
        };

        let mut pipes = Vec::new();

        let results = cmds.iter().map(|cmd| {
            let new_ctx = if Some(cmd) == cmds.first() {
                let (reader, writer) = os_pipe::pipe()?;
                pipes.push((reader.try_clone()?, writer.try_clone()?));

                IOContext {
                    input: io.input.clone(),
                    output: writer.into(),
                    error: io.error.clone(),
                }
            } else if Some(cmd) == cmds.last() {
                let (reader, _) = pipes.pop().unwrap();

                IOContext {
                    input: reader.into(),
                    output: io.output.clone(),
                    error: io.error.clone(),
                }
            } else {
                let (last_reader, _) = pipes.pop().unwrap();
                let (reader, writer) = os_pipe::pipe()?;
                pipes.push((reader.try_clone()?, writer.try_clone()?));

                IOContext {
                    input: last_reader.into(),
                    output: writer.into(),
                    error: io.error.clone(),
                }
            };

            Ok::<i32, io::Error>(self.execute(Some(new_ctx), cmd))
        });

        match results.last() {
            Some(Ok(code)) => code,
            Some(Err(e)) => {
                self.io.eprintln(format!("crsh: {e}"));
                -1
            }
            None => 0,
        }
    }

    fn execute_list(&mut self, ctx: Option<IOContext>, cmds: &[Command]) -> i32 {
        cmds.iter()
            .map(|cmd| self.execute(ctx.clone(), cmd))
            .last()
            .unwrap_or(0)
    }
}
