use ansi_term::Colour;

pub struct PromptStyle {
    pub colour_path: Colour,
    pub colour_success: Colour,
    pub colour_fail: Colour,
}

impl Default for PromptStyle {
    fn default() -> Self {
        Self {
            colour_path: Colour::Fixed(232),
            colour_success: Colour::Green,
            colour_fail: Colour::Red,
        }
    }
}

impl PromptStyle {
    pub fn new() -> Self {
        Default::default()
    }
}
