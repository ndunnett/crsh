use std::{
    env, fs,
    io::{self, IsTerminal, Read},
    path::PathBuf,
};

use argh::{ArgsInfo, FromArgValue, FromArgs};
use itertools::Itertools;
use sysexits::ExitCode;

use crsh::{Prompt, Shell};

#[derive(Debug, FromArgs, ArgsInfo)]
#[argh(description = "Crappy Rust SHell.")]
struct Cli {
    /// execute script at path
    #[argh(positional)]
    path: Option<String>,

    /// run command non-interactively
    #[argh(option, short = 'c')]
    command: Option<String>,

    /// input from stdin
    #[argh(switch, hidden_help)]
    stdin: bool,

    /// path to the history file to use
    #[argh(option, short = 'H', arg_name = "PATH")]
    history: Option<String>,

    /// set shell option
    #[argh(option, short = 'o', arg_name = "OPTION", from_str_fn(parse_options))]
    set: Vec<Vec<ShellOption>>,

    /// unset shell option
    #[argh(option, short = 'u', arg_name = "OPTION", from_str_fn(parse_options))]
    unset: Vec<Vec<ShellOption>>,

    /// display version information
    #[argh(switch, short = 'V')]
    version: bool,
}

macro_rules! shell_options {
    ($($id:ident : $description:expr),* $(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromArgValue)]
        enum ShellOption {
            $($id,)*
        }

        impl ShellOption {
            const fn display_possible_options() -> &'static str {
                concat!("Shell Options:\n", $("- ", stringify!($id), ": ", $description, "\n",)*)
            }
        }
    };
}

shell_options!(None: "shell options not yet implemented");

fn parse_options(value: &str) -> Result<Vec<ShellOption>, String> {
    value.split(',').map(ShellOption::from_arg_value).collect()
}

fn flatten_options(options: Vec<Vec<ShellOption>>) -> Vec<ShellOption> {
    options
        .into_iter()
        .flatten()
        .sorted_unstable()
        .dedup()
        .collect()
}

#[derive(Clone, PartialEq, Eq)]
pub enum ShellMode {
    Interactive(Option<String>),
    Read,
    Command(String),
    Script(String),
}

impl From<&Cli> for ShellMode {
    fn from(cli: &Cli) -> Self {
        if cli.stdin {
            Self::Read
        } else if let Some(cmd) = &cli.command {
            Self::Command(cmd.into())
        } else if let Some(path) = &cli.path {
            Self::Script(path.into())
        } else {
            Self::Interactive(cli.history.clone())
        }
    }
}

struct ShellContext {
    mode: ShellMode,
    #[expect(unused)]
    set: Vec<ShellOption>,
    #[expect(unused)]
    unset: Vec<ShellOption>,
}

impl From<Cli> for ShellContext {
    fn from(cli: Cli) -> Self {
        Self {
            mode: ShellMode::from(&cli),
            set: flatten_options(cli.set),
            unset: flatten_options(cli.unset),
        }
    }
}

impl ShellContext {
    fn invoke(self) -> ExitCode {
        // TODO: implement shell options + config loading
        let mut shell = Shell::default();

        match self.mode {
            ShellMode::Interactive(history) => {
                let history_source =
                    history.map_or_else(|| shell.config_filepath(".crsh-history"), PathBuf::from);

                let mut prompt = Prompt::new(&mut shell).with_history(history_source);

                match prompt.repl() {
                    Ok(code) => code,
                    Err(e) => {
                        eprintln!("crsh: prompt error: {e:#?}");
                        ExitCode::OsErr
                    }
                }
            }
            ShellMode::Read => {
                let mut input = String::new();

                match io::stdin().read_to_string(&mut input) {
                    Ok(_) => shell.interpret(&input),
                    Err(e) => {
                        eprintln!("crsh: failed to read stdin: {e}");
                        ExitCode::IoErr
                    }
                }
            }
            ShellMode::Command(input) => shell.interpret(&input),
            ShellMode::Script(path) => match fs::read_to_string(&path) {
                Ok(script) => shell.interpret(&script),
                Err(e) => {
                    eprintln!("crsh: failed to run script at \"{path}\": {e}");
                    ExitCode::NoInput
                }
            },
        }
    }
}

enum Invocation {
    Shell(ShellContext),
    Version,
    Help(String),
    UsageError(String),
}

impl Invocation {
    fn from_env() -> Self {
        let args = env::args().skip(1).collect::<Vec<_>>();
        let args = args.iter().map(String::as_str).collect::<Vec<_>>();
        Self::from_args(&args)
    }

    fn from_args(args: &[&str]) -> Self {
        let mut cli = match Cli::from_args(&["crsh"], args) {
            Ok(cli) => cli,
            Err(exit) => {
                if exit.status.is_ok() {
                    return Self::Help(format!(
                        "{}\n{}",
                        exit.output,
                        ShellOption::display_possible_options()
                    ));
                }

                return Self::UsageError(exit.output);
            }
        };

        if cli.version {
            return Self::Version;
        }

        if !io::stdin().is_terminal() {
            cli.stdin = true;
        }

        let inputs = usize::from(cli.path.is_some())
            + usize::from(cli.command.is_some())
            + usize::from(cli.stdin);

        if inputs > 1 {
            return Self::UsageError(
                "crsh: specify at most one of a script path, --command, or piped input to stdin"
                    .to_string(),
            );
        }

        Self::Shell(ShellContext::from(cli))
    }
}

fn main() -> ExitCode {
    match Invocation::from_env() {
        Invocation::Shell(shell_context) => shell_context.invoke(),
        Invocation::Version => {
            println!("crsh v{}", env!("VERSION"));
            ExitCode::Ok
        }
        Invocation::Help(msg) => {
            println!("{msg}");
            ExitCode::Ok
        }
        Invocation::UsageError(msg) => {
            eprintln!("{msg}");
            ExitCode::Usage
        }
    }
}
