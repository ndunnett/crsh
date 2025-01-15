use std::{
    fmt, fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use itertools::Itertools;

const MAX_HISTORY_SIZE: usize = 25;

pub enum HistorySource {
    None,
    File(PathBuf),
}

impl Default for HistorySource {
    fn default() -> Self {
        Self::None
    }
}

impl From<PathBuf> for HistorySource {
    fn from(file_path: PathBuf) -> Self {
        _ = fs::create_dir_all(file_path.parent().unwrap());
        Self::File(file_path)
    }
}

impl HistorySource {
    fn save(&self, history: &[Vec<char>]) -> io::Result<()> {
        match self {
            Self::None => Ok(()),
            Self::File(path) => {
                fs::create_dir_all(path.parent().unwrap())?;

                let mut f = fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?;

                let contents = history
                    .iter()
                    .map(|line| line.iter().collect::<String>())
                    .join("\n");

                writeln!(f, "{contents}")
            }
        }
    }

    fn read(&self) -> io::Result<Vec<Vec<char>>> {
        match self {
            Self::None => Ok(Vec::new()),
            Self::File(path) => match fs::OpenOptions::new().read(true).open(path) {
                Ok(mut file) => {
                    let mut buffer = String::new();
                    let _ = file.read_to_string(&mut buffer);

                    Ok(buffer
                        .trim()
                        .split('\n')
                        .map(|line| line.chars().collect())
                        .collect())
                }
                Err(_) => Ok(Vec::new()),
            },
        }
    }
}

#[derive(Default)]
pub struct PromptHistory {
    source: HistorySource,
    history: Vec<Vec<char>>,
    index: usize,
}

impl Drop for PromptHistory {
    fn drop(&mut self) {
        let _ = self.save();
    }
}

impl PromptHistory {
    pub fn new<S>(source: S) -> Self
    where
        HistorySource: std::convert::From<S>,
    {
        let source = HistorySource::from(source);
        let history = source.read().unwrap();
        let index = history.len();

        Self {
            source,
            history,
            index,
        }
    }

    pub fn save(&self) -> io::Result<()> {
        self.source.save(&self.history)
    }

    pub fn back(&mut self) -> Option<Vec<char>> {
        if self.index > 0 {
            self.index -= 1;
            Some(self.history[self.index].clone())
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<Vec<char>> {
        match self.history.len() - self.index {
            0 => None,
            1 => {
                self.index += 1;
                Some(Vec::new())
            }
            _ => {
                self.index += 1;
                Some(self.history[self.index].clone())
            }
        }
    }

    pub fn push(&mut self, buffer: Vec<char>) {
        if self.history.last() != Some(&buffer) {
            self.history.push(buffer.clone());

            if self.history.len() > MAX_HISTORY_SIZE {
                self.history.remove(0);
            }
        }

        self.index = self.history.len();
    }
}

impl fmt::Display for PromptHistory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.history
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
