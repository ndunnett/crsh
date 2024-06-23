pub struct ShellConfig {
    pub start_path: String,
    pub args: Vec<String>,
    pub history_file: String,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            start_path: "/".into(),
            args: Vec::new(),
            history_file: ".crsh-history".into(),
        }
    }
}
