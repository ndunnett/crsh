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

    fn run(&self, sh: &mut Shell) -> i32 {
        sh.println(&self.message);
        0
    }
}
