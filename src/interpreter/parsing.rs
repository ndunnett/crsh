use logos::{Lexer, Logos, Span};

type ParseResult<T> = Result<T, (String, Span)>;

#[derive(Debug, Logos)]
#[logos(skip r"\s+")]
enum Token<'a> {
    #[token("|", priority = 10)]
    Pipe,

    #[token("||", priority = 10)]
    Or,

    #[token("&&", priority = 10)]
    And,

    #[regex(r"[^\s]+", priority = 1, callback = |lex| lex.slice())]
    Word(&'a str),
}

#[derive(Debug)]
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

fn parse_command<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> ParseResult<Command<'a>> {
    let mut command = Command::Empty;

    while let Some(token) = lexer.next() {
        match (token, &mut command) {
            // simple
            (Ok(Token::Word(s)), Command::Empty) => {
                command = Command::Simple {
                    keyword: s,
                    args: Vec::new(),
                };
            }
            (Ok(Token::Word(s)), Command::Simple { args, .. }) => {
                args.push(s);
            }
            // pipeline
            (Ok(Token::Pipe), _) => {
                command = Command::Pipeline {
                    left: Box::new(command),
                    right: Box::new(parse_command(lexer)?),
                };
            }
            // and
            (Ok(Token::And), _) => {
                let rem = parse_command(lexer)?;

                match rem {
                    Command::Pipeline { left, right } => {
                        command = Command::Pipeline {
                            left: Box::new(Command::And {
                                left: Box::new(command),
                                right: left,
                            }),
                            right,
                        };
                    }
                    _ => {
                        command = Command::And {
                            left: Box::new(command),
                            right: Box::new(rem),
                        };
                    }
                }
            }
            // or
            (Ok(Token::Or), _) => {
                let rem = parse_command(lexer)?;

                match rem {
                    Command::Pipeline { left, right } => {
                        command = Command::Pipeline {
                            left: Box::new(Command::Or {
                                left: Box::new(command),
                                right: left,
                            }),
                            right,
                        };
                    }
                    _ => {
                        command = Command::Or {
                            left: Box::new(command),
                            right: Box::new(rem),
                        };
                    }
                }
            }
            // error
            (Ok(token), _) => {
                return Err((
                    format!("unexpected \"{token:?}\" here").to_owned(),
                    lexer.span(),
                ));
            }
            (Err(e), _) => return Err((format!("error: {e:?}").to_owned(), lexer.span())),
        }
    }

    Ok(command)
}

pub fn parse(input: &str) -> Result<Command, String> {
    match parse_command(&mut Token::lexer(input)) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(format!("{}: {:?}", e.0, e.1)),
    }
}
