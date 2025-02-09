use crate::{
    error::{SourceError, SourceErrorVariant},
    parsing::{Command, Node, Parameter, ParsingIterator, Scanner, Token, TokenVariant, Word},
};

#[derive(Debug, Clone)]
pub enum ParseErrorVariant {
    IncompleteParse,
    UnexpectedTokens,
    UnmatchedParenthesis,
    UnmatchedBrace,
    InvalidName,
}

type Result<T> = std::result::Result<T, Vec<SourceError>>;

pub struct Parser<'source> {
    scanner: Scanner<'source>,
    errors: Vec<SourceError>,
    next_token_value: Option<Token>,
    current_token_value: Option<Token>,
}

// public interface
impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        let mut scanner = Scanner::new(source);
        let next_token = Some(scanner.next_token());

        Self {
            scanner,
            errors: Vec::new(),
            next_token_value: next_token,
            current_token_value: None,
        }
    }

    pub fn parse(mut self) -> Result<Node<'source>> {
        let output = self
            .list()
            .unwrap_or_else(|| Node::List { nodes: Vec::new() });

        self.recover(ParseErrorVariant::IncompleteParse, TokenVariant::EndOfInput);

        if self.errors.is_empty() {
            Ok(output)
        } else {
            Err(self.errors)
        }
    }
}

// iteration methods
impl ParsingIterator for Parser<'_> {
    type PredicateItem = TokenVariant;
    type Item = Token;

    fn peek_item(&mut self) -> Option<&Self::Item> {
        self.next_token_value.as_ref()
    }

    fn next_item(&mut self) -> Option<&Self::Item> {
        if self
            .current_token_value
            .as_ref()
            .is_some_and(|t| t.variant == TokenVariant::EndOfInput)
        {
            return None;
        }

        self.current_token_value = self.next_token_value.take();
        self.next_token_value = Some(self.scanner.next_token());

        if let Some(t) = self.current_token_value.clone() {
            println!(
                "{:?} -> {:?}",
                self.lexeme(t.span).unwrap_or_default(),
                t.variant
            );
        }

        self.current_token_value.as_ref()
    }

    fn check_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> bool {
        self.peek_item().is_some_and(|t| f(t.variant))
    }
}

// helper methods
impl<'source> Parser<'source> {
    fn recover(&mut self, error: ParseErrorVariant, token: TokenVariant) {
        self.scanner.reset_mode();
        let start = self.peek_item();

        if start.is_none_or(|t| t.variant == token) {
            return;
        }

        let start = start.cloned().unwrap();
        self.take_until(|t| t == token);
        let end = self.current_token_value.as_ref().unwrap_or(&start);
        let span = start.span.start..end.span.end;
        let lexeme = self.make_string(span.clone()).unwrap_or_default();

        self.errors.push(SourceError {
            variant: SourceErrorVariant::Parse(error),
            lexeme,
            span,
            line: start.line,
            column: start.column,
        });
    }

    fn lexeme(&self, span: std::ops::Range<usize>) -> Option<&'source str> {
        self.scanner.source().get(span)
    }

    fn make_string(&self, span: std::ops::Range<usize>) -> Option<String> {
        self.lexeme(span).map(String::from)
    }

    fn make_word(&self, span: std::ops::Range<usize>) -> Option<Word<'source>> {
        self.lexeme(span).map(Word::String)
    }

    fn check_name(lexeme: &str) -> bool {
        let mut chars = lexeme.chars();
        chars.next().is_some_and(|c| c.is_alphabetic() || c == '_')
            && chars.all(|c| c.is_alphanumeric() || c == '_')
    }
}

