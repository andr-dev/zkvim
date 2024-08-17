use super::{normal::NormalVimEngine, VimEngine, VimEngineRunResult};
use crate::lines::Lines;
use crate::mode::{insert_mode_action, InsertModeAction};

#[derive(Debug, Default)]
pub struct InsertVimEngine {
    pub line: usize,
    pub col: usize,
}

impl VimEngine for InsertVimEngine {
    fn run<'a>(mut self: Box<Self>, input: &'a str, lines: &mut Lines) -> VimEngineRunResult<'a> {
        let (input, action) = insert_mode_action(input)?;

        match action {
            InsertModeAction::Char(char) => {
                lines.insert_char(self.line, self.col, char)?;

                self.col += 1;
            }
            InsertModeAction::Escape => return Ok((input, Box::new(NormalVimEngine::from(*self)))),
        }

        Ok((input, self))
    }
}

impl From<NormalVimEngine> for InsertVimEngine {
    fn from(value: NormalVimEngine) -> Self {
        Self {
            line: value.line,
            col: value.col,
        }
    }
}
