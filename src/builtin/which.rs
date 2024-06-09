use crate::builtin::{self, Builtin};
use crate::shell::Shell;

pub struct Which {
    keyword: String,
}

impl Builtin for Which {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        let keyword = args.first().map(|&s| s.to_string()).unwrap_or_default();
        Ok(Box::new(Self { keyword }))
    }

    fn run(&self, shell: &mut Shell) -> i32 {
        if builtin::get_builder(&self.keyword).is_some() {
            shell.println(format!("{}: shell builtin", self.keyword));
        } else if let Some(path) = shell.find_on_path(&self.keyword) {
            shell.println(path.display().to_string());
        } else {
            shell.println(format!("{} not found", self.keyword));
        }

        0
    }
}
