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

use crate::interpreter::Executable;

type BuiltinOption = Option<Box<dyn Fn(&[&str]) -> Box<dyn Executable>>>;

pub fn get(keyword: &str) -> BuiltinOption {
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

pub fn build(keyword: &str, args: &[&str]) -> Option<Box<dyn Executable>> {
    if let Some(builder) = get(keyword) {
        Some(builder(args))
    } else {
        None
    }
}
