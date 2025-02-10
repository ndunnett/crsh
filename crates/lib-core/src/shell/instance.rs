use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use sysexits::ExitCode;

use lib_os::{dir, io};

use crate::{config::Config, parsing::Parser};

#[derive(Debug)]
pub struct Shell {
    pub(crate) io: io::Context,
    pub(crate) config: Config,
    pub(crate) exit_code: ExitCode,
    pub(crate) should_exit: bool,
    pub(crate) pwd: String,
    pub(crate) old_pwd: String,
    pub(crate) args: Vec<String>,
    pub(crate) _variables: HashMap<String, String>, // todo
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            io: io::Context::default(),
            config: Config::default(),
            exit_code: ExitCode::Ok,
            should_exit: false,
            pwd: dir::current(),
            old_pwd: String::new(),
            args: std::env::args().collect(),
            _variables: HashMap::new(),
        }
    }
}

impl Shell {
    pub fn interpret(&mut self, input: &str) -> ExitCode {
        match Parser::new(input).parse() {
            Ok(ast) => {
                println!("\n{ast:#?}\n");

                match self.execute(None, &ast) {
                    Ok(code) => {
                        self.set_exit_code(code);
                    }
                    Err(e) => {
                        self.io.eprintln(format!("crsh: interpreter error: {e:#?}"));
                        self.set_exit_code(ExitCode::DataErr);
                    }
                }
            }
            Err(errors) => {
                for e in errors {
                    self.io.eprintln(e.to_string());
                }

                self.set_exit_code(ExitCode::DataErr);
            }
        };

        self.exit_code
    }

    pub fn stdin(&mut self) -> &mut io::Input {
        &mut self.io.input
    }

    pub fn stdout(&mut self) -> &mut io::Output {
        &mut self.io.output
    }

    pub fn stderr(&mut self) -> &mut io::Output {
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
        let home = dir::my_home();

        if self.pwd.starts_with(&home) {
            Some(self.pwd.replacen(&home, "~", 1))
        } else {
            Some(self.pwd.clone())
        }
    }

    pub fn config_filepath<S: AsRef<Path>>(&self, filename: S) -> PathBuf {
        let mut path = self.config.path.clone();
        path.push(filename);
        path
    }
}
