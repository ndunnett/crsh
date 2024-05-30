mod builtin;
mod call;
mod shell;
mod system;

use crate::shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.main_loop()
}
