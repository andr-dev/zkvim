use crate::lines::Lines;

use super::{error::VimEngineResult, normal::NormalVimEngine, VimEngine};

pub struct VimEngineRunner {
    lines: Lines,
}

impl VimEngineRunner {
    pub fn new(text: String) -> Self {
        Self {
            lines: Lines::new(text),
        }
    }

    pub fn run(&mut self, input: String) -> VimEngineResult {
        let mut state: Box<dyn VimEngine> = Box::new(NormalVimEngine::default());

        let mut input = input.as_str();

        while !input.is_empty() {
            (input, state) = state.run(input, &mut self.lines)?;
        }

        Ok(self.lines.to_string())
    }
}
