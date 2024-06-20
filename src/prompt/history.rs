use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const MAX_HISTORY_SIZE: usize = 25;

#[derive(Default)]
pub struct PromptHistory {
    history: Vec<Vec<char>>,
    index: usize,
}

impl PromptHistory {
    pub fn new<S: AsRef<Path>>(history_file: S) -> Self {
        let history = match fs::OpenOptions::new().read(true).open(history_file) {
            Ok(mut file) => {
                let mut buf = String::new();
                let _ = file.read_to_string(&mut buf);

                buf.trim()
                    .split('\n')
                    .map(|line| line.chars().collect())
                    .collect()
            }
            Err(_) => Vec::new(),
        };

        let index = history.len();
        Self { history, index }
    }

    pub fn save(&self, history_file: PathBuf) -> io::Result<()> {
        fs::create_dir_all(history_file.parent().unwrap())?;

        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(history_file)?;

        writeln!(f, "{self}")
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
