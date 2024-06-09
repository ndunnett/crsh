use std::cmp::Ordering;
use std::io::{self, Write};
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyModifiers};
use crossterm::{cursor, queue, terminal};

use crate::interpreter;
use crate::shell::Shell;

struct PromptStyle<'a> {
    path_decoration: &'a str,
    symbol_decoration: &'a str,
    colour_success: &'a str,
    colour_fail: &'a str,
}

impl Default for PromptStyle<'_> {
    fn default() -> Self {
        Self {
            path_decoration: "\x1b[2m",
            symbol_decoration: "\x1b[1m",
            colour_success: "\x1b[32m",
            colour_fail: "\x1b[31m",
        }
    }
}

enum PromptCapture {
    String(String),
    Kill,
    End,
    Suspend,
}

pub struct Prompt<'a> {
    shell: &'a mut Shell,
    style: PromptStyle<'a>,
    history: Vec<Vec<char>>,
}

impl<'a> Prompt<'a> {
    pub fn new(shell: &'a mut Shell) -> Self {
        Self {
            shell,
            style: Default::default(),
            history: Default::default(),
        }
    }

    pub fn interactive_loop(&mut self) -> Result<(), ()> {
        loop {
            match self.readline() {
                Ok(PromptCapture::String(input)) => {
                    self.shell.exit_code = interpreter::interpret(self.shell, &input);
                }
                Ok(PromptCapture::Kill) => {
                    // todo: ctrl-c unimplemented
                    continue;
                }
                Ok(PromptCapture::End) => {
                    break;
                }
                Ok(PromptCapture::Suspend) => {
                    // todo: ctrl-z unimplemented
                    continue;
                }
                Err(e) => {
                    self.shell.eprintln(format!("crsh: {e}"));
                    self.shell.exit_code = -1;
                }
            }
        }

        Ok(())
    }

    fn readline(&mut self) -> Result<PromptCapture, io::Error> {
        terminal::enable_raw_mode()?;
        queue!(self.shell.output, terminal::EnableLineWrap)?;
        let mut buffer: Vec<char> = Vec::new();
        let mut cursor_index: usize = 0;
        let mut history_offset: usize = 0;
        let mut output_rows = 0;
        let mut taken_rows = output_rows;
        let mut last_output_rows = output_rows;

        loop {
            let (cols, rows) = terminal::size()?;

            if rows > 0 {
                let ps1 = self.prompt();
                let buffer_string = buffer.iter().collect::<String>();
                let output = format!("{ps1}{buffer_string}");

                output_rows = (strip_ansi_escapes::strip_str(&output)
                    .len()
                    .div_ceil(cols as usize)) as u16
                    - 1;

                match last_output_rows.cmp(&output_rows) {
                    Ordering::Less => {
                        if output_rows > taken_rows {
                            queue!(
                                self.shell.output,
                                terminal::ScrollUp(output_rows - last_output_rows)
                            )?;

                            taken_rows = output_rows;
                        } else {
                            queue!(
                                self.shell.output,
                                cursor::MoveToNextLine(output_rows - last_output_rows)
                            )?;
                        }

                        last_output_rows = output_rows;
                    }
                    Ordering::Greater => {
                        queue!(
                            self.shell.output,
                            cursor::MoveToPreviousLine(last_output_rows - output_rows)
                        )?;

                        last_output_rows = output_rows;
                    }
                    Ordering::Equal => {}
                }

                if output_rows > 0 {
                    queue!(
                        self.shell.output,
                        cursor::MoveToPreviousLine(output_rows),
                        terminal::Clear(terminal::ClearType::FromCursorDown)
                    )?;
                } else {
                    queue!(
                        self.shell.output,
                        cursor::MoveToColumn(0),
                        terminal::Clear(terminal::ClearType::FromCursorDown)
                    )?;
                }

                print!("{output}");
            }

            self.shell.output.flush()?;

            if event::poll(Duration::from_millis(500))? {
                if let event::Event::Key(key) = event::read()? {
                    match (key.modifiers, key.code) {
                        (KeyModifiers::NONE, KeyCode::Home) => cursor_index = 0,
                        (KeyModifiers::NONE, KeyCode::End) => cursor_index = buffer.len(),
                        (KeyModifiers::NONE, KeyCode::Left) => {
                            cursor_index = cursor_index.saturating_sub(1)
                        }
                        (KeyModifiers::NONE, KeyCode::Right) => {
                            if cursor_index < buffer.len() {
                                cursor_index += 1;
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Up) => {
                            if history_offset < self.history.len() {
                                history_offset += 1;
                                buffer
                                    .clone_from(&self.history[self.history.len() - history_offset]);
                                cursor_index = buffer.len();
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Down) => {
                            if history_offset > 0 {
                                history_offset -= 1;

                                if history_offset > 0 {
                                    buffer.clone_from(
                                        &self.history[self.history.len() - history_offset],
                                    );
                                    cursor_index = buffer.len();
                                } else {
                                    buffer.clear();
                                    cursor_index = 0;
                                }
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Backspace) => {
                            if cursor_index > 0 {
                                buffer.remove(cursor_index - 1);
                                cursor_index -= 1;
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Delete) => {
                            if cursor_index != buffer.len() {
                                buffer.remove(cursor_index);
                            }
                        }
                        (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(character)) => {
                            buffer.insert(cursor_index, character);
                            cursor_index += 1;
                        }
                        (KeyModifiers::NONE, KeyCode::Enter) => {
                            break;
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                            print!("^C");
                            self.post_read()?;
                            self.shell.exit_code = -1;
                            return Ok(PromptCapture::Kill);
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                            print!("^D");
                            self.post_read()?;
                            self.shell.exit_code = -1;
                            return Ok(PromptCapture::End);
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('z')) => {
                            print!("^Z");
                            self.post_read()?;
                            self.shell.exit_code = -1;
                            return Ok(PromptCapture::Suspend);
                        }
                        _ => (),
                    };
                }
            }
        }

        self.post_read()?;

        if !buffer.is_empty() && self.history.last() != Some(&buffer) {
            self.history.push(buffer.clone());
        }

        Ok(PromptCapture::String(String::from_iter(buffer)))
    }

    fn post_read(&mut self) -> Result<(), io::Error> {
        println!();
        queue!(self.shell.output, cursor::MoveToColumn(0))?;
        self.shell.output.flush()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn prompt(&self) -> String {
        let pwd = &self.shell.common_env.pwd;
        let home = &self.shell.common_env.home;

        let current_dir = if pwd.starts_with(home) {
            pwd.replacen(home, "~", 1)
        } else {
            pwd.clone()
        };

        let colour = match self.shell.exit_code {
            0 => self.style.colour_success,
            _ => self.style.colour_fail,
        };

        format!(
            "{}{}\x1b[m {}{}{}\x1b[m ",
            self.style.path_decoration,
            current_dir,
            self.style.symbol_decoration,
            colour,
            self.shell.common_env.ps1
        )
    }
}
