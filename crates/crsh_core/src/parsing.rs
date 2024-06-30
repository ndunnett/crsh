use std::fmt;

use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::{input::SpannedInput, prelude::*};

type Span = SimpleSpan<usize>;
type Spanned<T> = (T, Span);
type ParserInput<'a, 'b> = SpannedInput<Token<'b>, Span, &'a [Spanned<Token<'b>>]>;
type ParserError<'a> = extra::Err<Rich<'a, char, Span>>;

#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
    Operator(&'a str),
    Control(char),
    Identifier(&'a str),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Operator(s) => write!(f, "{}", s),
            Token::Control(c) => write!(f, "{}", c),
            Token::Identifier(s) => write!(f, "{}", s),
        }
    }
}

fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<Spanned<Token<'a>>>, ParserError<'a>> {
    let ident = text::ascii::ident()
        .map(Token::Identifier)
        .labelled("identifier");

    let filename = any()
        .and_is(one_of("{}<>/\\|\":*?").not())
        .and_is(one_of(" \n\r").not())
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Identifier)
        .labelled("filename");

    let path = filename
        .separated_by(just('/'))
        .allow_leading()
        .allow_trailing()
        .at_least(1)
        .to_slice()
        .map(Token::Identifier)
        .labelled("path");

    let double_quoted = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .to_slice()
        .map(Token::Identifier)
        .labelled("double quoted");

    let single_quoted = just('\'')
        .ignore_then(none_of('\'').repeated())
        .then_ignore(just('\''))
        .to_slice()
        .map(Token::Identifier)
        .labelled("single quoted");

    let op = one_of("&|")
        .repeated()
        .at_least(1)
        .at_most(2)
        .to_slice()
        .map(Token::Operator)
        .labelled("operator");

    let ctrl = one_of("()[]{};,")
        .map(Token::Control)
        .labelled("control character");

    let token = ctrl
        .or(op)
        .or(path)
        .or(double_quoted)
        .or(single_quoted)
        .or(ident);

    let comment = just("#")
        .then(any().and_is(text::newline().not()).repeated())
        .ignored();

    token
        .padded_by(comment.repeated())
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .map_with(|tok, e| (tok, e.span()))
        .repeated()
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command<'a> {
    Simple(Vec<&'a str>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Pipeline(Vec<Self>),
    List(Vec<Self>),
}

fn parser<'a, 'b: 'a>() -> impl Parser<'a, ParserInput<'a, 'b>, Command<'b>> + Clone {
    recursive(|cmd| {
        let word = select! {
            Token::Identifier(s) => s,
        };

        let simple_cmd = word
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map(Command::Simple);

        let atom = cmd
            .clone()
            .delimited_by(just(Token::Control('(')), just(Token::Control(')')))
            .or(simple_cmd);

        let logical_cmd = atom.clone().foldl(
            choice((
                just(Token::Operator("&&")).to(Command::And as fn(_, _) -> _),
                just(Token::Operator("||")).to(Command::Or as fn(_, _) -> _),
            ))
            .then(atom.clone())
            .repeated(),
            |a, (op, b)| op(Box::new(a), Box::new(b)),
        );

        let pipeline = logical_cmd
            .clone()
            .separated_by(just(Token::Operator("|")))
            .at_least(2)
            .collect::<Vec<_>>()
            .map(Command::Pipeline);

        let cmd_list = pipeline
            .clone()
            .separated_by(just(Token::Control(';')))
            .at_least(2)
            .allow_trailing()
            .collect::<Vec<_>>()
            .map(Command::List);

        let empty = any().repeated().at_most(0).map(|_| Command::List(vec![]));

        cmd_list.or(pipeline).or(logical_cmd).or(empty)
    })
}

pub fn parse(input: &str) -> Result<Command, String> {
    let mut lex_errors = vec![];
    let mut parse_errors = vec![];

    match lexer().parse(input).into_result() {
        Ok(tokens) => match parser()
            .parse(tokens.as_slice().spanned((input.len()..input.len()).into()))
            .into_result()
        {
            Ok(ast) => return Ok(ast),
            Err(e) => parse_errors.extend(e),
        },
        Err(e) => lex_errors.extend(e),
    }

    let mut output = vec![];

    if !lex_errors.is_empty() {
        let filename = "";
        let mut buffer = vec![];

        lex_errors
            .clone()
            .into_iter()
            .map(|e| e.map_token(|c| c.to_string()))
            .for_each(|e| {
                let _ = Report::build(ReportKind::Error, filename, e.span().start)
                    .with_label(
                        Label::new((filename, e.span().into_range()))
                            .with_message(e.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .with_labels(e.contexts().map(|(label, span)| {
                        Label::new((filename, span.into_range()))
                            .with_message(format!("while parsing this {}", label))
                            .with_color(Color::Yellow)
                    }))
                    .finish()
                    .write_for_stdout(sources([(filename, input)]), &mut buffer);
            });

        output.push(String::from_utf8(buffer).unwrap_or_default());
    }

    if !parse_errors.is_empty() {
        for e in parse_errors {
            output.push(format!("{e:#?}"));
        }
    }

    Err(output.join("\n"))
}
