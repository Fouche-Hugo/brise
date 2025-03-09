use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use brise_token::{BriseContext, RawString};

use super::{Expr, ExprVariant};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LiteralVariant {
    Number(NumberLiteral),
    String(RawString),
    FormattedString(Vec<Expr>),
    True,
    False,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct NumberLiteral {
    value: f64,
    /// The id is there to hash the value,
    /// each number literal should have a different id
    id: usize,
}

impl NumberLiteral {
    pub fn new(value: f64) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        Self {
            value,
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// This method should only be used in tests
    pub fn new_with_custom_id(value: f64, id: usize) -> Self {
        Self { value, id }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl PartialEq for NumberLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for NumberLiteral {}

impl Hash for NumberLiteral {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Literal {
    variant: LiteralVariant,
    context: BriseContext,
}

impl Literal {
    pub fn new(variant: LiteralVariant, context: BriseContext) -> Self {
        Self { variant, context }
    }

    pub fn variant(&self) -> &LiteralVariant {
        &self.variant
    }

    pub fn context(&self) -> &BriseContext {
        &self.context
    }
}

impl From<Literal> for Expr {
    fn from(value: Literal) -> Self {
        Self::new(ExprVariant::Literal(value))
    }
}
