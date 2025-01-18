#[derive(Debug, Clone, PartialEq)]
pub enum Expansion<'a> {
    Unquoted(&'a str),
    DoubleQuoted(&'a str),
    SingleQuoted(&'a str),
}

impl Expansion<'_> {
    pub fn expand(&self) -> String {
        match self {
            Self::Unquoted(s) => Self::unquoted(s),
            Self::DoubleQuoted(s) => Self::double_quoted(s),
            Self::SingleQuoted(s) => s.to_string(),
        }
    }

    fn unquoted(text: &str) -> String {
        let expanded = {
            if text.starts_with('~') {
                let home = std::env::var("HOME").unwrap_or_default();
                Some(text.replacen('~', &home, 1))
            } else {
                None
            }
        };

        if let Some(expanded) = expanded {
            Self::unquoted(&expanded)
        } else {
            Self::double_quoted(text)
        }
    }

    fn double_quoted(text: &str) -> String {
        if text.starts_with('$') {
            std::env::var(&text[1..text.len()]).unwrap_or_default()
        } else {
            text.into()
        }
    }
}
