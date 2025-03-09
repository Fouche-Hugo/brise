use brise_token::BriseContext;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExprErrorVariant {
    #[error("A grouping expression was started here but was never closed")]
    UnclosedGrouping,
    #[error("A token was expected here")]
    ExpectedToken,
}

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
