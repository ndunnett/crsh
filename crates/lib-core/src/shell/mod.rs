use std::path::{Path, PathBuf};

use sysexits::ExitCode;

use crate::{Config, IOContext, Input, Output};

#[derive(Debug)]
pub struct Shell {
    pub(super) io: IOContext,
    pub(super) config: Config,
    pub(super) exit_code: ExitCode,
    pub(super) should_exit: bool,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            io: IOContext::default(),
            config: Config::default(),
            exit_code: ExitCode::Ok,
            should_exit: false,
        }
    }
}

impl Shell {
    pub fn stdin(&mut self) -> &mut Input {
        &mut self.io.input
    }

    pub fn stdout(&mut self) -> &mut Output {
        &mut self.io.output
    }

    pub fn stderr(&mut self) -> &mut Output {
        &mut self.io.error
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn exit_code(&self) -> ExitCode {
        self.exit_code
    }

    pub fn set_exit_code(&mut self, code: ExitCode) {
        self.exit_code = code;
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn pretty_pwd(&self) -> Option<String> {
        let home = std::env::var("HOME").ok()?;
        let pwd = std::env::var("PWD").ok()?;

        if pwd.starts_with(&home) {
            Some(pwd.replacen(&home, "~", 1))
        } else {
            Some(pwd)
        }
    }

    pub fn find_on_path<P: AsRef<Path>>(&self, keyword: P) -> Option<PathBuf> {
        std::env::split_paths(&std::env::var_os("PATH")?).find(|dir| dir.join(&keyword).is_file())
    }

    pub fn config_filepath<S: AsRef<Path>>(&self, filename: S) -> PathBuf {
        let mut path = self.config.profile_path.clone();
        path.push(filename);
        path
    }
}
