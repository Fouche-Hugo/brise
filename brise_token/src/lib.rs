mod context;
mod raw_string;

use std::{fmt::Display, hash::Hash};

pub use context::{BriseContext, BriseFile, Column, Line};
pub use raw_string::RawString;

#[derive(Debug, PartialEq)]
pub enum TokenVariant {
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `;`
    Semicolon,
    /// `/`
    Slash,
    /// `*`
    Star,
    /// `!`
    Bang,
    /// `!=`
    BangEqual,
    /// `=`
    Equal,
    /// `==`
    EqualEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
    /// `<`
    Less,
    /// `<=`
    LessEqual,
    Identifier(RawString),
    /// ""
    String(RawString),
    /// ``` `${}` ```
    FormattedString(Vec<Token>),
    Number(f64),
    /// `&&`
    AmpersandAmpersand,
    /// `||`
    BarBar,
    /// `!>`
    BangRightChevron,
    /// `return`
    Return,
    /// `if`
    If,
    /// `else`
    Else,
    /// `while`
    While,
    /// `loop`
    Loop,
    /// `for`
    For,
    /// `self`
    BriseSelf,
    /// `let`
    Let,
    /// `true`
    True,
    /// `false`
    False,
    /// `?`
    QuestionMark,
    /// `:`
    Colon,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `fn`
    Fn,
    /// `->`
    RightArrow,
}

impl Display for TokenVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::AmpersandAmpersand => "&&",
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::BangRightChevron => "!>",
            Self::BarBar => "||",
            Self::Break => "break",
            Self::BriseSelf => "self",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Continue => "continue",
            Self::Dot => ".",
            Self::Else => "else",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::False => "false",
            Self::Fn => "fn",
            Self::For => "for",
            Self::FormattedString(tokens) => &format!(
                "`{}`",
                tokens
                    .iter()
                    .map(|token| token.variant().to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ),
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Identifier(identifier) => identifier.as_str(),
            Self::If => "if",
            Self::LeftBrace => "{",
            Self::LeftBracket => "[",
            Self::LeftParen => "(",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Let => "let",
            Self::Loop => "loop",
            Self::Minus => "-",
            Self::Number(num) => &num.to_string(),
            Self::Plus => "+",
            Self::QuestionMark => "?",
            Self::Return => "return",
            Self::RightArrow => "->",
            Self::RightBrace => "}",
            Self::RightBracket => "]",
            Self::RightParen => ")",
            Self::Semicolon => ";",
            Self::Slash => "/",
            Self::Star => "*",
            Self::String(string) => &format!("\"{string}\""),
            Self::True => "true",
            Self::While => "while",
        };

        f.write_str(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    variant: TokenVariant,
    context: BriseContext,
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.context.hash(state);
    }
}

impl Token {
    pub fn new(variant: TokenVariant, context: BriseContext) -> Self {
        Self { variant, context }
    }

    pub fn variant(&self) -> &TokenVariant {
        &self.variant
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.variant, self.context))
    }
}
