use crossterm::style::Color;

pub struct Style {
    pub colour_path: Color,
    pub colour_success: Color,
    pub colour_fail: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            colour_path: Color::DarkGrey,
            colour_success: Color::Green,
            colour_fail: Color::Red,
        }
    }
}
