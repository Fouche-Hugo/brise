use std::fmt::Display;

use brise_token::{BriseContext, BriseFile};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsingErrorVariant {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
}

#[derive(Debug, Error)]
#[error("{context} {variant}")]
pub struct ParsingError {
    variant: ParsingErrorVariant,
    context: BriseContext,
}

impl ParsingError {
    pub fn new(variant: ParsingErrorVariant, context: BriseContext) -> Self {
        Self { variant, context }
    }
}

#[derive(Debug, Error)]
pub struct ParsingErrors(Vec<ParsingError>);

impl From<ParsingError> for ParsingErrors {
    fn from(value: ParsingError) -> Self {
        Self(vec![value])
    }
}

impl<T: IntoIterator<Item = ParsingError>> From<T> for ParsingErrors {
    fn from(value: T) -> Self {
        Self(value.into_iter().collect())
    }
}

impl Display for ParsingErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error(transparent)]
    Parsing(#[from] ParsingErrors),
    #[error("Failed to read file: {} - {}", .0.as_path().display(), .1)]
    FailedToReadFile(BriseFile, std::io::Error),
}
