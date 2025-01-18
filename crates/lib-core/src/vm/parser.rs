use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::{input::SpannedInput, prelude::*};

use crate::{scanner, Error, Expansion, Result, Span, Spanned, Token};

type ParserInput<'a, 'b> = SpannedInput<Token<'b>, Span, &'a [Spanned<Token<'b>>]>;
type ParserError<'a, 'b> = extra::Err<Rich<'a, Token<'b>, Span>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Command<'a> {
    Simple(Spanned<Vec<Expansion<'a>>>),
    And(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Or(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Pipeline(Spanned<Vec<Spanned<Self>>>),
    List(Spanned<Vec<Spanned<Self>>>),
}

pub fn parser<'a, 'b: 'a>(
) -> impl Parser<'a, ParserInput<'a, 'b>, Spanned<Command<'b>>, ParserError<'a, 'b>> + Clone {
    recursive(|cmd| {
        let word = select! {
            Token::Unquoted(s) => Expansion::Unquoted(s),
            Token::DoubleQuoted(s) => Expansion::DoubleQuoted(s),
            Token::SingleQuoted(s) => Expansion::SingleQuoted(s),
        };

        let simple_cmd = word
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|c, e| (c, e.span()))
            .map(Command::Simple);

        let atom = simple_cmd.or(cmd
            .clone()
            .delimited_by(just(Token::Control('(')), just(Token::Control(')'))));

        let logical_and = atom.clone().foldl_with(
            just(Token::Operator("&&"))
                .ignore_then(atom.clone())
                .repeated(),
            |a, b, e| Command::And(Box::new((a, e.span())), Box::new((b, e.span()))),
        );

        let logical_or = logical_and.clone().foldl_with(
            just(Token::Operator("||"))
                .ignore_then(logical_and.clone())
                .repeated(),
            |a, b, e| Command::Or(Box::new((a, e.span())), Box::new((b, e.span()))),
        );

        let pipeline = logical_or
            .clone()
            .map_with(|c, e| (c, e.span()))
            .separated_by(just(Token::Operator("|")))
            .at_least(2)
            .collect::<Vec<_>>()
            .map_with(|c, e| (c, e.span()))
            .map(Command::Pipeline);

        let cmd_list = pipeline
            .clone()
            .map_with(|c, e| (c, e.span()))
            .separated_by(just(Token::Control(';')))
            .at_least(2)
            .allow_trailing()
            .collect::<Vec<_>>()
            .map_with(|c, e| (c, e.span()))
            .map(Command::List);

        let empty = any()
            .repeated()
            .at_most(0)
            .map(|_| Command::List((vec![], SimpleSpan::splat(0))));

        cmd_list
            .or(pipeline)
            .or(logical_or)
            .or(logical_and)
            .or(empty)
    })
    .map_with(|c, e| (c, e.span()))
}

pub fn format_errors<T: std::fmt::Display>(input: &str, errors: &[Rich<'_, T>]) -> Error {
    Error::Adhoc {
        string: errors
            .iter()
            .filter_map(|e| {
                let mut buffer = vec![];

                let report_result = Report::build(ReportKind::Error, ("", e.span().into_range()))
                    .with_label(
                        Label::new(("", e.span().into_range()))
                            .with_message(e.to_string())
                            .with_color(Color::Red),
                    )
                    .with_labels(e.contexts().map(|(label, span)| {
                        Label::new(("", span.into_range()))
                            .with_message(format!("while parsing this {}", label))
                            .with_color(Color::Yellow)
                    }))
                    .finish()
                    .write_for_stdout(sources([("", input)]), &mut buffer);

                match (report_result, String::from_utf8(buffer)) {
                    (Ok(_), Ok(msg)) => Some(msg.trim().to_string()),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn parse(input: &str) -> Result<Spanned<Command>> {
    match scanner().parse(input).into_result() {
        Ok(tokens) => {
            match parser()
                .parse(tokens.as_slice().spanned((input.len()..input.len()).into()))
                .into_result()
            {
                Ok(ast) => Ok(ast),
                Err(e) => Err(format_errors(input, &e)),
            }
        }
        Err(e) => Err(format_errors(input, &e)),
    }
}
