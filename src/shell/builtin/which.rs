use super::{get_builder, Builtin};
use crate::shell::Shell;

pub struct Which {
    keyword: String,
}

impl Builtin for Which {
    fn build(args: &[&str]) -> Result<Box<dyn Builtin>, String> {
        let keyword = args.first().map(|&s| s.to_string()).unwrap_or_default();
        Ok(Box::new(Self { keyword }))
    }

    fn run(&self, sh: &mut Shell) -> i32 {
        if get_builder(&self.keyword).is_some() {
            sh.println(format!("{}: shell builtin", self.keyword));
        } else if let Some(path) = sh.find_on_path(&self.keyword) {
            sh.println(path.display().to_string());
        } else {
            sh.println(format!("{} not found", self.keyword));
        }

        0
    }
}
