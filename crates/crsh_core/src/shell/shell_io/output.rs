use std::fs;
use std::io;
use std::process::Stdio;

#[derive(Debug)]
pub enum Output {
    Null,
    Pipe(os_pipe::PipeWriter),
    File(fs::File),
    Stdout(io::Stdout),
    Stderr(io::Stderr),
}

impl Default for Output {
    fn default() -> Self {
        Self::Stdout(io::stdout())
    }
}

impl From<io::Stdout> for Output {
    fn from(output: io::Stdout) -> Self {
        Self::Stdout(output)
    }
}

impl From<io::Stderr> for Output {
    fn from(output: io::Stderr) -> Self {
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

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Self::Null => Ok(0),
            Self::Pipe(ref mut pipe) => pipe.write(buf),
            Self::File(ref mut file) => file.write(buf),
            Self::Stdout(ref mut stream) => stream.write(buf),
            Self::Stderr(ref mut stream) => stream.write(buf),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match *self {
            Self::Null => Ok(()),
            Self::Pipe(ref mut pipe) => pipe.write_all(buf),
            Self::File(ref mut file) => file.write_all(buf),
            Self::Stdout(ref mut stream) => stream.write_all(buf),
            Self::Stderr(ref mut stream) => stream.write_all(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Self::Null => Ok(()),
            Self::Pipe(ref mut pipe) => pipe.flush(),
            Self::File(ref mut file) => file.flush(),
            Self::Stdout(ref mut stream) => stream.flush(),
            Self::Stderr(ref mut stream) => stream.flush(),
        }
    }
}

impl Clone for Output {
    fn clone(&self) -> Self {
        match *self {
            Self::Null => Self::Null,
            Self::Pipe(ref pipe) => Self::Pipe(pipe.try_clone().unwrap()),
            Self::File(ref file) => Self::File(file.try_clone().unwrap()),
            Self::Stdout(_) => Self::Stdout(io::stdout()),
            Self::Stderr(_) => Self::Stderr(io::stderr()),
        }
    }
}
