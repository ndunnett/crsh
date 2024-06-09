use crate::builtin::Builtin;
use crate::shell::Shell;

pub struct Echo {
    message: String,
}

impl Builtin for Echo {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        Ok(Box::new(Self {
            message: args.join(" "),
        }))
    }

    fn run(&self, shell: &mut Shell) -> i32 {
        shell.println(&self.message);
        0
    }
}
