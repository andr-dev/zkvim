mod engine;
mod lines;
mod mode;

pub use self::engine::VimEngineRunner;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ZkVimPuzzle {
    pub input: String,
    pub output: String,
}
