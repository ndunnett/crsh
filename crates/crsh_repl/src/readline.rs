use std::borrow::Cow;

use reedline::{
    Color, Prompt as ReedlinePrompt, PromptEditMode, PromptHistorySearch, PromptViMode,
};

pub use super::Prompt;

impl ReedlinePrompt for Prompt<'_> {
    fn render_prompt_left(&self) -> Cow<str> {
        let mut pwd = self.shell.env.pwd.to_string_lossy().to_string();
        let home = &self.shell.env.home.to_string_lossy().to_string();

        if pwd.starts_with(home) {
            pwd = pwd.replacen(home, "~", 1);
        }

        pwd.push(' ');
        pwd.into()
    }

    fn render_prompt_right(&self) -> Cow<str> {
        "".into()
    }

    fn render_prompt_indicator(&self, mode: PromptEditMode) -> Cow<str> {
        match mode {
            PromptEditMode::Default | PromptEditMode::Emacs => {
                format!("{} ", self.shell.env.ps1).into()
            }
            PromptEditMode::Vi(PromptViMode::Normal) => "> ".into(),
            PromptEditMode::Vi(PromptViMode::Insert) => ": ".into(),
            PromptEditMode::Custom(mode) => format!("({mode}) ").into(),
        }
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        format!("{} ", self.shell.env.ps2).into()
    }

    fn render_prompt_history_search_indicator(&self, search: PromptHistorySearch) -> Cow<str> {
        Cow::Owned(format!("(reverse-search: {}) ", search.term))
    }

    fn get_indicator_color(&self) -> Color {
        if self.shell.exit_code.is_success() {
            self.style.colour_success
        } else {
            self.style.colour_fail
        }
    }

    fn get_prompt_color(&self) -> Color {
        self.style.path_colour
    }
}
