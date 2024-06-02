use std::fs;
use std::io;
use std::process::Stdio;

#[derive(Debug)]
pub enum Input {
    File(fs::File),
    Stdin(io::Stdin),
}

impl Default for Input {
    fn default() -> Self {
        Self::Stdin(io::stdin())
    }
}

impl From<io::Stdin> for Input {
    fn from(input: io::Stdin) -> Self {
        Self::Stdin(input)
    }
}

impl From<fs::File> for Input {
    fn from(input: fs::File) -> Self {
        Self::File(input)
    }
}

impl From<Input> for Stdio {
    fn from(input: Input) -> Stdio {
        match input {
            Input::File(file) => file.into(),
            Input::Stdin(_) => Stdio::inherit(),
        }
    }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.read(buf),
            Self::Stdin(ref mut stream) => stream.read(buf),
        }
    }
}

impl Clone for Input {
    fn clone(&self) -> Self {
        match *self {
            Self::File(ref file) => Self::File(file.try_clone().unwrap()),
            Self::Stdin(_) => Self::Stdin(io::stdin()),
        }
    }
}

#[derive(Debug)]
pub enum Output {
    File(fs::File),
    Stdout(io::Stdout),
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

impl From<fs::File> for Output {
    fn from(output: fs::File) -> Self {
        Self::File(output)
    }
}

impl From<Output> for Stdio {
    fn from(output: Output) -> Stdio {
        match output {
            Output::File(file) => file.into(),
            Output::Stdout(_) => Stdio::inherit(),
        }
    }
}

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.write(buf),
            Self::Stdout(ref mut stream) => stream.write(buf),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match *self {
            Self::File(ref mut file) => file.write_all(buf),
            Self::Stdout(ref mut stream) => stream.write_all(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Self::File(ref mut file) => file.flush(),
            Self::Stdout(ref mut stream) => stream.flush(),
        }
    }
}

impl Clone for Output {
    fn clone(&self) -> Self {
        match *self {
            Self::File(ref file) => Self::File(file.try_clone().unwrap()),
            Self::Stdout(_) => Self::Stdout(io::stdout()),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    File(fs::File),
    Stdout(io::Stdout),
    Stderr(io::Stderr),
}

impl Default for Error {
    fn default() -> Self {
        Self::Stdout(io::stdout())
    }
}

impl From<io::Stdout> for Error {
    fn from(output: io::Stdout) -> Self {
        Self::Stdout(output)
    }
}

impl From<io::Stderr> for Error {
    fn from(output: io::Stderr) -> Self {
        Self::Stderr(output)
    }
}

impl From<fs::File> for Error {
    fn from(output: fs::File) -> Self {
        Self::File(output)
    }
}

impl From<Error> for Stdio {
    fn from(output: Error) -> Stdio {
        match output {
            Error::File(file) => file.into(),
            Error::Stdout(_) => Stdio::inherit(),
            Error::Stderr(_) => Stdio::inherit(),
        }
    }
}

impl io::Write for Error {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Self::File(ref mut file) => file.write(buf),
            Self::Stdout(ref mut stream) => stream.write(buf),
            Self::Stderr(ref mut stream) => stream.write(buf),
        }
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        match *self {
            Self::File(ref mut file) => file.write_all(buf),
            Self::Stdout(ref mut stream) => stream.write_all(buf),
            Self::Stderr(ref mut stream) => stream.write_all(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Self::File(ref mut file) => file.flush(),
            Self::Stdout(ref mut stream) => stream.flush(),
            Self::Stderr(ref mut stream) => stream.flush(),
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match *self {
            Self::File(ref file) => Self::File(file.try_clone().unwrap()),
            Self::Stdout(_) => Self::Stdout(io::stdout()),
            Self::Stderr(_) => Self::Stderr(io::stderr()),
        }
    }
}
