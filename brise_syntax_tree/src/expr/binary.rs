use brise_token::{BriseContext, Token, TokenVariant};

use crate::error::TokenConversionError;

use super::{Expr, ExprVariant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BinaryOperatorVariant {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Or,
    And,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BinaryOperator {
    variant: BinaryOperatorVariant,
    context: BriseContext,
}

impl BinaryOperator {
    pub fn new(variant: BinaryOperatorVariant, context: BriseContext) -> Self {
        Self { variant, context }
    }

    pub fn variant(&self) -> BinaryOperatorVariant {
        self.variant
    }

    pub fn context(&self) -> &BriseContext {
        &self.context
    }
}

impl TryFrom<Token> for BinaryOperator {
    type Error = TokenConversionError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        let variant = match value.variant() {
            TokenVariant::EqualEqual => BinaryOperatorVariant::EqualEqual,
            TokenVariant::BangEqual => BinaryOperatorVariant::BangEqual,
            TokenVariant::Less => BinaryOperatorVariant::Less,
            TokenVariant::LessEqual => BinaryOperatorVariant::LessEqual,
            TokenVariant::Greater => BinaryOperatorVariant::Greater,
            TokenVariant::GreaterEqual => BinaryOperatorVariant::GreaterEqual,
            TokenVariant::Plus => BinaryOperatorVariant::Plus,
            TokenVariant::Minus => BinaryOperatorVariant::Minus,
            TokenVariant::Star => BinaryOperatorVariant::Star,
            TokenVariant::Slash => BinaryOperatorVariant::Slash,
            TokenVariant::BarBar => BinaryOperatorVariant::Or,
            TokenVariant::AmpersandAmpersand => BinaryOperatorVariant::And,
            _ => return Err(TokenConversionError::BinaryOperator(value)),
        };

        Ok(Self {
            variant,
            context: value.into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BinaryExpr {
    left: Expr,
    operator: BinaryOperator,
    right: Expr,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: BinaryOperator, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Expr {
        &self.left
    }

    pub fn operator(&self) -> &BinaryOperator {
        &self.operator
    }

    pub fn right(&self) -> &Expr {
        &self.right
    }
}

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Self::new(ExprVariant::Binary(value))
    }
}
