use std::{fs, process::Stdio};

#[derive(Debug)]
pub enum Output {
    Null,
    Pipe(os_pipe::PipeWriter),
    File(fs::File),
    Stdout(std::io::Stdout),
    Stderr(std::io::Stderr),
}

impl From<std::io::Stdout> for Output {
    fn from(output: std::io::Stdout) -> Self {
        Self::Stdout(output)
    }
}

impl From<std::io::Stderr> for Output {
    fn from(output: std::io::Stderr) -> Self {
        Self::Stderr(output)
    }
}

impl From<fs::File> for Output {
    fn from(output: fs::File) -> Self {
        Self::File(output)
    }
}

impl From<os_pipe::PipeWriter> for Output {
    fn from(output: os_pipe::PipeWriter) -> Self {
        Self::Pipe(output)
    }
}

impl From<Output> for Stdio {
    fn from(output: Output) -> Stdio {
        match output {
            Output::Null => Stdio::null(),
            Output::Pipe(pipe) => pipe.into(),
            Output::File(file) => file.into(),
            Output::Stdout(_) => Stdio::inherit(),
            Output::Stderr(_) => Stdio::inherit(),
        }
    }
}

impl std::io::Write for Output {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        match *self {
            Self::Null => Ok(0),
            Self::Pipe(ref mut pipe) => pipe.write(buffer),
            Self::File(ref mut file) => file.write(buffer),
            Self::Stdout(ref mut stream) => stream.write(buffer),
            Self::Stderr(ref mut stream) => stream.write(buffer),
        }
    }

    fn write_all(&mut self, buffer: &[u8]) -> std::io::Result<()> {
        match *self {
            Self::Null => Ok(()),
            Self::Pipe(ref mut pipe) => pipe.write_all(buffer),
            Self::File(ref mut file) => file.write_all(buffer),
            Self::Stdout(ref mut stream) => stream.write_all(buffer),
            Self::Stderr(ref mut stream) => stream.write_all(buffer),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match *self {
            Self::Null => Ok(()),
            Self::Pipe(ref mut pipe) => pipe.flush(),
            Self::File(ref mut file) => file.flush(),
            Self::Stdout(ref mut stream) => stream.flush(),
            Self::Stderr(ref mut stream) => stream.flush(),
        }
    }
}

impl Output {
    pub fn try_clone(&self) -> std::io::Result<Self> {
        match *self {
            Self::Null => Ok(Self::Null),
            Self::Pipe(ref pipe) => Ok(Self::Pipe(pipe.try_clone()?)),
            Self::File(ref file) => Ok(Self::File(file.try_clone()?)),
            Self::Stdout(_) => Ok(Self::Stdout(std::io::stdout())),
            Self::Stderr(_) => Ok(Self::Stderr(std::io::stderr())),
        }
    }
}
