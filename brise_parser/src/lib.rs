use std::num::NonZeroUsize;

pub use brise_token;
use error::ParserError;
pub use error::{ParsingError, ParsingErrorVariant, ParsingErrors};

mod error;

use brise_token::{BriseContext, BriseFile, Column, Line, Token, TokenVariant};

#[derive(Debug)]
pub struct Parser {
    file: Option<BriseFile>,
    line: Line,
    col: Column,
    current: usize,
    input: String,
}

impl Parser {
    pub fn parse_file(file: impl Into<BriseFile>) -> Result<Vec<Token>, ParserError> {
        let file = file.into();
        let input =
            Self::read_file(&file).map_err(|e| ParserError::FailedToReadFile(file.clone(), e))?;

        let tokens = Self::new(Some(file), input).parse_input()?;

        Ok(tokens)
    }

    pub fn parse(input: String) -> Result<Vec<Token>, ParserError> {
        Self::new(None, input).parse_input().map_err(|e| e.into())
    }

    fn read_file(file_path: &BriseFile) -> Result<String, std::io::Error> {
        std::fs::read_to_string(file_path.as_path())
    }

    fn new(file: Option<BriseFile>, input: String) -> Self {
        Self {
            file,
            line: Line::default(),
            col: Column::default(),
            current: 0,
            input,
        }
    }

    fn parse_input(&mut self) -> Result<Vec<Token>, ParsingErrors> {
        let mut tokens = vec![];
        let mut errors = vec![];
        let input_len = self.input.chars().count();

        while self.current < input_len {
            match self.parse_next_token() {
                Ok(token) => tokens.push(token),
                Err(e) => errors.push(e),
            }
            self.current += 1;
        }

        if !errors.is_empty() {
            Err(errors.into())
        } else {
            Ok(tokens)
        }
    }

    fn parse_next_token(&mut self) -> Result<Token, ParsingError> {
        // We can unwrap here, because a check is made by the loop in parse
        let current_token = self.input.chars().nth(self.current).unwrap();
        match current_token {
            '\n' => {
                self.line += 1;
                self.current += 1;
                self.parse_next_token()
            }
            '\t' | '\r' | ' ' => {
                self.col += 1;
                self.current += 1;
                self.parse_next_token()
            }
            _ => {
                let result = self.parse_token(current_token);
                self.col += 1;
                result
            }
        }
    }

    fn parse_token(&mut self, token: char) -> Result<Token, ParsingError> {
        let context = self.compute_context();
        let variant = match token {
            '(' => TokenVariant::LeftParen,
            ')' => TokenVariant::RightParen,
            '{' => TokenVariant::LeftBrace,
            '}' => TokenVariant::RightBrace,
            '[' => TokenVariant::LeftBracket,
            ']' => TokenVariant::RightBracket,
            ';' => TokenVariant::Semicolon,
            ',' => TokenVariant::Comma,
            ':' => TokenVariant::Colon,
            '.' => TokenVariant::Dot,
            '+' => TokenVariant::Plus,
            '/' => TokenVariant::Slash,
            '*' => TokenVariant::Star,
            '?' => TokenVariant::QuestionMark,
            '=' => TokenVariant::Equal,
            '!' => self.bang(),
            '>' => self.greater(),
            '<' => self.less(),
            '-' => self.minus(),
            '&' if self.next_token_matches('&') => {
                self.current += 1;
                TokenVariant::AmpersandAmpersand
            }
            '|' if self.next_token_matches('|') => {
                self.current += 1;
                TokenVariant::BarBar
            }
            '0'..='9' => self.number(token),
            '"' => self.string()?,
            '_' | 'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => {
                return Err(ParsingError::new(
                    ParsingErrorVariant::UnexpectedCharacter(token),
                    self.compute_context(),
                ))
            }
        };

