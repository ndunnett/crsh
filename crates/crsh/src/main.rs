use std::env;
use std::io;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use crossterm::tty::IsTty;

use crsh_core::*;

#[derive(Parser, Debug)]
#[command(version = env!("VERSION"))]
#[command(after_help = ShellOption::display_possible_options())]
#[command(about, long_about = None)]
#[group(multiple = false)]
struct Cli {
    /// Execute script at path
    #[arg(group = "input")]
    path: Option<String>,

    /// Run command non-interactively
    #[arg(short, long, group = "input")]
    command: Option<String>,

    /// Input from stdin
    #[arg(long, hide = true, group = "input")]
    stdin: bool,

    /// Name of history file to use
    #[arg(short = 'H', long, value_name = "FILENAME")]
    history_file: Option<String>,

    /// Set shell option
    #[arg(
        value_enum,
        short = 'o',
        value_delimiter = ',',
        hide_possible_values = true,
        value_name = "OPTION"
    )]
    set: Vec<ShellOption>,

    /// Unset shell option
    #[arg(
        value_enum,
        short = 'u',
        value_delimiter = ',',
        hide_possible_values = true,
        value_name = "OPTION"
    )]
    unset: Vec<ShellOption>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ShellOption {
    /// Setting shell options not yet implemented
    None,
}

impl ShellOption {
    fn display_possible_options() -> String {
        let options = Self::value_variants()
            .iter()
            .filter_map(|v| v.to_possible_value())
            .map(|v| format!("- {}: {}", v.get_name(), v.get_help().unwrap_or_default()));

        ["Settable shell options:".to_string()]
            .into_iter()
            .chain(options)
            .collect::<Vec<_>>()
            .join("\n  ")
    }
}

impl Cli {
    pub fn parse_shell_mode(&self) -> ShellMode {
        if self.stdin {
            ShellMode::Read
        } else if let Some(cmd) = &self.command {
            ShellMode::Command(cmd.into())
        } else if let Some(path) = &self.path {
            ShellMode::Script(path.into())
        } else {
            ShellMode::Interactive
        }
    }
}

impl From<Cli> for Shell {
    fn from(cli: Cli) -> Self {
        let mut sh = Self::default();
        sh.config.args = env::args().collect::<Vec<_>>();

        if let Some(history_file) = cli.history_file {
            sh.config.history_file = history_file;
        }

        sh
    }
}

fn main() -> ExitCode {
    let mut args = env::args().collect::<Vec<_>>();

    if !io::stdin().is_tty() {
        args.push("--stdin".into());
    }

    let cli = Cli::parse_from(args);
    let mode = cli.parse_shell_mode();

    Shell::from(cli).main(mode)
}
