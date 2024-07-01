use sysexits::ExitCode;

mod builtin;
mod common_env;
mod config;
mod error;
mod execution;
mod parser;
mod shell_io;

pub use builtin::*;
pub use common_env::*;
pub use config::*;
pub use error::*;
pub use parser::*;
pub use shell_io::*;

pub struct Shell {
    pub env: CommonEnv,
    pub exit_code: ExitCode,
    pub io: IOContext,
    pub config: ShellConfig,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            env: CommonEnv::default(),
            exit_code: ExitCode::Ok,
            io: IOContext::default(),
            config: ShellConfig::default(),
        }
    }
}
