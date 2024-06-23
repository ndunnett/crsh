use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct CommonEnv {
    pub pwd: PathBuf,
    pub oldpwd: PathBuf,
    pub home: PathBuf,
    pub config: PathBuf,
    pub path: Vec<PathBuf>,
    pub ps1: String,
    pub ps2: String,
    pub ps4: String,
}

impl Default for CommonEnv {
    fn default() -> Self {
        let pwd = Self::get_pathbuf("PWD").unwrap_or_else(|| "/".into());
        let oldpwd = Self::get_pathbuf("OLDPWD").unwrap_or_else(|| pwd.clone());
        let home = Self::get_pathbuf("HOME").unwrap_or_else(|| "/".into());

        let mut config = Self::get_pathbuf("XDG_CONFIG_HOME").unwrap_or_else(|| {
            let mut c = home.clone();
            c.push(".config");
            c
        });
        config.push("crsh");

        let path = env::var_os("PATH")
            .map(|path| env::split_paths(&path).collect())
            .unwrap_or_else(|| env::split_paths("/usr/sbin:/usr/bin:/sbin:/bin").collect());

        let ps1 = Self::get_string("PS1").unwrap_or_else(|| "$".into());
        let ps2 = Self::get_string("PS2").unwrap_or_else(|| ">".into());
        let ps4 = Self::get_string("PS4").unwrap_or_else(|| "+".into());

        Self {
            pwd,
            oldpwd,
            home,
            config,
            path,
            ps1,
            ps2,
            ps4,
        }
    }
}

impl CommonEnv {
    pub fn get_string<S: AsRef<OsStr>>(var_name: S) -> Option<String> {
        env::var(var_name).ok()
    }

    pub fn get_pathbuf<S: AsRef<OsStr>>(var_name: S) -> Option<PathBuf> {
        env::var(var_name).ok().map(PathBuf::from)
    }
}
