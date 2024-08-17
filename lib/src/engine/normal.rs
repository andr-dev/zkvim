use crate::{
    lines::Lines,
    mode::{normal_mode_action, NormalModeAction, VimCommand},
};

use super::{insert::InsertVimEngine, visual::VisualVimEngine, VimEngine, VimEngineRunResult};

#[derive(Debug, Default)]
pub struct NormalVimEngine {
    pub line: usize,
    pub col: usize,
}

impl VimEngine for NormalVimEngine {
    fn run<'a>(mut self: Box<Self>, input: &'a str, lines: &mut Lines) -> VimEngineRunResult<'a> {
        let (input, actions) = normal_mode_action(input)?;

        for action in actions.into_vec().into_iter() {
            match action {
                NormalModeAction::EnterInsertMode => {
                    return Ok((input, Box::new(InsertVimEngine::from(*self))))
                }
                NormalModeAction::EnterVisualMode => {
                    return Ok((input, Box::new(VisualVimEngine::from(*self))))
                }
                NormalModeAction::MoveLeft => {
                    self.col = self.col.saturating_sub(1);
                }
                NormalModeAction::MoveDown => {
                    self.line = self.line.saturating_add(1).min(lines.max_line());
                }
                NormalModeAction::MoveUp => {
                    self.line = self.line.saturating_sub(1);
                }
                NormalModeAction::MoveRight => {
                    self.col = self.col.saturating_add(1).min(lines.max_col(self.line)?);
                }
                NormalModeAction::MoveToEndOfLine => {
                    self.col = lines.end_of_line(self.line);
                }
                NormalModeAction::Command(command) => match command {
                    VimCommand::JumpTo { line } => {
                        (self.line, self.col) = lines.jump_to(line.saturating_sub(1));
                    }
                    VimCommand::JumpForward { lines: num_lines } => {
                        (self.line, self.col) = lines.jump_to(self.line.saturating_add(num_lines));
                    }
                    VimCommand::JumpBackward { lines: num_lines } => {
                        (self.line, self.col) = lines.jump_to(self.line.saturating_sub(num_lines));
                    }
                },
            }
        }

        Ok((input, self))
    }
}

impl From<InsertVimEngine> for NormalVimEngine {
    fn from(value: InsertVimEngine) -> Self {
        Self {
            line: value.line,
            col: value.col,
        }
    }
}
