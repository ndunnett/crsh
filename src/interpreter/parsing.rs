use logos::{Lexer, Logos};

type ParseResult<T> = Result<T, String>;

#[derive(Debug, Copy, Clone, Logos)]
#[logos(skip r"\s+")]
enum Token<'a> {
    #[token("|", priority = 10)]
    Pipe,

    #[token("||", priority = 10)]
    Or,

    #[token("&&", priority = 10)]
    And,

    #[regex(r"[^\s\|&]+", priority = 1, callback = |lex| lex.slice())]
    Word(&'a str),
}

enum BindingPower {
    None,
    _Prefix(u8),
    _Postfix(u8),
    Infix(u8, u8),
}

impl<'a> Token<'a> {
    fn bp(&self) -> BindingPower {
        match self {
            Token::And => BindingPower::Infix(2, 2),
            Token::Or => BindingPower::Infix(2, 2),
            Token::Pipe => BindingPower::Infix(1, 1),
            _ => BindingPower::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command<'a> {
    Empty,
    Simple {
        keyword: &'a str,
        args: Vec<&'a str>,
    },
    And {
        left: Box<Command<'a>>,
        right: Box<Command<'a>>,
    },
    Or {
        left: Box<Command<'a>>,
        right: Box<Command<'a>>,
    },
    Pipeline {
        left: Box<Command<'a>>,
        right: Box<Command<'a>>,
    },
}

pub struct Parser<'a> {
    lexer: Lexer<'a, Token<'a>>,
    next: Option<Result<Token<'a>, ()>>,
    last: Option<Result<Token<'a>, ()>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Token::lexer(input);
        let next = lexer.next();

        Self {
            lexer,
            next,
            last: None,
        }
    }

    fn peek(&self) -> Option<Result<Token<'a>, ()>> {
        self.next
    }

    fn next(&mut self) -> Option<Result<Token<'a>, ()>> {
        self.last = self.next;
        self.next = self.lexer.next();
        self.last
    }

    pub fn parse(&mut self, bp: u8) -> ParseResult<Command<'a>> {
        let mut ast = Command::Empty;

        while let Some(Ok(token)) = self.peek() {
            match (&ast, token.bp()) {
                (Command::Empty, BindingPower::None) => {
                    ast = match token {
                        Token::Word(keyword) => {
                            self.next();
                            let mut args = Vec::new();

                            while let Some(Ok(Token::Word(arg))) = self.peek() {
                                args.push(arg);
                                self.next();
                            }

                            Command::Simple { keyword, args }
                        }
                        t => return Err(format!("bad token: {t:?}")),
                    };
                }
                (_, BindingPower::Infix(left_bp, right_bp)) => {
                    if left_bp < bp {
                        break;
                    }
                    self.next();

                    let left = Box::new(ast);
                    let right = Box::new(self.parse(right_bp)?);

                    ast = match token {
                        Token::And => Command::And { left, right },
                        Token::Or => Command::Or { left, right },
                        Token::Pipe => Command::Pipeline { left, right },
                        t => return Err(format!("bad token: {t:?}")),
                    };
                }
                _ => break,
            }
        }

        Ok(ast)
    }
}
