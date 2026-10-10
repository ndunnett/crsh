#[derive(Debug, Clone)]
pub enum Node<'source> {
    Command {
        command: Command<'source>,
    },
    Redirection {
        redirections: Vec<Redirection>,
        node: Box<Self>,
    },
    List {
        nodes: Vec<Self>,
    },
    Pipeline {
        nodes: Vec<Self>,
    },
    Subshell {
        node: Box<Self>,
    },
    Or {
        left: Box<Self>,
        right: Box<Self>,
    },
    And {
        left: Box<Self>,
        right: Box<Self>,
    },
    While {
        predicate: Box<Self>,
        body: Box<Self>,
    },
    Until {
        predicate: Box<Self>,
        body: Box<Self>,
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
    Compound { words: Vec<Self> },
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
