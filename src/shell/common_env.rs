use std::env;
use std::path::PathBuf;

pub struct CommonEnv {
    pub pwd: String,
    pub oldpwd: String,
    pub home: String,
    pub path: Vec<PathBuf>,
    pub ps1: String,
    pub ps2: String,
    pub ps4: String,
}

impl Default for CommonEnv {
    fn default() -> Self {
        let pwd = Self::get_string_or("PWD", "/");
        let oldpwd = Self::get_string_or("OLDPWD", &pwd);
        let path = Self::get_path().unwrap_or_else(|| {
            env::split_paths("/usr/sbin:/usr/bin:/sbin:/bin").collect::<Vec<_>>()
        });

        Self {
            pwd,
            oldpwd,
            home: Self::get_string_or("HOME", "/"),
            path,
            ps1: Self::get_string_or("PS1", "$"),
            ps2: Self::get_string_or("PS2", ">"),
            ps4: Self::get_string_or("PS4", "+"),
        }
    }
}

impl CommonEnv {
    fn get_string(var_name: &str) -> Option<String> {
        env::var(var_name).ok()
    }

    fn get_string_or(var_name: &str, or: &str) -> String {
        Self::get_string(var_name).unwrap_or(or.into())
    }

    fn get_path() -> Option<Vec<PathBuf>> {
        env::var_os("PATH").map(|path| env::split_paths(&path).collect::<Vec<_>>())
    }
}
