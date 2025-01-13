use std::borrow::Cow::{self, Borrowed, Owned};

use ansi_term::Colour;
use rustyline::{
    completion::FilenameCompleter,
    highlight::{CmdKind, Highlighter, MatchingBracketHighlighter},
    hint::HistoryHinter,
    validate::MatchingBracketValidator,
    Completer, Helper, Hinter, Validator,
};

#[derive(Helper, Completer, Hinter, Validator)]
pub struct PromptHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    coloured_prompt: String,
}

impl PromptHelper {
    pub fn new() -> Self {
        Self {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter::new(),
            coloured_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        }
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.coloured_prompt = prompt;
    }
}

impl Highlighter for PromptHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.coloured_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(
            Colour::Fixed(232).prefix().to_string()
                + hint
                + &Colour::Fixed(232).suffix().to_string(),
        )
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}
