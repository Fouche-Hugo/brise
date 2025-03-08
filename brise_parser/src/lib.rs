use brise_token::{BriseFile, Token};
use error::ParserError;

mod error;
mod tokens;
mod expr;

pub fn parse_tokens(input: String) -> Result<Vec<Token>, ParserError> {
    tokens::TokenParser::parse(input)
}

pub fn parser_file_tokens(file: impl Into<BriseFile>) -> Result<Vec<Token>, ParserError> {
    tokens::TokenParser::parse_file(file)
}
