mod builtin;
mod config;
mod error;
mod parsing;
mod shell;

pub use error::Result;
pub use shell::Shell;
pub use parsing::ParsingIterator;
