use std::{collections::HashMap, str::Chars};

use crate::parsing::{ParsingIterator, Token, TokenVariant};

#[derive(Debug, Clone, PartialEq)]
enum ScanMode {
    Subshell,
    DoubleQuotes,
    BackQuotes,
    ReserveWord,
    Braces,
    FunctionBody, // todo
    Assignment,   // todo
}

#[derive(Debug, Clone)]
pub struct Scanner<'source> {
    source: &'source str,
    chars: Chars<'source>,
    next_char_value: Option<char>,
    current_char_value: Option<char>,
    mode_stack: Vec<ScanMode>,
    token_start: usize,
    token_end: usize,
    line: usize,
    column: usize,
    reserved_words: HashMap<&'static str, TokenVariant>,
}

// public interface
impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        let mut chars = source.chars();
        let next_char_value = chars.next();

        Self {
            source,
            chars,
            next_char_value,
            current_char_value: None,
            mode_stack: Vec::new(),
            token_start: 0,
            token_end: 0,
            line: 1,
            column: 1,
            reserved_words: Self::RESERVED_WORDS.into_iter().collect(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.mode_stack.last() {
            Some(ScanMode::DoubleQuotes) => self.double_quotes(),
            Some(ScanMode::BackQuotes) => self.back_quotes(),
            Some(ScanMode::Braces) => self.braces(),
            _ => self.root(),
        }
    }

    pub fn source(&self) -> &'source str {
        self.source
    }

    pub fn reset_mode(&mut self) {
        self.mode_stack.clear();
    }
}

// iteration methods
impl ParsingIterator for Scanner<'_> {
    type PredicateItem = char;
    type Item = char;

    fn peek_item(&mut self) -> Option<&Self::Item> {
        self.next_char_value.as_ref()
    }

    fn next_item(&mut self) -> Option<&Self::Item> {
        self.token_end += 1;

        if self.current_char_value.is_some_and(|c| c == '\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        self.current_char_value = self.next_char_value;
        self.next_char_value = self.chars.next();
        self.current_char_value.as_ref()
    }

    fn check_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> bool {
        self.peek_item().copied().is_some_and(f)
    }
}

// helper methods
impl Scanner<'_> {
    const RESERVED_WORDS: [(&'static str, TokenVariant); 22] = [
        ("namespace", TokenVariant::Namespace),
        ("function", TokenVariant::Function),
        ("select", TokenVariant::Select),
        ("until", TokenVariant::Until),
        ("while", TokenVariant::While),
        ("case", TokenVariant::Case),
        ("done", TokenVariant::Done),
        ("elif", TokenVariant::Elif),
        ("else", TokenVariant::Else),
        ("esac", TokenVariant::Esac),
        ("then", TokenVariant::Then),
        ("time", TokenVariant::Time),
        ("for", TokenVariant::For),
        ("do", TokenVariant::Do),
        ("fi", TokenVariant::Fi),
        ("if", TokenVariant::If),
        ("in", TokenVariant::In),
        ("[[", TokenVariant::DoubleLeftSquare),
        ("]]", TokenVariant::DoubleRightSquare),
        ("{", TokenVariant::LeftBrace),
        ("}", TokenVariant::RightBrace),
        ("!", TokenVariant::Bang),
    ];

    const META_CHARS: &'static str = "|&;()<>";
    const DOUBLE_QUOTED_CHARS: &'static str = "$`\"\\";

    fn delimit_token(&mut self, variant: TokenVariant) -> Token {
        let span = self.token_start..self.token_end;
        let line = self.line;
        let column = self.column + self.token_start - self.token_end;
        self.token_start = self.token_end;

        Token {
            variant,
            span,
            line,
            column,
        }
    }
}

