use std::str::Chars;

use lib_core::ParsingIterator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Quoted(String),
    Blob(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Question,
    Colon,
}

#[derive(Debug, Clone)]
pub struct Scanner<'source> {
    source: &'source str,
    chars: Chars<'source>,
    next_char_value: Option<char>,
    current_char_value: Option<char>,
    token_start: usize,
    token_end: usize,
}

impl ParsingIterator for Scanner<'_> {
    type PredicateItem = char;
    type Item = char;

    fn peek_item(&mut self) -> Option<&Self::Item> {
        self.next_char_value.as_ref()
    }

    fn next_item(&mut self) -> Option<&Self::Item> {
        self.token_end += 1;
        self.current_char_value = self.next_char_value;
        self.next_char_value = self.chars.next();
        self.current_char_value.as_ref()
    }

    fn check_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> bool {
        self.peek_item().copied().is_some_and(f)
    }
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let mut chars = source.chars();
        let next_char_value = chars.next();

        Self {
            source,
            chars,
            next_char_value,
            current_char_value: None,
            token_start: 0,
            token_end: 0,
        }
    }

    const PUNCTUATION: &'static str = "{}()[]?:";

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.next_item() {
            match c {
                '{' => {
                    self.token_start = self.token_end;
                    return Some(Token::LeftBrace);
                }
                '}' => {
                    self.token_start = self.token_end;
                    return Some(Token::RightBrace);
                }
                '(' => {
                    self.token_start = self.token_end;
                    return Some(Token::LeftParen);
                }
                ')' => {
                    self.token_start = self.token_end;
                    return Some(Token::RightParen);
                }
                '[' => {
                    self.token_start = self.token_end;
                    return Some(Token::LeftSquare);
                }
                ']' => {
                    self.token_start = self.token_end;
                    return Some(Token::RightSquare);
                }
                '?' => {
                    self.token_start = self.token_end;
                    return Some(Token::Question);
                }
                ':' => {
                    self.token_start = self.token_end;
                    return Some(Token::Colon);
                }
                '"' | '\'' | '`' => {
                    let quote_char = *c;
                    self.token_start = self.token_end;
                    self.take_until(|c| c == quote_char);
                    let span = self.token_start..self.token_end;
                    let lexeme = self.source.get(span).unwrap_or_default();
                    self.next_item();
                    return Some(Token::Quoted(lexeme.replace("\\n", "\n")));
                }
                c if c.is_whitespace() => {
                    self.token_start = self.token_end;
                    continue;
                }
                _ => {
                    self.take_until(|c| c.is_whitespace() || Self::PUNCTUATION.contains(c));
                    let span = self.token_start..self.token_end;
                    self.token_start = self.token_end;

                    return Some(Token::Blob(
                        self.source.get(span).unwrap_or_default().into(),
                    ));
                }
            }
        }

        None
    }
}
