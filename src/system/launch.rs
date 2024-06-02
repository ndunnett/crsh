use std::path::PathBuf;
use std::process::Command;

use crate::interpreter::{Executable, ExecutionContext};

pub struct Launch {
    path: PathBuf,
    args: Vec<String>,
}

impl Launch {
    pub fn new(path: PathBuf, args: Vec<&str>) -> Box<Self> {
        Box::new(Self {
            path,
            args: args.iter().map(|s| s.to_string()).collect(),
        })
    }
}

impl Executable for Launch {
    fn run(&self, ctx: ExecutionContext) -> Result<(), ()> {
        match Command::new(self.path.as_path())
            .stdin(ctx.input)
            .stdout(ctx.output)
            .stderr(ctx.error)
            .args(&self.args)
            .spawn()
        {
            Ok(mut c) => match c.wait() {
                Ok(status) => {
                    if status.success() {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                Err(_) => Err(()),
            },
            Err(e) => {
                eprintln!("crsh: {}", e);
                Err(())
            }
        }
    }
}
