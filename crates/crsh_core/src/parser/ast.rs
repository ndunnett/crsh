use chumsky::{input::SpannedInput, prelude::*};

use super::lex::Token;
use crate::{Expansion, Span, Spanned};

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
