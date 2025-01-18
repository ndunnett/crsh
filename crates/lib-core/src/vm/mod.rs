mod builtin;
mod executor;
mod expansion;
mod parser;
mod scanner;

pub use builtin::*;
// pub use executor::*;
pub use expansion::*;
pub use parser::*;
pub use scanner::*;

pub type Span = chumsky::prelude::SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);
