mod builtin;
mod config;
mod error;
mod parsing;
mod shell;

pub use error::Result;
pub use parsing::ParsingIterator;
pub use shell::Shell;