// parsing methods
impl<'source> Parser<'source> {
    fn subshell(&mut self) -> Option<Node<'source>> {
        if self.advance_if(|t| t == TokenVariant::LeftParen) {
            let node = self.subshell();

            if !self.advance_if(|t| t == TokenVariant::RightParen) {
                self.recover(
                    ParseErrorVariant::UnmatchedParenthesis,
                    TokenVariant::Newline,
                );
            }

            Some(Node::Subshell {
                node: Box::new(node?),
            })
        } else {
            self.list()
        }
    }

    fn list(&mut self) -> Option<Node<'source>> {
        let mut nodes = Vec::new();

        while let Some(node) = self.pipeline() {
            nodes.push(node);

            if !self.advance_if(|t| t == TokenVariant::Newline || t == TokenVariant::Semicolon) {
                break;
            }
        }

        match nodes.len() {
            0 => None,
            1 => nodes.into_iter().next(),
            _ => Some(Node::List { nodes }),
        }
    }

    fn pipeline(&mut self) -> Option<Node<'source>> {
        let mut nodes = Vec::new();

        while let Some(node) = self.logical_or() {
            nodes.push(node);

            if !self.advance_if(|t| t == TokenVariant::Bar) {
                break;
            }
        }

        match nodes.len() {
            0 => None,
            1 => nodes.into_iter().next(),
            _ => Some(Node::Pipeline { nodes }),
        }
    }

    fn logical_or(&mut self) -> Option<Node<'source>> {
        let left = self.logical_and()?;

        if self.advance_if(|t| t == TokenVariant::BarBar) {
            if let Some(right) = self.logical_and() {
                Some(Node::Or {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            } else {
                self.recover(ParseErrorVariant::UnexpectedTokens, TokenVariant::Newline);
                None
            }
        } else {
            Some(left)
        }
    }

    fn logical_and(&mut self) -> Option<Node<'source>> {
        let left = self.command()?;

        if self.advance_if(|t| t == TokenVariant::AmperAmper) {
            if let Some(right) = self.command() {
                Some(Node::And {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            } else {
                self.recover(ParseErrorVariant::UnexpectedTokens, TokenVariant::Newline);
                None
            }
        } else {
            Some(left)
        }
    }

    fn command(&mut self) -> Option<Node<'source>> {
        let name = Box::new(self.word()?);
        let mut args = Vec::new();

        while let Some(arg) = self.word() {
            args.push(arg);
        }

        Some(Node::Command {
            command: Command { name, args },
        })
    }

    fn word(&mut self) -> Option<Word<'source>> {
        if self.advance_if(|t| t == TokenVariant::Dollar) {
            self.parameter()
        } else if self.advance_if(|t| t == TokenVariant::DollarLeftBrace) {
            let word = self.parameter();

            if !self.advance_if(|t| t == TokenVariant::RightBrace) {
                self.recover(ParseErrorVariant::UnmatchedBrace, TokenVariant::Newline);
            }

            word
        } else if self.advance_if(|t| t == TokenVariant::DollarLeftParen) {
            let node = self.subshell()?;

            if !self.advance_if(|t| t == TokenVariant::RightParen) {
                self.recover(
                    ParseErrorVariant::UnmatchedParenthesis,
                    TokenVariant::Newline,
                );
            }

            Some(Word::Command {
                node: Box::new(node),
            })
        } else if let Some(tilde) = self.next_if(|t| t == TokenVariant::Tilde) {
            let mut words = Vec::new();
            let span = tilde.span.clone();

            match self.lexeme(span)? {
                "" => words.push(Word::Parameter(Parameter::MyHome)),
                lex => words.push(Word::Parameter(Parameter::OtherHome(Box::new(
                    Word::String(lex),
                )))),
            }

            if let Some(token) = self.next_if(|t| t == TokenVariant::Blob) {
                let span = token.span.clone();
                words.push(self.make_word(span)?);
            }

            match words.len() {
                0 => None,
                1 => words.into_iter().next(),
                _ => Some(Word::Compound { words }),
            }
        } else {
            let span = self.next_if(|t| t == TokenVariant::Blob)?.span.clone();
            self.make_word(span)
        }
    }

    fn parameter(&mut self) -> Option<Word<'source>> {
        let span = self.next_if(|t| t == TokenVariant::Blob)?.span.clone();
        let lexeme = self.lexeme(span)?;

        if let Ok(n) = lexeme.parse::<usize>() {
            Some(Word::Parameter(Parameter::Number(n)))
        } else if Self::check_name(lexeme) {
            Some(Word::Parameter(Parameter::String(lexeme)))
        } else {
            self.recover(ParseErrorVariant::UnexpectedTokens, TokenVariant::Newline);
            None
        }
    }
}
