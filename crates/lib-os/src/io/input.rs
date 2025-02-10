use std::{fs, process::Stdio};

#[derive(Debug)]
pub enum Input {
    Null,
    Pipe(os_pipe::PipeReader),
    File(fs::File),
    Stdin(std::io::Stdin),
}

impl From<std::io::Stdin> for Input {
    fn from(input: std::io::Stdin) -> Self {
        Self::Stdin(input)
    }
}

impl From<fs::File> for Input {
    fn from(input: fs::File) -> Self {
        Self::File(input)
    }
}

impl From<os_pipe::PipeReader> for Input {
    fn from(input: os_pipe::PipeReader) -> Self {
        Self::Pipe(input)
    }
}

impl From<Input> for Stdio {
    fn from(input: Input) -> Stdio {
        match input {
            Input::Null => Stdio::null(),
            Input::Pipe(pipe) => pipe.into(),
            Input::File(file) => file.into(),
            Input::Stdin(_) => Stdio::inherit(),
        }
    }
}

impl std::io::Read for Input {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        match *self {
            Self::Null => Ok(0),
            Self::Pipe(ref mut pipe) => pipe.read(buffer),
            Self::File(ref mut file) => file.read(buffer),
            Self::Stdin(ref mut stream) => stream.read(buffer),
        }
    }
}

impl Input {
    pub fn try_clone(&self) -> std::io::Result<Self> {
        match *self {
            Self::Null => Ok(Self::Null),
            Self::Pipe(ref pipe) => Ok(Self::Pipe(pipe.try_clone()?)),
            Self::File(ref file) => Ok(Self::File(file.try_clone()?)),
            Self::Stdin(_) => Ok(Self::Stdin(std::io::stdin())),
        }
    }
}
