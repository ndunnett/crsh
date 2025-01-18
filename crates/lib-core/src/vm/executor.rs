use sysexits::ExitCode;

use crate::{parse, Builtin, Command, Expansion, IOContext, Result, Shell, Spanned};

impl Shell {
    pub fn interpret(&mut self, input: &str) -> ExitCode {
        let ast = match parse(input) {
            Ok(ast) => ast,
            Err(e) => {
                self.io.eprintln(format!("crsh: parsing error: {e:#?}"));
                return ExitCode::DataErr;
            }
        };

        self.exit_code = match self.execute(None, &ast) {
            Ok(code) => code,
            Err(e) => {
                self.io.eprintln(format!("crsh: interpreter error: {e:#?}"));
                return ExitCode::DataErr;
            }
        };

        self.exit_code
    }

    fn execute(&mut self, ctx: Option<IOContext>, ast: &Spanned<Command>) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        match ast {
            (Command::Simple((args, _)), _) => self.execute_simple(ctx, args),
            (Command::And(left, right), _) => self.execute_logical(ctx, true, left, right),
            (Command::Or(left, right), _) => self.execute_logical(ctx, false, left, right),
            (Command::Pipeline((cmds, _)), _) => self.execute_pipeline(ctx, cmds),
            (Command::List((cmds, _)), _) => self.execute_list(ctx, cmds),
        }
    }

    fn execute_simple(&mut self, ctx: Option<IOContext>, args: &[Expansion]) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        let mut io = match ctx {
            Some(ctx) => ctx,
            None => self.io.try_clone()?,
        };

        let keyword = &args[0].expand();
        let args = args
            .iter()
            .skip(1)
            .map(|arg| arg.expand())
            .collect::<Vec<_>>();

        if let Some(builtin) = Builtin::get(keyword) {
            match builtin.run(self, &mut io, &args) {
                Ok(code) => Ok(code),
                Err(e) => {
                    io.eprintln(e);
                    Ok(ExitCode::Usage)
                }
            }
        } else if self.find_on_path(keyword).is_some() {
            let mut cmd = std::process::Command::new(keyword);

            let mut child = cmd
                .stdin(io.input.try_clone()?)
                .stdout(io.output.try_clone()?)
                .stderr(io.error.try_clone()?)
                .args(args)
                .spawn()?;

            drop(io);
            drop(cmd);

            Ok(child
                .wait()?
                .code()
                .unwrap_or(0)
                .try_into()
                .unwrap_or_default())
        } else {
            self.io
                .eprintln(format!("crsh: command not found: {keyword}"));
            Ok(ExitCode::Unavailable)
        }
    }

    fn execute_logical(
        &mut self,
        ctx: Option<IOContext>,
        and: bool,
        left: &Spanned<Command>,
        right: &Spanned<Command>,
    ) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        let left_result = if let Some(ref ctx) = ctx {
            self.execute(Some(ctx.try_clone()?), left)?
        } else {
            self.execute(None, left)?
        };

        if (left_result == ExitCode::Ok) == and {
            self.execute(ctx, right)
        } else {
            Ok(left_result)
        }
    }

    fn execute_pipeline(
        &mut self,
        ctx: Option<IOContext>,
        cmds: &[Spanned<Command>],
    ) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        let io = match ctx {
            Some(ctx) => ctx,
            None => self.io.try_clone()?,
        };

        let mut pipes = Vec::new();
        let mut exit_code = ExitCode::Ok;

        for cmd in cmds {
            let new_ctx = if Some(cmd) == cmds.first() {
                let (reader, writer) = os_pipe::pipe()?;
                pipes.push((reader.try_clone()?, writer.try_clone()?));

                IOContext {
                    input: io.input.try_clone()?,
                    output: writer.into(),
                    error: io.error.try_clone()?,
                }
            } else if Some(cmd) == cmds.last() {
                let (reader, _) = pipes.pop().unwrap();

                IOContext {
                    input: reader.into(),
                    output: io.output.try_clone()?,
                    error: io.error.try_clone()?,
                }
            } else {
                let (last_reader, _) = pipes.pop().unwrap();
                let (reader, writer) = os_pipe::pipe()?;
                pipes.push((reader.try_clone()?, writer.try_clone()?));

                IOContext {
                    input: last_reader.into(),
                    output: writer.into(),
                    error: io.error.try_clone()?,
                }
            };

            exit_code = self.execute(Some(new_ctx), cmd)?;
        }

        Ok(exit_code)
    }

    fn execute_list(
        &mut self,
        ctx: Option<IOContext>,
        cmds: &[Spanned<Command>],
    ) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        let mut exit_code = ExitCode::Ok;

        for cmd in cmds {
            exit_code = if let Some(ref ctx) = ctx {
                self.execute(Some(ctx.try_clone()?), cmd)?
            } else {
                self.execute(None, cmd)?
            };
        }

        Ok(exit_code)
    }
}
