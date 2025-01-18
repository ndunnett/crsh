use std::fmt;

use chumsky::prelude::*;

use crate::{Span, Spanned};

type LexerError<'a> = extra::Err<Rich<'a, char, Span>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Operator(&'a str),
    Control(char),
    Unquoted(&'a str),
    DoubleQuoted(&'a str),
    SingleQuoted(&'a str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Operator(s) => write!(f, "{s}"),
            Token::Control(c) => write!(f, "{c}"),
            Token::Unquoted(s) => write!(f, "{s}"),
            Token::DoubleQuoted(s) => write!(f, "\"{s}\""),
            Token::SingleQuoted(s) => write!(f, "'{s}'"),
        }
    }
}

pub fn scanner<'a>() -> impl Parser<'a, &'a str, Vec<Spanned<Token<'a>>>, LexerError<'a>> {
    let op = one_of("&|$<>")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Operator);

    let ctrl = one_of("()[]{};,").map(Token::Control);

    let filename = none_of("()[]{};,<>/\\|\":*?")
        .filter(|c: &char| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .to_slice();

    let path = filename
        .separated_by(one_of("/\\"))
        .allow_leading()
        .allow_trailing()
        .at_least(1)
        .to_slice()
        .or(one_of("/\\").to_slice());

    let unquoted = path
        .or(filename)
        .or(text::ascii::ident())
        .map(Token::Unquoted);

    let double_quoted = none_of('"')
        .repeated()
        .to_slice()
        .delimited_by(just('"'), just('"'))
        .map(Token::DoubleQuoted);

    let single_quoted = none_of('\'')
        .repeated()
        .to_slice()
        .delimited_by(just('\''), just('\''))
        .map(Token::SingleQuoted);

    let expansion = unquoted.or(double_quoted).or(single_quoted);

    let token = expansion.or(op).or(ctrl);

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
