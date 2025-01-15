use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    queue, terminal,
};
use itertools::Itertools;
use sysexits::ExitCode;

use crsh_core::Shell;

mod style;
use style::PromptStyle;

mod history;
use history::{HistorySource, PromptHistory};

enum Signal {
    Buffer(String),
    Interrupt,
    End,
}

pub struct Prompt<'a> {
    shell: &'a mut Shell,
    style: PromptStyle,
    history: PromptHistory,
    buffer: Vec<char>,
}

impl<'a> Prompt<'a> {
    pub fn new(shell: &'a mut Shell) -> Prompt<'a> {
        Prompt {
            shell,
            style: PromptStyle::new(),
            history: PromptHistory::default(),
            buffer: Vec::new(),
        }
    }

    pub fn with_history<S: Into<HistorySource>>(mut self, source: S) -> Prompt<'a> {
        self.history = PromptHistory::new(source.into());
        self
    }

    fn generate_prompt(&self) -> (String, usize) {
        let home = &self.shell.env.home.to_string_lossy().to_string();
        let mut pwd = self.shell.env.pwd.to_string_lossy().to_string();

        if pwd.starts_with(home) {
            pwd = pwd.replacen(home, "~", 1);
        }

        let indicator_colour = if self.shell.exit_code.is_success() {
            self.style.colour_success
        } else {
            self.style.colour_fail
        };

        let len = pwd.len() + self.shell.env.ps1.len() + 2;

        let prompt = format!(
            "{} {} ",
            self.style.colour_path.paint(pwd),
            indicator_colour.paint(&self.shell.env.ps1)
        );

        (prompt, len)
    }

    fn render_lines(&mut self) -> io::Result<Vec<String>> {
        let (prompt, prompt_len) = self.generate_prompt();
        let cols = terminal::size()?.0 as usize - 1;
        let first_len = prompt.len() + cols.saturating_sub(prompt_len);

        let chars = prompt.chars().chain(self.buffer.iter().cloned());
        let mut lines = vec![String::from_iter(chars.clone().take(first_len))];

        for line in chars.skip(first_len).chunks(cols).into_iter() {
            lines.push(String::from_iter(line));
        }

        Ok(lines)
    }

    fn read_line(&mut self) -> io::Result<Signal> {
        terminal::enable_raw_mode()?;
        let mut signal = Signal::End;
        let mut cursor_index = 0;

        queue!(
            self.shell.io.output,
            cursor::MoveToColumn(0),
            cursor::SavePosition,
            terminal::DisableLineWrap
        )?;

        loop {
            queue!(
                self.shell.io.output,
                cursor::RestorePosition,
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )?;

            let mut lines = self.render_lines()?.into_iter();
            let last_line = lines.next_back().unwrap_or_default();

            for line in lines {
                self.shell.io.println(line);
                queue!(self.shell.io.output, cursor::MoveToColumn(0))?;
            }

            self.shell.io.print(last_line);
            self.shell.io.output.flush()?;

            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(key) = event::read()? {
                    match (key.modifiers, key.code) {
                        (KeyModifiers::NONE, KeyCode::Home) => cursor_index = 0,
                        (KeyModifiers::NONE, KeyCode::End) => cursor_index = self.buffer.len(),
                        (KeyModifiers::NONE, KeyCode::Left) => {
                            cursor_index = cursor_index.saturating_sub(1);
                        }
                        (KeyModifiers::NONE, KeyCode::Right) => {
                            if cursor_index < self.buffer.len() {
                                cursor_index += 1;
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Up) => {
                            if let Some(buffer) = self.history.back() {
                                self.buffer = buffer;
                                cursor_index = self.buffer.len();
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Down) => {
                            if let Some(buffer) = self.history.forward() {
                                self.buffer = buffer;
                                cursor_index = self.buffer.len();
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Backspace) => {
                            if cursor_index > 0 {
                                self.buffer.remove(cursor_index - 1);
                                cursor_index -= 1;
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Delete) => {
                            if cursor_index < self.buffer.len() {
                                self.buffer.remove(cursor_index);
                            }
                        }
                        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(character)) => {
                            self.buffer.insert(cursor_index, character);
                            cursor_index += 1;
                        }
                        (KeyModifiers::NONE, KeyCode::Enter) => {
                            if !self.buffer.is_empty() {
                                self.history.push(self.buffer.clone());
                            }

                            signal = Signal::Buffer(String::from_iter(self.buffer.drain(..)));
                            break;
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                            signal = Signal::Interrupt;
                            break;
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                            break;
                        }
                        _ => {}
                    };
                }
            }
        }

        self.shell.io.println("");
        queue!(self.shell.io.output, cursor::MoveToColumn(0))?;
        self.shell.io.output.flush()?;
        terminal::disable_raw_mode()?;
        Ok(signal)
    }

    pub fn repl(&mut self) -> ExitCode {
        _ = ctrlc::set_handler(|| {});

        loop {
            match self.read_line() {
                Ok(Signal::Buffer(buffer)) => {
                    self.shell.interpret(&buffer);
                }
                Ok(Signal::Interrupt) => {
                    self.shell.io.println("^C");
                    self.shell.exit_code = ExitCode::DataErr;
                    continue;
                }
                Ok(Signal::End) => {
                    self.shell.io.println("^D");
                    self.shell.exit_code = ExitCode::DataErr;
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
