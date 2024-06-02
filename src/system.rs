use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

pub mod io;

use crate::interpreter::ExecutionContext;

pub fn launch(keyword: &str, args: Vec<&str>, ctx: ExecutionContext) -> Result<(), ()> {
    let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    match Command::new(keyword)
        .stdin(ctx.input)
        .stdout(ctx.output)
        .stderr(ctx.error)
        .args(&args)
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

pub fn find_on_path<P>(keyword: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let path = dir.join(&keyword);

                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
            .next()
    })
}

pub fn home() -> String {
    env::var_os("HOME")
        .unwrap_or(OsString::from("/"))
        .into_string()
        .unwrap_or("".to_string())
}
