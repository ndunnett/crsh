#[derive(PartialEq)]
pub enum ShellMode {
    Interactive,
    Command,
    Script,
}

pub struct ShellConfig {
    pub start_path: String,
    pub args: Vec<String>,
    pub mode: ShellMode,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            start_path: "/".into(),
            args: Vec::new(),
            mode: ShellMode::Interactive,
        }
    }
}
