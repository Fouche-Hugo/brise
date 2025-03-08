use std::collections::VecDeque;

use brise_syntax_tree::expr::{
    binary::{BinaryExpr, BinaryOperator, BinaryOperatorVariant},
    Expr, ExprVariant,
};
use brise_token::{Token, TokenVariant};
use error::ExprError;

pub mod error;

#[derive(Debug)]
pub struct ExprParser<'a> {
    input: &'a mut VecDeque<Token>,
}

impl<'a> ExprParser<'a> {
    pub fn parse(tokens: &'a mut VecDeque<Token>) -> Result<Expr, ExprError> {
        Self::new(tokens).parse_input()
    }

    fn new(input: &'a mut VecDeque<Token>) -> Self {
        Self { input }
    }

    fn parse_input(&mut self) -> Result<Expr, ExprError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ExprError> {
        let expr = self.parse_and()?;

        if self
            .input
            .front()
            .is_some_and(|token| matches!(token.variant(), TokenVariant::BarBar))
        {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_or()?;
            let binary_operator =
                BinaryOperator::new(BinaryOperatorVariant::Or, operator_token.into());
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            return Ok(binary_expr.into());
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, ExprError> {
        let expr = self.parse_equality()?;

        if self
            .input
            .front()
            .is_some_and(|token| matches!(token.variant(), TokenVariant::AmpersandAmpersand))
        {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_and()?;
            let binary_operator =
                BinaryOperator::new(BinaryOperatorVariant::And, operator_token.into());
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            return Ok(binary_expr.into());
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ExprError> {
        let expr = self.parse_comparison()?;

        if self.input.front().is_some_and(|token| token.is_equality()) {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_equality()?;
            let binary_operator = operator_token.try_into().unwrap();
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            return Ok(binary_expr.into());
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ExprError> {}
}
