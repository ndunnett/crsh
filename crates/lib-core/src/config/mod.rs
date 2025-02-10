use std::path::PathBuf;

use lib_os::dir;

#[derive(Debug)]
pub struct Config {
    pub(super) path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let path = PathBuf::from(dir::my_home()).join(".crsh");

        if !path.is_dir() {
            _ = std::fs::create_dir_all(&path);
        }

        Self { path }
    }
}