// parsing methods
impl Scanner<'_> {
    fn root(&mut self) -> Token {
        while let Some(c) = self.next_item() {
            match c {
                '"' => {
                    self.token_start = self.token_end;
                    self.mode_stack.push(ScanMode::DoubleQuotes);
                    return self.double_quotes();
                }
                '(' => {
                    self.mode_stack.push(ScanMode::Subshell);
                    return self.delimit_token(TokenVariant::LeftParen);
                }
                ')' => {
                    self.mode_stack.pop();
                    return self.delimit_token(TokenVariant::RightParen);
                }
                '#' => {
                    self.comment();
                    continue;
                }
                '\n' => return self.delimit_token(TokenVariant::Newline),
                ';' => return self.delimit_token(TokenVariant::Semicolon),
                '`' => return self.back_quotes(),
                '\'' => return self.single_quotes(),
                '~' => return self.tilde(),
                '$' => return self.dollar(),
                '&' => return self.ampersand(),
                '|' => return self.bar(),
                c if c.is_whitespace() => {
                    self.token_start = self.token_end;
                    continue;
                }
                _ => return self.blob(),
            }
        }

        self.delimit_token(TokenVariant::EndOfInput)
    }

    fn double_quotes(&mut self) -> Token {
        while let Some(c) = self.next_item() {
            match c {
                '$' => return self.dollar(),
                '"' => {
                    self.mode_stack.pop();
                    self.token_start = self.token_end;
                    return self.next_token();
                }
                '`' => {
                    self.mode_stack.push(ScanMode::BackQuotes);
                    return self.delimit_token(TokenVariant::BackQuote);
                }
                '\\' => todo!("implement character escapes"),
                c if c.is_whitespace() => {
                    self.token_start = self.token_end;
                    continue;
                }
                _ => {
                    self.take_until(|c| Self::DOUBLE_QUOTED_CHARS.contains(c));
                    return self.delimit_token(TokenVariant::Blob);
                }
            }
        }

        self.delimit_token(TokenVariant::EndOfInput)
    }

    fn back_quotes(&mut self) -> Token {
        self.token_start = self.token_end;
        self.mode_stack.push(ScanMode::BackQuotes);

        todo!("implement back quotes")
    }

    fn single_quotes(&mut self) -> Token {
        self.token_start = self.token_end;
        self.take_until(|c| c == '\'');
        let token = self.delimit_token(TokenVariant::Blob);
        self.next_item();
        token
    }

    fn tilde(&mut self) -> Token {
        self.token_start = self.token_end;
        self.take_until(|c| c == '/' || c == ';' || c.is_whitespace());
        self.delimit_token(TokenVariant::Tilde)
    }

    fn dollar(&mut self) -> Token {
        if self.advance_if(|c| c == '{') {
            self.mode_stack.push(ScanMode::Braces);
            self.delimit_token(TokenVariant::DollarLeftBrace)
        } else if self.advance_if(|c| c == '(') {
            self.mode_stack.push(ScanMode::Subshell);
            self.delimit_token(TokenVariant::DollarLeftParen)
        } else {
            self.delimit_token(TokenVariant::Dollar)
        }
    }

    fn braces(&mut self) -> Token {
        if self.advance_if(|c| c == '}') {
            self.mode_stack.pop();
            self.delimit_token(TokenVariant::RightBrace)
        } else {
            self.take_until(|c| c == '}');
            self.delimit_token(TokenVariant::Blob)
        }
    }

    fn ampersand(&mut self) -> Token {
        if self.advance_if(|c| c == '&') {
            self.delimit_token(TokenVariant::AmperAmper)
        } else {
            self.delimit_token(TokenVariant::Ampersand)
        }
    }

    fn bar(&mut self) -> Token {
        if self.advance_if(|c| c == '|') {
            self.delimit_token(TokenVariant::BarBar)
        } else {
            self.delimit_token(TokenVariant::Bar)
        }
    }

    fn comment(&mut self) {
        self.take_until(|c| c == '\n');
        self.token_start = self.token_end;
    }

    fn blob(&mut self) -> Token {
        self.take_until(|c| c.is_whitespace() || Self::META_CHARS.contains(c));

        if self.mode_stack.last() == Some(&ScanMode::ReserveWord)
            && let Some(token) = self.reserved_word()
        {
            return token;
        }

        self.delimit_token(TokenVariant::Blob)
    }

    fn reserved_word(&mut self) -> Option<Token> {
        let lexeme = self.source.get(self.token_start..self.token_end)?;
        let variant = *self.reserved_words.get(lexeme)?;
        Some(self.delimit_token(variant))
    }
}
