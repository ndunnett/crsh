use rustyline::{
    error::ReadlineError, history::FileHistory, CompletionType, Config, EditMode, Editor,
};
use sysexits::ExitCode;

use crsh_core::Shell;

mod helper;
use helper::PromptHelper;

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

    fn build_readline(&self) -> rustyline::Result<Editor<PromptHelper, FileHistory>> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();

        let mut rl = Editor::with_config(config)?;
        rl.set_helper(Some(PromptHelper::new()));
        rl.load_history(&self.shell.config_filepath(&self.shell.config.history_file))?;

        Ok(rl)
    }

    fn render_prompt(&self) -> (String, String) {
        let ps1 = "$";
        let home = &self.shell.env.home.to_string_lossy().to_string();
        let mut pwd = self.shell.env.pwd.to_string_lossy().to_string();

        if pwd.starts_with(home) {
            pwd = pwd.replacen(home, "~", 1);
        }

        let uncoloured = format!("{pwd} {ps1} ");

        let indicator_colour = if self.shell.exit_code.is_success() {
            self.style.colour_success
        } else {
            self.style.colour_fail
        };

        let coloured = format!(
            "{} {} ",
            self.style.colour_path.paint(pwd),
            indicator_colour.paint(ps1)
        );

        (coloured, uncoloured)
    }

    pub fn repl(&mut self) -> ExitCode {
        let mut rl = match self.build_readline() {
            Ok(rl) => rl,
            Err(e) => {
                self.shell.io.eprintln(format!("crsh: error: {e:?}"));
                self.shell.exit_code = ExitCode::DataErr;
                return self.shell.exit_code;
            }
        };

        loop {
            let (coloured, uncoloured) = self.render_prompt();
            rl.helper_mut().expect("No helper").set_prompt(coloured);

            match rl.readline(&uncoloured) {
                Ok(buffer) => {
                    _ = rl.add_history_entry(buffer.as_str());
                    self.shell.interpret(&buffer);
                }
                Err(ReadlineError::Interrupted) => {
                    self.shell.io.println("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    self.shell.io.println("^D");
                    break;
                }
                Err(e) => {
                    self.shell.io.eprintln(format!("crsh: error: {e:?}"));
                    self.shell.exit_code = ExitCode::DataErr;
                }
            }
        }

        _ = rl.append_history(&self.shell.config_filepath(&self.shell.config.history_file));
        self.shell.exit_code
    }
}
