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

            let report_result = Report::build(ReportKind::Error, filename, e.span().start)
                .with_label(
                    Label::new((filename, e.span().into_range()))
                        .with_message(e.to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((filename, span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .write_for_stdout(sources([(filename, input)]), &mut buffer);

            match (report_result, String::from_utf8(buffer)) {
                (Ok(_), Ok(msg)) => Some(msg.trim().to_string()),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
