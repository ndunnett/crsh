use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use itertools::Itertools;

pub enum Source {
    None,
    File(PathBuf),
}

impl Default for Source {
    fn default() -> Self {
        Self::None
    }
}

impl From<PathBuf> for Source {
    fn from(file_path: PathBuf) -> Self {
        _ = fs::create_dir_all(file_path.parent().unwrap());
        Self::File(file_path)
    }
}

impl Source {
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
pub struct History {
    source: Source,
    buffers: Vec<Vec<char>>,
    index: usize,
}

impl History {
    const MAX_SIZE: usize = 50;

    pub fn new<S>(source: S) -> Self
    where
        Source: std::convert::From<S>,
    {
        let source = Source::from(source);
        let history = source.read().unwrap();
        let index = history.len();

        Self {
            source,
            buffers: history,
            index,
        }
    }

    pub fn save(&self) -> io::Result<()> {
        self.source.save(&self.buffers)
    }

    pub fn back(&mut self) -> Option<Vec<char>> {
        if self.index > 0 {
            self.index -= 1;
            Some(self.buffers[self.index].clone())
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<Vec<char>> {
        match self.buffers.len() - self.index {
            0 => None,
            1 => {
                self.index += 1;
                Some(Vec::new())
            }
            _ => {
                self.index += 1;
                Some(self.buffers[self.index].clone())
            }
        }
    }

    pub fn push(&mut self, buffer: Vec<char>) {
        if self.buffers.last() != Some(&buffer) {
            self.buffers.push(buffer.clone());

            if self.buffers.len() > Self::MAX_SIZE {
                self.buffers.remove(0);
            }
        }

        self.index = self.buffers.len();
    }
}
