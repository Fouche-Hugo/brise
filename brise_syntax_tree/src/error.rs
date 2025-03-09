use brise_token::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenConversionError {
    #[error("Can't convert token {0} into a binary operator")]
    BinaryOperator(Token),
    #[error("Can't convert token {0} into an unary operator")]
    UnaryOperator(Token),
}
