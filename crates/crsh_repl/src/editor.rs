use std::io;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::history::{History, Source};

pub enum Signal {
    None,
    Buffer(String),
    Interrupt,
    End,
}

#[derive(Default)]
pub struct Editor {
    buffer: Vec<char>,
    cursor: usize,
    history: History,
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = self.history.save();
    }
}

impl Editor {
    pub fn iter(&self) -> BufferIterator<'_> {
        BufferIterator {
            buffer: &self.buffer,
            index: 0,
        }
    }

    pub fn set_history<S: Into<Source>>(&mut self, source: S) {
        self.history = History::new(source.into());
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    fn take_buffer(&mut self) -> String {
        if !self.buffer.is_empty() {
            self.history.push(self.buffer.clone());
        }

        self.cursor = 0;
        String::from_iter(self.buffer.drain(..))
    }

    pub fn poll(&mut self) -> io::Result<Signal> {
        if let Event::Key(key) = event::read()? {
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Home) => self.cursor = 0,
                (KeyModifiers::NONE, KeyCode::End) => self.cursor = self.buffer.len(),
                (KeyModifiers::NONE, KeyCode::Left) => {
                    self.cursor = self.cursor.saturating_sub(1);
                }
                (KeyModifiers::NONE, KeyCode::Right) => {
                    if self.cursor < self.buffer.len() {
                        self.cursor += 1;
                    }
                }
                (KeyModifiers::NONE, KeyCode::Up) => {
                    if let Some(buffer) = self.history.back() {
                        self.buffer = buffer;
                        self.cursor = self.buffer.len();
                    }
                }
                (KeyModifiers::NONE, KeyCode::Down) => {
                    if let Some(buffer) = self.history.forward() {
                        self.buffer = buffer;
                        self.cursor = self.buffer.len();
                    }
                }
                (KeyModifiers::NONE, KeyCode::Backspace) => {
                    if self.cursor > 0 {
                        self.buffer.remove(self.cursor - 1);
                        self.cursor -= 1;
                    }
                }
                (KeyModifiers::NONE, KeyCode::Delete) => {
                    if self.cursor < self.buffer.len() {
                        self.buffer.remove(self.cursor);
                    }
                }
                (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(character)) => {
                    self.buffer.insert(self.cursor, character);
                    self.cursor += 1;
                }
                (KeyModifiers::NONE, KeyCode::Enter) => {
                    return Ok(Signal::Buffer(self.take_buffer()));
                }
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    return Ok(Signal::Interrupt);
                }
                (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                    return Ok(Signal::End);
                }
                _ => {}
            };
        }

        Ok(Signal::None)
    }
}

#[derive(Clone, Copy)]
pub struct BufferIterator<'a> {
    buffer: &'a [char],
    index: usize,
}

impl Iterator for BufferIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.buffer.len() {
            let next = self.buffer[self.index];
            self.index += 1;
            Some(next)
        } else {
            None
        }
    }
}
