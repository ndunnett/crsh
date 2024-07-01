#[derive(Debug, Clone, PartialEq)]
pub enum Expansion<'a> {
    Unquoted(&'a str),
    DoubleQuoted(&'a str),
    SingleQuoted(&'a str),
}

impl Expansion<'_> {
    pub fn expand(&self) -> String {
        match self {
            Self::Unquoted(s) => s,
            Self::DoubleQuoted(s) => s,
            Self::SingleQuoted(s) => s,
        }
        .to_string()
    }
}
