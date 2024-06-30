use std::{
    fmt,
    path::{Path, PathBuf},
};

use ariadne::{sources, Color, Label, Report, ReportKind};
use sysexits::ExitCode;

mod builtin;
mod common_env;
mod config;
mod execution;
mod parsing;
mod shell_io;

pub use common_env::*;
pub use config::*;
pub use parsing::parse;
pub use shell_io::*;

pub type Span = chumsky::prelude::SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: ExitCode,
    pub io: IOContext,
    pub config: ShellConfig,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            env: CommonEnv::default(),
            exit_code: ExitCode::Ok,
            io: IOContext::default(),
            config: ShellConfig::default(),
        }
    }
}

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

    pub fn find_on_path<P: AsRef<Path>>(&self, keyword: P) -> Option<PathBuf> {
        self.env
            .path
            .iter()
            .filter_map(|dir| {
                let path = dir.join(&keyword);

                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn config_filepath<S: AsRef<Path>>(&self, filename: S) -> PathBuf {
        let mut path = self.env.config.clone();
        path.push(filename);
        path
    }
}

pub fn format_errors<T: fmt::Display>(
    filename: &'static str,
    input: &str,
    errors: &[chumsky::prelude::Rich<'_, T>],
) -> String {
    errors
        .iter()
        .filter_map(|e| {
            let mut buffer = vec![];

            if Report::build(ReportKind::Error, filename, e.span().start)
                .with_message(e.to_string())
                .with_label(Label::new((filename, e.span().into_range())).with_color(Color::Red))
                .finish()
                .write_for_stdout(sources([(filename, input)]), &mut buffer)
                .is_ok()
            {
                String::from_utf8(buffer).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
