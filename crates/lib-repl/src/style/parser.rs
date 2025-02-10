use crossterm::style::Color;
use lib_core::ParsingIterator;

use crate::style::{
    scanner::{Scanner, Token},
    AppliedStyle, Condition, Function, Style,
};

impl Style {
    pub fn parse(input: &str) -> Self {
        Parser::new(input).parse()
    }
}

pub struct Parser<'source> {
    scanner: Scanner<'source>,
    next_char_value: Option<Token>,
    current_char_value: Option<Token>,
}

impl ParsingIterator for Parser<'_> {
    type PredicateItem = Token;
    type Item = Token;

    fn peek_item(&mut self) -> Option<&Self::Item> {
        self.next_char_value.as_ref()
    }

    fn next_item(&mut self) -> Option<&Self::Item> {
        self.current_char_value = self.next_char_value.take();
        self.next_char_value = self.scanner.next_token();
        self.current_char_value.as_ref()
    }

    fn check_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> bool {
        self.peek_item().cloned().is_some_and(f)
    }
}

impl<'source> Parser<'source> {
    pub fn new(input: &'source str) -> Self {
        let mut scanner = Scanner::new(input);
        let next_char_value = scanner.next_token();

        Self {
            scanner,
            next_char_value,
            current_char_value: None,
        }
    }

    pub fn parse(&mut self) -> Style {
        let mut children = Vec::new();

        while let Some(child) = self.primary() {
            children.push(child);
        }

        Style::Group { children }
    }

    fn primary(&mut self) -> Option<Style> {
        while let Some(t) = self.next_item() {
            if *t == Token::LeftBrace {
                return self.func();
            } else if *t == Token::LeftSquare {
                return self.text();
            } else {
                continue;
            }
        }

        None
    }

    fn func(&mut self) -> Option<Style> {
        if let Some(Token::Blob(string)) = self.next_item().cloned() {
            self.advance_if(|t| t == Token::RightBrace);

            match string.to_lowercase().as_str() {
                "directory" => Some(Style::Function {
                    func: Function::Directory,
                    styling: self.applied_styles(),
                }),
                _ => Some(Style::Text {
                    string: format!("!{string}!"),
                    styling: self.applied_styles(),
                }),
            }
        } else {
            None
        }
    }

    fn text(&mut self) -> Option<Style> {
        if let Some(Token::Quoted(string)) = self.next_item().cloned() {
            self.advance_if(|t| t == Token::RightSquare);

            Some(Style::Text {
                string,
                styling: self.applied_styles(),
            })
        } else {
            None
        }
    }

    fn applied_styles(&mut self) -> Vec<AppliedStyle> {
        let mut styles = Vec::new();

        if self.advance_if(|t| t == Token::LeftParen) {
            while let Some(style) = self.applied_style() {
                styles.push(style);
            }

            self.advance_if(|t| t == Token::RightParen);
        }

        styles
    }

    fn applied_style(&mut self) -> Option<AppliedStyle> {
        if let Some(Token::Blob(string)) = self.next_item().cloned() {
            match string.to_lowercase().as_str() {
                "green" => Some(AppliedStyle::Foreground(Color::Green)),
                "red" => Some(AppliedStyle::Foreground(Color::Red)),
                "dark_grey" | "dark_gray" => Some(AppliedStyle::Foreground(Color::DarkGrey)),
                "exit_success" => {
                    let condition = Condition::ExitSuccess;
                    self.next_if(|t| t == Token::Question)?;
                    let style_true = Box::new(self.applied_style()?);
                    self.next_if(|t| t == Token::Colon)?;
                    let style_false = Box::new(self.applied_style()?);

                    Some(AppliedStyle::Conditional {
                        condition,
                        style_true,
                        style_false,
                    })
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
