use std::{io, process};

use sysexits::ExitCode;

use crate::{parse, Builtin, Command, Expansion, IOContext, Shell, Spanned};

impl Shell {
    pub fn interpret(&mut self, input: &str) -> ExitCode {
        self.exit_code = match parse(input) {
            Ok(ast) => {
                // self.io.println(format!("{ast:#?}\n"));
                self.execute(None, &ast)
            }
            Err(e) => {
                self.io.eprintln(e);
                ExitCode::DataErr
            }
        };

        self.exit_code
    }

    fn execute(&mut self, ctx: Option<IOContext>, ast: &Spanned<Command>) -> ExitCode {
        if self.should_exit {
            return self.exit_code;
        }

        match ast {
            (Command::Simple((args, _)), _) => self.execute_simple(ctx, args),
            (Command::And(left, right), _) => self.execute_logical(ctx, true, left, right),
            (Command::Or(left, right), _) => self.execute_logical(ctx, false, left, right),
            (Command::Pipeline((cmds, _)), _) => self.execute_pipeline(ctx, cmds),
            (Command::List((cmds, _)), _) => self.execute_list(ctx, cmds),
        }
    }

    fn execute_simple(&mut self, ctx: Option<IOContext>, args: &[Expansion]) -> ExitCode {
        if self.should_exit {
            return self.exit_code;
        }

        let mut io = match ctx {
            Some(ctx) => ctx,
            None => self.io.clone(),
        };

        let expanded_args = args.iter().map(|arg| arg.expand(self)).collect::<Vec<_>>();
        let keyword = &expanded_args[0];
        let args = &expanded_args[1..expanded_args.len()];

        if let Some(builtin) = Builtin::get(keyword) {
            match builtin.run(self, &mut io, args) {
                Ok(code) => code,
                Err(e) => {
                    io.eprintln(e);
                    ExitCode::Usage
                }
            }
        } else if self.find_on_path(keyword).is_some() {
            let mut cmd = process::Command::new(keyword);

            let child = cmd
                .stdin(io.input.clone())
                .stdout(io.output.clone())
                .stderr(io.error.clone())
                .args(args)
                .spawn();

            drop(io);
            drop(cmd);

            match child {
                Ok(mut child) => match child.wait() {
                    Ok(status) => status
                        .code()
                        .unwrap_or(0)
                        .try_into()
                        .unwrap_or(ExitCode::Ok),
                    Err(e) => {
                        self.io.eprintln(format!("crsh: {e}"));
                        ExitCode::OsErr
                    }
                },
                Err(e) => {
                    self.io.eprintln(format!("crsh: {e}"));
                    ExitCode::OsErr
                }
            }
        } else {
            self.io
                .eprintln(format!("crsh: command not found: {keyword}"));
            ExitCode::Unavailable
        }
    }

    fn execute_logical(
        &mut self,
        ctx: Option<IOContext>,
        and: bool,
        left: &Spanned<Command>,
        right: &Spanned<Command>,
    ) -> ExitCode {
        if self.should_exit {
            return self.exit_code;
        }

        let left_result = self.execute(ctx.clone(), left);

        if (left_result == ExitCode::Ok) == and {
            self.execute(ctx, right)
        } else {
            left_result
        }
    }

    fn execute_pipeline(&mut self, ctx: Option<IOContext>, cmds: &[Spanned<Command>]) -> ExitCode {
        if self.should_exit {
            return self.exit_code;
        }

        let io = match ctx {
            Some(ctx) => ctx,
            None => self.io.clone(),
        };

        let mut pipes = Vec::new();

        let mut results = cmds.iter().map(|cmd| {
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

            Ok::<ExitCode, io::Error>(self.execute(Some(new_ctx), cmd))
        });

        match results.next_back() {
            Some(Ok(code)) => code,
            Some(Err(e)) => {
                self.io.eprintln(format!("crsh: {e}"));
                ExitCode::IoErr
            }
            None => ExitCode::Ok,
        }
    }

    fn execute_list(&mut self, ctx: Option<IOContext>, cmds: &[Spanned<Command>]) -> ExitCode {
        if self.should_exit {
            return self.exit_code;
        }

        cmds.iter()
            .map(|cmd| self.execute(ctx.clone(), cmd))
            .next_back()
            .unwrap_or(ExitCode::Ok)
    }
}
