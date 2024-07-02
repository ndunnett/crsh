use crate::Shell;

#[derive(Debug, Clone, PartialEq)]
pub enum Expansion<'a> {
    Unquoted(&'a str),
    DoubleQuoted(&'a str),
    SingleQuoted(&'a str),
}

impl Expansion<'_> {
    pub fn expand(&self, sh: &Shell) -> String {
        match self {
            Self::Unquoted(s) => Self::unquoted(s, sh),
            Self::DoubleQuoted(s) => Self::double_quoted(s, sh),
            Self::SingleQuoted(s) => s.to_string(),
        }
    }

    fn unquoted(text: &str, sh: &Shell) -> String {
        let expanded = {
            if text.starts_with('~') {
                let home = sh.env.home.to_string_lossy().to_string();
                Some(text.replacen('~', &home, 1))
            } else {
                None
            }
        };

        if let Some(expanded) = expanded {
            Self::unquoted(&expanded, sh)
        } else {
            Self::double_quoted(text, sh)
        }
    }

    fn double_quoted(text: &str, sh: &Shell) -> String {
        if text.starts_with('$') {
            if let Some(var) = text.get(1..text.len()) {
                sh.get_variable(var).unwrap_or("".into())
            } else {
                text.into()
            }
        } else {
            text.into()
        }
    }
}
