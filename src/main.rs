mod builtin;
mod interpreter;
mod prompt;
mod system;

use crate::prompt::Prompt;

fn main() {
    let mut prompt = Prompt::new();
    prompt.interactive_loop()
}
