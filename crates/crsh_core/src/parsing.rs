use std::fmt;

use chumsky::input::SpannedInput;
use chumsky::prelude::*;

use super::*;

type ParserInput<'a, 'b> = SpannedInput<Token<'b>, Span, &'a [Spanned<Token<'b>>]>;
type LexerError<'a> = extra::Err<Rich<'a, char, Span>>;
type ParserError<'a, 'b> = extra::Err<Rich<'a, Token<'b>, Span>>;

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

fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<Spanned<Token<'a>>>, LexerError<'a>> {
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
    Simple(Spanned<Vec<&'a str>>),
    And(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Or(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Pipeline(Vec<Spanned<Self>>),
    List(Vec<Spanned<Self>>),
}

fn parser<'a, 'b: 'a>(
) -> impl Parser<'a, ParserInput<'a, 'b>, Spanned<Command<'b>>, ParserError<'a, 'b>> + Clone {
    let word = select! {
        Token::Identifier(s) => s,
    }
    .labelled("word");

    let simple_cmd = word
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|c, e| (c, e.span()))
        .map(Command::Simple)
        .labelled("simple command");

    let logical_and = simple_cmd
        .foldl_with(
            just(Token::Operator("&&"))
                .ignore_then(simple_cmd)
                .repeated(),
            |a, b, e| Command::And(Box::new((a, e.span())), Box::new((b, e.span()))),
        )
        .labelled("logical and expression");

    let logical_or = logical_and
        .clone()
        .foldl_with(
            just(Token::Operator("||"))
                .ignore_then(logical_and.clone())
                .repeated(),
            |a, b, e| Command::Or(Box::new((a, e.span())), Box::new((b, e.span()))),
        )
        .labelled("logical or expression");

    let pipeline = logical_or
        .clone()
        .map_with(|c, e| (c, e.span()))
        .separated_by(just(Token::Operator("|")))
        .at_least(2)
        .collect::<Vec<_>>()
        .map(Command::Pipeline)
        .labelled("command pipeline");

    let cmd_list = pipeline
        .clone()
        .map_with(|c, e| (c, e.span()))
        .separated_by(just(Token::Control(';')))
        .at_least(2)
        .allow_trailing()
        .collect::<Vec<_>>()
        .map(Command::List)
        .labelled("command list");

    let empty = any().repeated().at_most(0).map(|_| Command::List(vec![]));

    cmd_list
        .or(pipeline)
        .or(logical_or)
        .or(logical_and)
        .or(empty)
        .map_with(|c, e| (c, e.span()))
}

pub fn parse(input: &str) -> Result<Spanned<Command>, String> {
    match lexer().parse(input).into_result() {
        Ok(tokens) => {
            match parser()
                .parse(tokens.as_slice().spanned((input.len()..input.len()).into()))
                .into_result()
            {
                Ok(ast) => Ok(ast),
                Err(e) => Err(format_errors("", input, &e)),
            }
        }
        Err(e) => Err(format_errors("", input, &e)),
    }
}
