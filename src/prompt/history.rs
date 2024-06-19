const MAX_HISTORY_SIZE: usize = 25;

#[derive(Default)]
pub struct PromptHistory {
    history: Vec<Vec<char>>,
    index: usize,
}

impl PromptHistory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn back(&mut self) -> Option<Vec<char>> {
        if self.index > 0 {
            self.index -= 1;
            Some(self.history[self.index].clone())
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<Vec<char>> {
        match self.history.len() - self.index {
            0 => None,
            1 => {
                self.index += 1;
                Some(Vec::new())
            }
            _ => {
                self.index += 1;
                Some(self.history[self.index].clone())
            }
        }
    }

    pub fn push(&mut self, buffer: Vec<char>) {
        if self.history.last() != Some(&buffer) {
            self.history.push(buffer.clone());

            if self.history.len() > MAX_HISTORY_SIZE {
                self.history.remove(0);
            }
        }

        self.index = self.history.len();
    }
}
