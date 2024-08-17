// use crate::mode::{visual_mode_action, VisualModeAction};

use crate::lines::Lines;

use super::{normal::NormalVimEngine, VimEngine, VimEngineRunResult};

#[derive(Debug, Default)]
pub struct VisualVimEngine {
    pub line: usize,
    pub col: usize,
}

impl VimEngine for VisualVimEngine {
    fn run<'a>(self: Box<Self>, input: &'a str, lines: &mut Lines) -> VimEngineRunResult<'a> {
        // let (input, action) = visual_mode_action(input)?;

        // match action {
        //     VisualModeAction::EnterVisualMode => {
        //         return Ok((input, Box::new(VisualVimEngine::from(self))))
        //     }
        //     VisualModeAction::EnterVisualMode => todo!(),
        // }

        todo!()
    }
}

impl From<NormalVimEngine> for VisualVimEngine {
    fn from(value: NormalVimEngine) -> Self {
        todo!()
    }
}
