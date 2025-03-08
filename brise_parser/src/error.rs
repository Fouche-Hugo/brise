use brise_token::BriseFile;
use thiserror::Error;

use crate::tokens::error::ParsingErrors;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error(transparent)]
    Parsing(#[from] ParsingErrors),
    #[error("Failed to read file: {} - {}", .0.as_path().display(), .1)]
    FailedToReadFile(BriseFile, std::io::Error),
}
