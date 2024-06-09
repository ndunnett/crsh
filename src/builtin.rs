mod cd;
mod echo;
mod exit;
mod pwd;
mod type_;
mod which;

use crate::shell::Shell;

pub trait Builtin {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String>
    where
        Self: Sized;
    fn run(&self, shell: &mut Shell) -> i32;
}

type BuilderOption = Option<Box<dyn Fn(&[&str]) -> Result<Box<dyn Builtin>, String>>>;

pub fn get_builder(keyword: &str) -> BuilderOption {
    match keyword {
        "cd" => Some(Box::new(cd::Cd::build)),
        "echo" => Some(Box::new(echo::Echo::build)),
        "exit" => Some(Box::new(exit::Exit::build)),
        "pwd" => Some(Box::new(pwd::Pwd::build)),
        "type" => Some(Box::new(type_::Type::build)),
        "which" => Some(Box::new(which::Which::build)),
        _ => None,
    }
}
