use std::cmp::Ordering;
use std::io::{self, Write};
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyModifiers};
use crossterm::{cursor, queue, terminal};

mod history;
mod style;

use history::*;
use style::*;

use crate::shell::Shell;

enum PromptCapture {
    String(String),
    Kill,
    End,
    Suspend,
}

pub struct Prompt<'a> {
    shell: &'a mut Shell,
    style: PromptStyle<'a>,
    history: PromptHistory,
}

impl<'a> Prompt<'a> {
    pub fn new(shell: &'a mut Shell) -> Self {
        Self {
            shell,
            style: PromptStyle::new(),
            history: PromptHistory::new(),
        }
    }

    pub fn interactive_loop(&mut self) -> Result<(), ()> {
        loop {
            match self.readline() {
                Ok(PromptCapture::String(input)) => {
                    self.shell.interpret(&input);
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
                    self.shell.io.eprintln(format!("crsh: {e}"));
                    self.shell.exit_code = -1;
                }
            }
        }

        Ok(())
    }

    fn prompt(&self) -> String {
        let pwd = &self.shell.env.pwd;
        let home = &self.shell.env.home;

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
            self.shell.env.ps1
        )
    }

    fn post_read(&mut self) -> Result<(), io::Error> {
        println!();
        queue!(self.shell.io.output, cursor::MoveToColumn(0))?;
        self.shell.io.output.flush()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn readline(&mut self) -> Result<PromptCapture, io::Error> {
        terminal::enable_raw_mode()?;
        queue!(self.shell.io.output, terminal::EnableLineWrap)?;
        let mut buffer: Vec<char> = Vec::new();
        let mut cursor_index: usize = 0;
        let mut output_rows = 0;
        let mut taken_rows = output_rows;
        let mut last_output_rows = output_rows;

        loop {
            let (cols, rows) = terminal::size()?;

            if rows > 0 {
                let ps1 = self.prompt();
                let ps1_len = strip_ansi_escapes::strip_str(&ps1).len();
                output_rows = ((ps1_len + buffer.len()).div_ceil(cols as usize)) as u16 - 1;

                match last_output_rows.cmp(&output_rows) {
                    Ordering::Less => {
                        if output_rows > taken_rows {
                            queue!(
                                self.shell.io.output,
                                terminal::ScrollUp(output_rows - last_output_rows)
                            )?;

                            taken_rows = output_rows;
                        } else {
                            queue!(
                                self.shell.io.output,
                                cursor::MoveToNextLine(output_rows - last_output_rows)
                            )?;
                        }

                        last_output_rows = output_rows;
                    }
                    Ordering::Greater => {
                        queue!(
                            self.shell.io.output,
                            cursor::MoveToPreviousLine(last_output_rows - output_rows)
                        )?;

                        last_output_rows = output_rows;
                    }
                    Ordering::Equal => {}
                }

                if output_rows > 0 {
                    queue!(
                        self.shell.io.output,
                        cursor::MoveToPreviousLine(output_rows),
                        terminal::Clear(terminal::ClearType::FromCursorDown)
                    )?;
                } else {
                    queue!(
                        self.shell.io.output,
                        cursor::MoveToColumn(0),
                        terminal::Clear(terminal::ClearType::FromCursorDown)
                    )?;
                }

                print!("{ps1}{}", buffer.iter().collect::<String>());

                queue!(
                    self.shell.io.output,
                    cursor::MoveToColumn((ps1_len + cursor_index) as u16 % cols),
                )?;
            }

            self.shell.io.output.flush()?;

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
                            if let Some(buf) = self.history.back() {
                                buffer = buf;
                                cursor_index = buffer.len();
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Down) => {
                            if let Some(buf) = self.history.forward() {
                                buffer = buf;
                                cursor_index = buffer.len();
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Backspace) => {
                            if cursor_index > 0 {
                                buffer.remove(cursor_index - 1);
                                cursor_index -= 1;
                            }
                        }
                        (KeyModifiers::NONE, KeyCode::Delete) => {
                            if cursor_index < buffer.len() {
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

        if !buffer.is_empty() {
            self.history.push(buffer.clone());
        }

        Ok(PromptCapture::String(String::from_iter(buffer)))
    }
}
