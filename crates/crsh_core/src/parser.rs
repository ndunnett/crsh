use chumsky::prelude::*;

use crate::format_errors;

mod ast;
mod expansion;
mod lex;

use ast::parser;
pub use ast::Command;
pub use expansion::Expansion;
use lex::lexer;

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

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
