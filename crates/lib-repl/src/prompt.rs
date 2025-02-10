use std::{
    io::{self, Write},
    time::Duration,
};

use ansi_width::ansi_width;
use crossterm::{cursor, event, queue, style, terminal};
use itertools::Itertools;
use sysexits::ExitCode;

use lib_core::{Result, Shell};

use crate::{
    editor::{Editor, Signal},
    history::Source,
    style::{AppliedStyle, Condition, Function, Style},
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

    fn apply_style(&self, style: &AppliedStyle, bytes: &mut Vec<u8>) {
        match style {
            AppliedStyle::Foreground(colour) => {
                _ = queue!(bytes, style::SetForegroundColor(*colour));
            }
            AppliedStyle::Conditional {
                condition,
                style_true,
                style_false,
            } => match condition {
                Condition::ExitSuccess => {
                    if self.shell.exit_code().is_success() {
                        self.apply_style(style_true, bytes);
                    } else {
                        self.apply_style(style_false, bytes);
                    }
                }
            },
        }
    }

    fn compile_style(&self, style: &Style, bytes: &mut Vec<u8>) {
        match style {
            Style::Function { func, styling } => {
                for s in styling {
                    self.apply_style(s, bytes);
                }

                let string = match func {
                    Function::Directory => self.shell.pretty_pwd().unwrap_or_default(),
                };

                _ = queue!(bytes, style::Print(string), style::ResetColor);
            }
            Style::Text { string, styling } => {
                for s in styling {
                    self.apply_style(s, bytes);
                }

                _ = queue!(bytes, style::Print(string), style::ResetColor);
            }
            Style::Group { children } => {
                for child in children {
                    self.compile_style(child, bytes);
                }
            }
        }
    }

    fn generate_prompt(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.compile_style(&self.style, &mut bytes);
        _ = bytes.flush();
        bytes
    }

    fn render(&mut self) -> io::Result<()> {
        let prompt = self.generate_prompt();
        let cols = terminal::size()?.0 as usize - 1;
        let mut lines = Vec::new();

        for bytes in prompt.split(|b| *b == b'\n') {
            let mut trimmed_line = String::from_utf8(bytes.to_vec()).unwrap_or_default();

            while ansi_width(&trimmed_line) > cols {
                trimmed_line.pop();
            }

            lines.push(trimmed_line);
        }

        let last_line_index = lines.len() - 1;
        let last_line_width = ansi_width(&lines[last_line_index]);
        let editor_start = lines.len().saturating_sub(1) * cols + last_line_width;
        let editor_chars = self.editor.iter();
        let first_line_width = cols.saturating_sub(last_line_width);

        lines[last_line_index].extend(editor_chars.take(first_line_width));

        for line in editor_chars.skip(first_line_width).chunks(cols).into_iter() {
            lines.push(String::from_iter(line));
        }

        let cursor_index = editor_start + self.editor.cursor();
        let mut cursor_col = cursor_index % cols;
        let mut cursor_row = (cursor_index - cursor_col) / cols;

        if cursor_col == 0 && cursor_row > 0 && cursor_index > editor_start {
            cursor_col = cols;
            cursor_row -= 1;
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

        queue!(
            self.shell.stdout(),
            style::Print(last_line),
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
