use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{cursor, event, execute, queue, style, terminal};
use itertools::Itertools;
use sysexits::ExitCode;

use lib_core::{Result, Shell};

use crate::{
    editor::{Editor, Signal},
    history::Source,
    style::Style,
};

pub struct Prompt<'a> {
    shell: &'a mut Shell,
    style: Style,
    editor: Editor,
}

impl<'a> Prompt<'a> {
    pub fn new(shell: &'a mut Shell) -> Prompt<'a> {
        Prompt {
            shell,
            style: Style::default(),
            editor: Editor::default(),
        }
    }

    pub fn with_history<S: Into<Source>>(mut self, source: S) -> Prompt<'a> {
        self.editor.set_history(source);
        self
    }

    fn generate_prompt(&self) -> (String, usize) {
        let pwd = self.shell.pretty_pwd().unwrap_or_default();
        let ps1 = "$";

        let indicator_colour = if self.shell.exit_code().is_success() {
            self.style.colour_success
        } else {
            self.style.colour_fail
        };

        let len = pwd.len() + ps1.len() + 2;
        let mut bytes = Vec::new();

        _ = execute!(
            bytes,
            style::SetForegroundColor(self.style.colour_path),
            style::Print(pwd),
            style::Print(' '),
            style::SetForegroundColor(indicator_colour),
            style::Print(ps1),
            style::ResetColor,
            style::Print(' '),
        );

        let prompt = String::from_utf8(bytes).unwrap_or_default();
        (prompt, len)
    }

    fn render(&mut self) -> io::Result<()> {
        let (prompt, prompt_len) = self.generate_prompt();
        let cols = terminal::size()?.0 as usize - 1;
        let first_len = prompt.len() + cols.saturating_sub(prompt_len);

        let chars = prompt.chars().chain(self.editor.iter());
        let mut lines = vec![String::from_iter(chars.clone().take(first_len))];

        for line in chars.skip(first_len).chunks(cols).into_iter() {
            lines.push(String::from_iter(line));
        }

        let mut lines = lines.into_iter();
        let last_line = lines.next_back().unwrap_or_default();

        queue!(
            self.shell.stdout(),
            cursor::RestorePosition,
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )?;

        for line in lines {
            queue!(
                self.shell.stdout(),
                style::Print(line),
                style::Print('\n'),
                cursor::MoveToColumn(0),
            )?;
        }

        queue!(self.shell.stdout(), style::Print(last_line))?;

        let cursor_index = prompt_len + self.editor.cursor();
        let mut cursor_col = cursor_index % cols;
        let mut cursor_row = (cursor_index - cursor_col) / cols;

        if cursor_col == 0 && cursor_row > 0 {
            cursor_col = cols;
            cursor_row -= 1;
        }

        queue!(
            self.shell.stdout(),
            cursor::RestorePosition,
            cursor::MoveToColumn(cursor_col as u16),
        )?;

        if cursor_row > 0 {
            queue!(self.shell.stdout(), cursor::MoveDown(cursor_row as u16))?;
        }

        self.shell.stdout().flush()?;
        Ok(())
    }

    fn read_line(&mut self) -> io::Result<Signal> {
        terminal::enable_raw_mode()?;

        queue!(
            self.shell.stdout(),
            cursor::MoveToColumn(0),
            cursor::SavePosition
        )?;

        loop {
            self.render()?;

            if event::poll(Duration::from_millis(500))? {
                match self.editor.poll() {
                    Ok(Signal::None) => {}
                    signal => {
                        queue!(
                            self.shell.stdout(),
                            style::Print('\n'),
                            cursor::MoveToColumn(0)
                        )?;

                        self.shell.stdout().flush()?;
                        terminal::disable_raw_mode()?;
                        return signal;
                    }
                }
            }
        }
    }

    pub fn repl(&mut self) -> Result<ExitCode> {
        _ = ctrlc::set_handler(|| {});

        while !self.shell.should_exit() {
            match self.read_line() {
                Ok(Signal::Buffer(buffer)) => {
                    self.shell.interpret(&buffer);
                }
                Ok(Signal::Interrupt) => {
                    writeln!(self.shell.stdout(), "^C")?;
                    self.shell.set_exit_code(ExitCode::DataErr);
                    continue;
                }
                Ok(Signal::End) => {
                    writeln!(self.shell.stdout(), "^D")?;
                    self.shell.set_exit_code(ExitCode::DataErr);
                    break;
                }
                Err(e) => {
                    writeln!(self.shell.stderr(), "crsh: error: {e:?}")?;
                    self.shell.set_exit_code(ExitCode::DataErr);
                }
                _ => {}
            }
        }

        Ok(self.shell.exit_code())
    }
}
