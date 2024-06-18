mod cd;
mod exit;
mod which;

pub trait Builtin {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String>
    where
        Self: Sized;
    fn run(&self, sh: &mut super::Shell) -> i32;
}

type BuilderOption = Option<Box<dyn Fn(&[&str]) -> Result<Box<dyn Builtin>, String>>>;

pub fn get_builder(keyword: &str) -> BuilderOption {
    match keyword {
        "cd" => Some(Box::new(cd::Cd::build)),
        "exit" => Some(Box::new(exit::Exit::build)),
        "which" => Some(Box::new(which::Which::build)),
        _ => None,
    }
}
