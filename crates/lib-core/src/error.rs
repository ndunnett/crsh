#![allow(dead_code)]

use derive_more::From;

use crate::parsing::ParseErrorVariant;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum SourceErrorVariant {
    Parse(ParseErrorVariant),
}

impl SourceErrorVariant {
    fn human_description(&self) -> &str {
        match self {
            SourceErrorVariant::Parse(ParseErrorVariant::IncompleteParse) => {
                "Failed to parse remaining input"
            }
            SourceErrorVariant::Parse(ParseErrorVariant::UnexpectedTokens) => {
                "Unexpected tokens in expression"
            }
            SourceErrorVariant::Parse(ParseErrorVariant::UnmatchedParenthesis) => {
                "Unmatched parenthesis"
            }
            SourceErrorVariant::Parse(ParseErrorVariant::UnmatchedBrace) => {
                "Unmatched brace"
            },
            SourceErrorVariant::Parse(ParseErrorVariant::InvalidName) => {
                "Name must not start with a number and must consist only of alphanumeric characters or '_'"
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceError {
    pub(crate) variant: SourceErrorVariant,
    pub(crate) lexeme: String,
    pub(crate) span: std::ops::Range<usize>,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl std::error::Error for SourceError {}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {:?}: {}.\n[line {}, column {}] \"{}\"",
            self.variant,
            self.variant.human_description(),
            self.line,
            self.column,
            self.lexeme,
        )
    }
}

#[derive(Debug, From)]
pub enum Error {
    Adhoc {
        string: String,
    },

    #[from]
    Source(SourceError),

    #[from]
    Io(std::io::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Adhoc { .. } => None,
            Error::Io(error) => error.source(),
            Error::Source(error) => error.source(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl From<&str> for Error {
    fn from(string: &str) -> Self {
        Self::Adhoc {
            string: String::from(string),
        }
    }
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Self::Adhoc { string }
    }
}
