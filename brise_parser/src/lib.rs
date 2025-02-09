pub use brise_token;
use error::ParserError;
pub use error::{ParsingError, ParsingErrorVariant, ParsingErrors};

mod error;

use brise_token::{BriseContext, BriseFile, Column, Line, Token, TokenVariant};

#[derive(Debug)]
pub struct Parser {
    line: Line,
    col: Column,
    file: BriseFile,
    current: usize,
    input: String,
}

impl Parser {
    pub fn parse_file(file: impl Into<BriseFile>) -> Result<Vec<Token>, ParserError> {
        let file = file.into();
        let input =
            Self::read_file(&file).map_err(|e| ParserError::FailedToReadFile(file.clone(), e))?;

        let tokens = Self::new(file, input).parse()?;

        Ok(tokens)
    }

    fn read_file(file_path: &BriseFile) -> Result<String, std::io::Error> {
        std::fs::read_to_string(file_path.as_path())
    }

    fn new(file: BriseFile, input: String) -> Self {
        Self {
            file,
            line: Line::default(),
            col: Column::default(),
            current: 0,
            input,
        }
    }

    fn parse(&mut self) -> Result<Vec<Token>, ParsingErrors> {
        let mut tokens = vec![];
        let mut errors = vec![];
        let input_len = self.input.chars().count();

        while self.current < input_len {
            match self.parse_next_token() {
                Ok(token) => tokens.push(token),
                Err(e) => errors.push(e),
            }
        }

        if !errors.is_empty() {
            Err(errors.into())
        } else {
            Ok(tokens)
        }
    }

    fn parse_next_token(&mut self) -> Result<Token, ParsingError> {
        self.current += 1;

        // We can unwrap here, because a check is made by the loop in parse
        let current_token = self.input.chars().nth(self.current).unwrap();
        match current_token {
            '\n' | '\t' | '\r' | ' ' => {
                self.line += 1;
                self.parse_next_token()
            }
            _ => {
                // care potential bug here because col is initialized to 1
                self.col += 1;
                self.parse_token(current_token)
            }
        }
    }

    fn parse_token(&mut self, token: char) -> Result<Token, ParsingError> {
        let variant = match token {
            '(' => TokenVariant::LeftParen,
        };

        let context = self.compute_context();

        Ok(Token::new(variant, context))
    }

    fn compute_context(&self) -> BriseContext {
        BriseContext::new(self.file.clone(), self.line, self.col)
    }
}
