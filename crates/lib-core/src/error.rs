use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Adhoc {
        string: String,
    },

    #[from]
    Io(std::io::Error),
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
