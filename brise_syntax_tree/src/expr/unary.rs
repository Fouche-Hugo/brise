use brise_token::BriseContext;

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
