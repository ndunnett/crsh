mod builtin;
mod interpreter;
mod prompt;
mod shell;
mod system;

use crate::shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.interactive_loop()
}
