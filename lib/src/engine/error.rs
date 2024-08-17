use thiserror::Error;

use crate::lines::LineError;

#[derive(Error, Debug)]
pub enum VimEngineError {
    #[error("failed to parse")]
    ParseError { error: String },

    #[error("failed to apply line operation")]
    LineError(#[from] LineError),
}

impl From<nom::Err<nom::error::Error<&str>>> for VimEngineError {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::ParseError {
            error: value.to_string(),
        }
    }
}

pub type VimEngineResult = Result<String, VimEngineError>;