        Ok(Token::new(variant, context))
    }

    fn next_token_matches(&self, c: char) -> bool {
        self.input
            .chars()
            .nth(self.current + 1)
            .is_some_and(|next_char| next_char == c)
    }

    fn compute_context(&self) -> BriseContext {
        BriseContext::new(self.file.clone(), self.line, self.col)
    }

    fn minus(&mut self) -> TokenVariant {
        if self.next_token_matches('>') {
            self.current += 1;
            self.col += 1;
            TokenVariant::RightArrow
        } else {
            TokenVariant::Minus
        }
    }

    fn bang(&mut self) -> TokenVariant {
        if self.next_token_matches('=') {
            self.current += 1;
            TokenVariant::BangEqual
        } else if self.next_token_matches('>') {
            self.current += 1;
            TokenVariant::BangRightChevron
        } else {
            TokenVariant::Bang
        }
    }

    fn greater(&mut self) -> TokenVariant {
        if self.next_token_matches('=') {
            self.current += 1;
            TokenVariant::GreaterEqual
        } else {
            TokenVariant::Greater
        }
    }

    fn less(&mut self) -> TokenVariant {
        if self.next_token_matches('=') {
            self.current += 1;
            TokenVariant::LessEqual
        } else {
            TokenVariant::Less
        }
    }

    fn number(&mut self, first_ch: char) -> TokenVariant {
        let mut num_str = String::from(first_ch);
        let mut has_dot = false;

        loop {
            self.current += 1;
            let Some(next_ch) = self.input.chars().nth(self.current) else {
                break;
            };

            match next_ch {
                '0'..='9' => num_str.push(next_ch),
                '.' => {
                    if has_dot {
                        break;
                    } else {
                        has_dot = true;
                        num_str.push(next_ch)
                    }
                }
                _ => break,
            }
        }

        self.col += num_str.len();
        // We can unwrap here, because the parse should never fail
        TokenVariant::Number(num_str.parse().unwrap())
    }

    fn string(&mut self) -> Result<TokenVariant, ParsingError> {
        let Some(str_length) = self
            .input
            .chars()
            .skip(self.current + 1)
            .position(|ch| ch == '"')
        else {
            return Err(ParsingError::new(
                ParsingErrorVariant::UnterminatedString,
                self.compute_context(),
            ));
        };

        let start = self.current + 1;
        self.current += str_length + 1;

        let brise_string: String = self.input.chars().skip(start).take(str_length).collect();

        let last_new_line = brise_string.chars().rev().position(|ch| ch == '\n');

        if let Some(last_new_line) = last_new_line {
            self.col = NonZeroUsize::new(1 + last_new_line).unwrap().into();
            let new_lines = brise_string.chars().filter(|ch| *ch == '\n').count();
            self.line += new_lines;
        } else {
            self.col += str_length + 1;
        }

        Ok(TokenVariant::String(brise_string.into()))
    }

    fn identifier(&mut self) -> TokenVariant {
        let identifier_start = self.current;

        while self
            .input
            .chars()
            .nth(self.current + 1)
            .is_some_and(|ch| matches!(ch, '_' | '0'..='9' | 'a'..='z' | 'A'..='Z'))
        {
            self.current += 1;
        }

        let identifier_len = 1 + self.current - identifier_start;
        let identifier: String = self
            .input
            .chars()
            .skip(identifier_start)
            .take(identifier_len)
            .collect();

        self.col += identifier_len - 1;

        match identifier.as_str() {
            "if" => TokenVariant::If,
            "else" => TokenVariant::Else,
            "loop" => TokenVariant::Loop,
            "while" => TokenVariant::While,
            "for" => TokenVariant::For,
            "fn" => TokenVariant::Fn,
            "self" => TokenVariant::BriseSelf,
            "let" => TokenVariant::Let,
            "true" => TokenVariant::True,
            "false" => TokenVariant::False,
            "break" => TokenVariant::Break,
            "continue" => TokenVariant::Continue,
            "return" => TokenVariant::Return,
            _ => TokenVariant::Identifier(identifier.into()),
        }
    }
}
