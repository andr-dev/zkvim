pub use self::runner::VimEngineRunner;

pub mod error;

use self::error::VimEngineError;
use crate::lines::Lines;

mod insert;
mod normal;
mod runner;
mod visual;

type VimEngineRunResult<'a> = Result<(&'a str, Box<dyn VimEngine>), VimEngineError>;

pub trait VimEngine {
    fn run<'a>(self: Box<Self>, input: &'a str, lines: &mut Lines) -> VimEngineRunResult<'a>;
}
