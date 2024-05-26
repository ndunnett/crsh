use std::env;
use std::ffi::OsString;

pub fn home() -> String {
    env::var_os("HOME")
        .unwrap_or(OsString::from("/"))
        .into_string()
        .unwrap_or("".to_string())
}
