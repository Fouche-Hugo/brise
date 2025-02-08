mod context;
mod raw_string;

use std::hash::Hash;

pub use context::{BriseContext, Column, Line};
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
