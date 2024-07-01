use std::fmt;

use ariadne::{sources, Color, Label, Report, ReportKind};

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
