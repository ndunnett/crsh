mod cd;
mod echo;
mod exit;
mod pwd;
mod type_;
mod which;

use cd::Cd;
use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use type_::Type;
use which::Which;

use crate::interpreter::ExecutionContext;

pub trait Builtin {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String>
    where
        Self: Sized;
    fn run(&self, ctx: ExecutionContext) -> Result<(), ()>;
}

type BuilderOption = Option<Box<dyn Fn(&[&str]) -> Result<Box<dyn Builtin>, String>>>;

pub fn get_builder(keyword: &str) -> BuilderOption {
    match keyword {
        "cd" => Some(Box::new(Cd::build)),
        "echo" => Some(Box::new(Echo::build)),
        "exit" => Some(Box::new(Exit::build)),
        "pwd" => Some(Box::new(Pwd::build)),
        "type" => Some(Box::new(Type::build)),
        "which" => Some(Box::new(Which::build)),
        _ => None,
    }
}
