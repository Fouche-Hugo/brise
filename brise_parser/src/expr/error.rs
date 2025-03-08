use brise_token::BriseContext;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExprErrorVariant {}

#[derive(Debug, Error)]
#[error("{context} {variant}")]
pub struct ExprError {
    variant: ExprErrorVariant,
    context: BriseContext,
}

impl ExprError {
    pub fn new(variant: ExprErrorVariant, context: BriseContext) -> Self {
        Self { variant, context }
    }
}
