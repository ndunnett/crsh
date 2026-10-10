#[derive(Debug, Clone)]
pub enum Node<'source> {
    Command {
        command: Command<'source>,
    },
    Redirection {
        redirections: Vec<Redirection>,
        node: Box<Node<'source>>,
    },
    List {
        nodes: Vec<Node<'source>>,
    },
    Pipeline {
        nodes: Vec<Node<'source>>,
    },
    Subshell {
        node: Box<Node<'source>>,
    },
    Or {
        left: Box<Node<'source>>,
        right: Box<Node<'source>>,
    },
    And {
        left: Box<Node<'source>>,
        right: Box<Node<'source>>,
    },
    While {
        predicate: Box<Node<'source>>,
        body: Box<Node<'source>>,
    },
    Until {
        predicate: Box<Node<'source>>,
        body: Box<Node<'source>>,
    },
    Arithmetic,  // todo
    Conditional, // todo
    For,         // todo
    Case,        // todo
    Function,    // todo
    Coproc,      // todo
    If,          // todo
    Group,       // todo
    Select,      // todo
    Timespec,    // todo
}

#[derive(Debug, Clone)]
pub enum Word<'source> {
    String(&'source str),
    Parameter(Parameter<'source>),
    Command { node: Box<Node<'source>> },
    Compound { words: Vec<Word<'source>> },
}

#[derive(Debug, Clone)]
pub enum Parameter<'source> {
    Number(usize),
    String(&'source str),
    MyHome,
    OtherHome(Box<Word<'source>>),
}

#[derive(Debug, Clone)]
pub struct Command<'source> {
    pub name: Box<Word<'source>>,
    pub args: Vec<Word<'source>>,
}

#[derive(Debug, Clone)]
pub struct Redirection;
