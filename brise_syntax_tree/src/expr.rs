use std::rc::Rc;

use binary::BinaryExpr;
use identifier::Identifier;
use literal::Literal;
use unary::UnaryExpr;

pub mod binary;
pub mod identifier;
pub mod literal;
pub mod unary;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Expr(Rc<ExprVariant>);

impl Expr {
    pub fn new(variant: ExprVariant) -> Self {
        Self(Rc::new(variant))
    }

    pub fn variant(&self) -> &ExprVariant {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ExprVariant {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Grouping(Expr),
    Literal(Literal),
    Identifier(Identifier),
}
