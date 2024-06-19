use logos::{Lexer, Logos};

type ParseResult<T> = Result<T, String>;

#[derive(Debug, Copy, Clone, Logos)]
#[logos(skip r"\s+")]
enum Token<'a> {
    #[token("|", priority = 10)]
    Pipe,

    #[token("||", priority = 10)]
    DoublePipe,

    #[token("&", priority = 10)]
    And,

    #[token("&&", priority = 10)]
    DoubleAnd,

    #[token(";", priority = 10)]
    Semicolon,

    #[regex(r"[^\s\|&;]+", priority = 1, callback = |lex| lex.slice())]
    Word(&'a str),
}

enum BindingPower {
    None,
    _Prefix(u8),
    _Postfix(u8),
    Infix(u8, u8),
}

impl<'a> Token<'a> {
    fn bp(&self, _is_prefix: bool) -> BindingPower {
        match self {
            Token::DoublePipe => BindingPower::Infix(7, 8),
            Token::DoubleAnd => BindingPower::Infix(5, 6),
            Token::Pipe => BindingPower::Infix(3, 4),
            Token::Semicolon => BindingPower::Infix(1, 2),
            _ => BindingPower::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command<'a> {
    Empty,
    Simple {
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
        cmds: Vec<Command<'a>>,
    },
    List {
        cmds: Vec<Command<'a>>,
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
        let mut is_prefix = true;

        while let Some(Ok(token)) = self.peek() {
            match (&ast, token.bp(is_prefix)) {
                (Command::Empty, BindingPower::None) => {
                    if matches!(token, Token::Word(_)) {
                        let args = std::iter::from_fn(|| {
                            if let Some(Ok(Token::Word(arg))) = self.peek() {
                                self.next();
                                Some(arg)
                            } else {
                                None
                            }
                        })
                        .collect();

                        ast = Command::Simple { args };
                    } else {
                        return Err(format!("bad token: {token:?}"));
                    }
                }
                // (Command::Empty, BindingPower::Prefix(right_bp)) => {
                //     self.next();
                //     let operand = Box::new(self.parse(right_bp)?);

                //     ast = match token {
                //         Token::_ => Command::_ { operand },
                //         t => return Err(format!("bad token: {t:?}")),
                //     };
                // }
                // (_, BindingPower::Postfix(left_bp)) => {
                //     if left_bp < bp {
                //         break;
                //     }

                //     self.next();
                //     let operand = Box::new(ast);

                //     ast = match token {
                //         Token::_ => Command::_ { operand },
                //         t => return Err(format!("bad token: {t:?}")),
                //     };
                // }
                (_, BindingPower::Infix(left_bp, right_bp)) => {
                    if left_bp < bp {
                        break;
                    }

                    self.next();

                    ast = match token {
                        Token::DoubleAnd => Command::And {
                            left: Box::new(ast),
                            right: Box::new(self.parse(right_bp)?),
                        },
                        Token::DoublePipe => Command::Or {
                            left: Box::new(ast),
                            right: Box::new(self.parse(right_bp)?),
                        },
                        Token::Pipe => {
                            let mut cmds = if let Command::Pipeline { cmds: next_cmds } = ast {
                                next_cmds
                            } else {
                                vec![ast]
                            };

                            cmds.push(self.parse(right_bp)?);
                            Command::Pipeline { cmds }
                        }
                        Token::Semicolon => {
                            let mut cmds = if let Command::List { cmds: next_cmds } = ast {
                                next_cmds
                            } else {
                                vec![ast]
                            };

                            cmds.push(self.parse(right_bp)?);
                            Command::List { cmds }
                        }
                        t => return Err(format!("bad token: {t:?}")),
                    };
                }
                _ => break,
            }

            is_prefix = false;
        }

        Ok(ast)
    }
}
