use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub mod io;
mod launch;

pub use launch::Launch;

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
