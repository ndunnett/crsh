use std::io::Read;

use sysexits::ExitCode;

use lib_os::{dir, io};

use crate::{
    builtin::Builtin,
    parsing::{Command, Node, Parameter, Redirection, Word},
    Result, Shell,
};

impl Shell {
    pub fn execute(&mut self, ctx: Option<io::Context>, node: &Node) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        match node {
            Node::Command {
                command: Command { name, args },
            } => self.command(ctx, name, args),
            Node::List { nodes } => self.list(ctx, nodes),
            Node::Pipeline { nodes } => self.pipeline(ctx, nodes),
            Node::Or { left, right } => self.logical(ctx, false, left, right),
            Node::And { left, right } => self.logical(ctx, true, left, right),
            Node::Subshell { node } => self.execute(ctx, node),
            Node::Redirection { redirections, node } => self.redirection(ctx, redirections, node),
            _ => todo!(),
        }
    }

    fn word(&mut self, word: &Word) -> Result<String> {
        match word {
            Word::String(s) => Ok(s.to_string()),
            Word::Parameter(p) => self.parameter(p),
            Word::Command { node } => {
                let (mut reader, writer) = io::pipe()?;

                let ctx = io::Context {
                    input: self.io.input.try_clone()?,
                    output: writer.into(),
                    error: self.io.error.try_clone()?,
                };

                self.execute(Some(ctx), node)?;
                let mut output = String::new();
                reader.read_to_string(&mut output)?;
                Ok(output)
            }
            Word::Compound { words } => {
                let strings = words
                    .iter()
                    .map(|w| self.word(w))
                    .collect::<Result<Vec<_>>>()?;

                Ok(strings.join(""))
            }
        }
    }

    fn parameter(&mut self, p: &Parameter) -> Result<String> {
        Ok(match p {
            Parameter::String(s) => std::env::var(s).unwrap_or_default(),
            Parameter::Number(n) => self.args.get(*n).cloned().unwrap_or_default(),
            Parameter::OtherHome(user) => dir::home(&self.word(user)?),
            Parameter::MyHome => dir::my_home(),
        })
    }

    fn redirection(
        &mut self,
        ctx: Option<io::Context>,
        _redirections: &[Redirection],
        node: &Node,
    ) -> Result<ExitCode> {
        // todo: implement redirection
        self.execute(ctx, node)
    }

    fn command(
        &mut self,
        ctx: Option<io::Context>,
        name: &Word,
        args: &[Word],
    ) -> Result<ExitCode> {
        if self.should_exit {
            return Ok(self.exit_code);
        }

        let name = self.word(name)?;

        let args_owned = args
            .iter()
            .map(|arg| self.word(arg))
            .collect::<Result<Vec<_>>>()?;

        let args = args_owned
            .iter()
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();

        let mut io = match ctx {
            Some(ctx) => ctx,
            None => self.io.try_clone()?,
        };

        if let Some(builtin) = Builtin::get(&name) {
            Ok(builtin.run(self, &mut io, &args))
        } else if let Some(path) = dir::find_on_path(&name) {
            let mut cmd = std::process::Command::new(path);

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
            self.io.eprintln(format!("crsh: command not found: {name}"));
            Ok(ExitCode::Unavailable)
        }
    }

    fn list(&mut self, ctx: Option<io::Context>, nodes: &[Node]) -> Result<ExitCode> {
        let mut exit_code = ExitCode::Ok;

        if let Some(ref ctx) = ctx {
            for node in nodes {
                exit_code = self.execute(ctx.try_clone().ok(), node)?;
            }
        } else {
            for node in nodes {
                exit_code = self.execute(None, node)?;
            }
        }

        Ok(exit_code)
    }

    fn pipeline(&mut self, ctx: Option<io::Context>, nodes: &[Node]) -> Result<ExitCode> {
        match nodes.len() {
            0 => Ok(ExitCode::Ok),
            1 => self.execute(ctx, &nodes[0]),
            len => {
                let mut pipes = Vec::new();

                let io = match ctx {
                    Some(ctx) => ctx,
                    None => self.io.try_clone()?,
                };

                let first_ctx = {
                    let (reader, writer) = io::pipe()?;
                    pipes.push((reader.try_clone()?, writer.try_clone()?));

                    io::Context {
                        input: io.input.try_clone()?,
                        output: writer.into(),
                        error: io.error.try_clone()?,
                    }
                };

                _ = self.execute(Some(first_ctx), &nodes[0])?;

                for node in &nodes[1..len - 1] {
                    let new_ctx = {
                        let (last_reader, _) = pipes.pop().unwrap();
                        let (reader, writer) = io::pipe()?;
                        pipes.push((reader.try_clone()?, writer.try_clone()?));

                        io::Context {
                            input: last_reader.into(),
                            output: writer.into(),
                            error: io.error.try_clone()?,
                        }
                    };

                    _ = self.execute(Some(new_ctx), node)?;
                }

                let last_ctx = {
                    let (reader, _) = pipes.pop().unwrap();

                    io::Context {
                        input: reader.into(),
                        output: io.output.try_clone()?,
                        error: io.error.try_clone()?,
                    }
                };

                self.execute(Some(last_ctx), &nodes[len - 1])
            }
        }
    }

    fn logical(
        &mut self,
        ctx: Option<io::Context>,
        and: bool,
        left: &Node,
        right: &Node,
    ) -> Result<ExitCode> {
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
}
