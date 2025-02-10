use crossterm::style::Color;

#[derive(Debug)]
pub enum AppliedStyle {
    Foreground(Color),
    Conditional {
        condition: Condition,
        style_true: Box<AppliedStyle>,
        style_false: Box<AppliedStyle>,
    },
}

#[derive(Debug)]
pub enum Function {
    Directory,
}

#[derive(Debug)]
pub enum Condition {
    ExitSuccess,
}

#[derive(Debug)]
pub enum Style {
    Function {
        func: Function,
        styling: Vec<AppliedStyle>,
    },
    Text {
        string: String,
        styling: Vec<AppliedStyle>,
    },
    Group {
        children: Vec<Style>,
    },
}

impl Default for Style {
    fn default() -> Self {
        Self::parse(
            r"
                {directory} (dark_grey)
                [' $ '] (exit_success ? green : red)
            ",
        )
    }
}
