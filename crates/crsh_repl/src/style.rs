use reedline::Color;

pub struct PromptStyle {
    pub path_colour: Color,
    pub colour_success: Color,
    pub colour_fail: Color,
}

impl Default for PromptStyle {
    fn default() -> Self {
        Self {
            path_colour: Color::DarkGrey,
            colour_success: Color::Green,
            colour_fail: Color::Red,
        }
    }
}

impl PromptStyle {
    pub fn new() -> Self {
        Default::default()
    }
}
