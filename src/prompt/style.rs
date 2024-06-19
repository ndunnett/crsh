pub struct PromptStyle<'a> {
    pub path_decoration: &'a str,
    pub symbol_decoration: &'a str,
    pub colour_success: &'a str,
    pub colour_fail: &'a str,
}

impl Default for PromptStyle<'_> {
    fn default() -> Self {
        Self {
            path_decoration: "\x1b[2m",
            symbol_decoration: "\x1b[1m",
            colour_success: "\x1b[32m",
            colour_fail: "\x1b[31m",
        }
    }
}

impl PromptStyle<'_> {
    pub fn new() -> Self {
        Default::default()
    }
}
