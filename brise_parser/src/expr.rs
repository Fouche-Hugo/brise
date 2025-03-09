use std::collections::VecDeque;

use brise_syntax_tree::expr::{
    binary::{BinaryExpr, BinaryOperator, BinaryOperatorVariant},
    identifier::Identifier,
    literal::{Literal, LiteralVariant, NumberLiteral},
    unary::UnaryExpr,
    Expr, ExprVariant,
};
use brise_token::{BriseContext, Token, TokenVariant};
use error::{ExprError, ExprErrorVariant};

pub mod error;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct ExprParser<'a> {
    input: &'a mut VecDeque<Token>,
    last_context: BriseContext,
}

impl<'a> ExprParser<'a> {
    pub fn parse(tokens: &'a mut VecDeque<Token>) -> Result<Expr, ExprError> {
        Self::new(tokens).parse_input()
    }

    fn new(input: &'a mut VecDeque<Token>) -> Self {
        Self {
            input,
            last_context: BriseContext::default(),
        }
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
        let mut expr = self.parse_comparison()?;

        while self.input.front().is_some_and(|token| token.is_equality()) {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_comparison()?;
            let binary_operator = operator_token.try_into().unwrap();
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            expr = binary_expr.into();
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.parse_term()?;

        while self
            .input
            .front()
            .is_some_and(|token| token.is_comparison())
        {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_term()?;
            let binary_operator = operator_token.try_into().unwrap();
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            expr = binary_expr.into();
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.parse_factor()?;

        while self.input.front().is_some_and(|token| token.is_term()) {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_factor()?;
            let binary_operator = operator_token.try_into().unwrap();
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            expr = binary_expr.into();
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ExprError> {
        let mut expr = self.parse_unary()?;

        if self.input.front().is_some_and(|token| token.is_factor()) {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_unary()?;
            let binary_operator = operator_token.try_into().unwrap();
            let binary_expr = BinaryExpr::new(expr, binary_operator, right);

            expr = binary_expr.into();
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ExprError> {
        if self.input.front().is_some_and(|token| token.is_unary()) {
            let operator_token = self.input.pop_front().unwrap();
            let right = self.parse_unary()?;
            let unary_operator = operator_token.try_into().unwrap();
            let unary_expr = UnaryExpr::new(unary_operator, right);

            return Ok(unary_expr.into());
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr, ExprError> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ExprError> {
        if let Some(token) = self.input.pop_front() {
            let token_variant = token.variant();
            if let TokenVariant::LeftParen = token_variant {
                let expr = self.parse_input()?;

                if self
                    .input
                    .front()
                    .is_some_and(|token| matches!(token.variant(), TokenVariant::RightParen))
                {
                    self.input.pop_front();

                    let expr = ExprVariant::Grouping(expr);
                    return Ok(Expr::new(expr));
                } else {
                    return Err(ExprError::new(
                        ExprErrorVariant::UnclosedGrouping,
                        token.into(),
                    ));
                }
            } else if let TokenVariant::Identifier(identity) = token_variant {
                let expr = ExprVariant::Identifier(Identifier::new(identity.clone(), token.into()));
                return Ok(Expr::new(expr));
            } else if token_variant.is_literal() {
                return self.parse_literal(token);
            }
        }

        Err(ExprError::new(
            ExprErrorVariant::UnclosedGrouping,
            self.last_context.clone(),
        ))
    }

    fn parse_literal(&mut self, token: Token) -> Result<Expr, ExprError> {
        let literal = match token.variant() {
            TokenVariant::Number(value) => {
                let number = NumberLiteral::new(*value);
                Literal::new(LiteralVariant::Number(number), token.into())
            }
            TokenVariant::String(value) => {
                Literal::new(LiteralVariant::String(value.clone()), token.into())
            }
            TokenVariant::FormattedString(_) => {
                unimplemented!()
            }
            TokenVariant::True => Literal::new(LiteralVariant::True, token.into()),
            TokenVariant::False => Literal::new(LiteralVariant::False, token.into()),
            TokenVariant::QuestionMark => Literal::new(LiteralVariant::Unknown, token.into()),
            _ => unreachable!(),
        };

        Ok(literal.into())
    }

    fn pop_front(&mut self) -> Token {
        let token = self.input.pop_front().unwrap();

        if self.input.is_empty() {
            self.last_context = token.context().clone();
        }

        token
    }
}
