use brise_token::{BriseContext, Token, TokenVariant};

use crate::error::TokenConversionError;

use super::{Expr, ExprVariant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnaryOperatorVariant {
    Bang,
    Minus,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnaryOperator {
    variant: UnaryOperatorVariant,
    context: BriseContext,
}

impl UnaryOperator {
    pub fn new(variant: UnaryOperatorVariant, context: BriseContext) -> Self {
        Self { variant, context }
    }

    pub fn variant(&self) -> UnaryOperatorVariant {
        self.variant
    }

    pub fn context(&self) -> &BriseContext {
        &self.context
    }
}

impl TryFrom<Token> for UnaryOperator {
    type Error = TokenConversionError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        let variant = match value.variant() {
            TokenVariant::Bang => UnaryOperatorVariant::Bang,
            TokenVariant::Minus => UnaryOperatorVariant::Minus,
            _ => return Err(TokenConversionError::UnaryOperator(value)),
        };

        Ok(Self::new(variant, value.into()))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnaryExpr {
    operator: UnaryOperator,
    expr: Expr,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, expr: Expr) -> Self {
        Self { operator, expr }
    }

    pub fn operator(&self) -> &UnaryOperator {
        &self.operator
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Self::new(ExprVariant::Unary(value))
    }
}
