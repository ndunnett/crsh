use reedline::{DefaultHinter, FileBackedHistory, Reedline, Signal};
use sysexits::ExitCode;

use crsh_core::Shell;

mod readline;
mod style;

use style::PromptStyle;

pub struct Prompt<'a> {
    shell: &'a mut Shell,
    style: PromptStyle,
}

impl<'a> Prompt<'a> {
    pub fn new(shell: &'a mut Shell) -> Prompt<'a> {
        Prompt {
            shell,
            style: PromptStyle::new(),
        }
    }

    pub fn repl(&mut self) -> ExitCode {
        let history = Box::new(
            FileBackedHistory::with_file(
                50,
                self.shell.config_filepath(&self.shell.config.history_file),
            )
            .expect("Error configuring history with file"),
        );

        let mut rl = Reedline::create()
            .with_history(history)
            .with_hinter(Box::<DefaultHinter>::default());

        loop {
            match rl.read_line(self) {
                Ok(Signal::Success(buffer)) => {
                    self.shell.interpret(&buffer);
                }
                Ok(Signal::CtrlC) => {
                    self.shell.io.println("^C");
                    continue;
                }
                Ok(Signal::CtrlD) => {
                    self.shell.io.println("^D");
                    break;
                }
                Err(e) => {
                    self.shell.io.eprintln(format!("crsh: error: {e:?}"));
                    self.shell.exit_code = ExitCode::DataErr;
                }
            }
        }

        self.shell.exit_code
    }
}
