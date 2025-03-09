use std::collections::VecDeque;

use brise_syntax_tree::expr::{
    literal::{Literal, LiteralVariant, NumberLiteral},
    Expr, ExprVariant,
};
use brise_token::{BriseContext, Token, TokenVariant};

use super::ExprParser;

fn tokens_from_variant(variants: Vec<TokenVariant>) -> VecDeque<Token> {
    variants
        .into_iter()
        .map(|variant| Token::new(variant, BriseContext::default()))
        .collect()
}

#[test]
fn number_literal() {
    let tokens = vec![TokenVariant::Number(45.6)];
    let mut tokens = tokens_from_variant(tokens);

    let expr = ExprParser::parse(&mut tokens).unwrap();

    let expected_expr = Expr::new(ExprVariant::Literal(Literal::new(
        LiteralVariant::Number(NumberLiteral::new_with_custom_id(45.6, 0)),
        BriseContext::default(),
    )));

    assert!(tokens.is_empty());
    assert_eq!(expected_expr, expr);
}
