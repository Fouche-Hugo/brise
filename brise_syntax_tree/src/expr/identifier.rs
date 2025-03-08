use brise_token::{BriseContext, RawString};

use super::{Expr, ExprVariant};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    identity: RawString,
    context: BriseContext,
}

impl Identifier {
    pub fn new(identity: RawString, context: BriseContext) -> Self {
        Self { identity, context }
    }

    pub fn identity(&self) -> &RawString {
        &self.identity
    }

    pub fn context(&self) -> &BriseContext {
        &self.context
    }
}

impl From<Identifier> for Expr {
    fn from(value: Identifier) -> Self {
        Self::new(ExprVariant::Identifier(value))
    }
}
