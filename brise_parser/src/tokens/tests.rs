use std::num::NonZeroUsize;

use super::*;
use brise_token::{BriseContext, Column, Line, Token, TokenVariant};

fn token(variant: TokenVariant, line: usize, col: usize) -> Token {
    Token::new(
        variant,
        BriseContext::new(
            None,
            Line::from(NonZeroUsize::new(line).unwrap()),
            Column::from(NonZeroUsize::new(col).unwrap()),
        ),
    )
}

#[test]
fn parse_integer() {
    let numbers = [1, 5, 56, 891, 120003, 145560321];
    let expected_numbers = [1.0, 5.0, 56.0, 891.0, 120003.0, 145560321.0];

    for (num, result) in numbers.iter().zip(expected_numbers) {
        let tokens = TokenParser::parse(num.to_string()).unwrap();

        assert_eq!(vec![token(TokenVariant::Number(result), 1, 1)], tokens);
    }
}

#[test]
fn parse_float() {
    let numbers = [2.0, 3.4, 8.41, 6.5982, 516.02, 123.981, 1482.0];

    for num in numbers {
        let tokens = TokenParser::parse(num.to_string()).unwrap();

        assert_eq!(vec![token(TokenVariant::Number(num), 1, 1)], tokens);
    }
}

#[test]
fn two_numbers() {
    let number1 = 21.0;
    let number2 = 24.4;

    let input = format!("{number1:?} {number2:?}");

    let tokens = TokenParser::parse(input).unwrap();

    assert_eq!(
        vec![
            token(TokenVariant::Number(number1), 1, 1),
            token(TokenVariant::Number(number2), 1, 6)
        ],
        tokens
    )
}

#[test]
fn parse_string() {
    let brise_string = "my string";
    let string = format!("\"{brise_string}\"");

    let tokens = TokenParser::parse(string.clone()).unwrap();

    assert_eq!(
        vec![token(TokenVariant::String(brise_string.into()), 1, 1)],
        tokens
    );
}

#[test]
fn parse_string_with_multiple_lines() {
    let brise_string = r"my
string";
    let number = 54.0;
    let string = format!("\"{brise_string}\"{number}");

    let tokens = TokenParser::parse(string.clone()).unwrap();

    let expected_tokens = vec![
        token(TokenVariant::String(brise_string.into()), 1, 1),
        token(TokenVariant::Number(number), 2, 8),
    ];

    assert_eq!(expected_tokens, tokens);
}

#[test]
fn parse_identifier() {
    let input = "let a";

    let tokens = TokenParser::parse(input.into()).unwrap();

    let expected_tokens = vec![
        token(TokenVariant::Let, 1, 1),
        token(TokenVariant::Identifier("a".into()), 1, 5),
    ];

    assert_eq!(expected_tokens, tokens);
}

#[test]
fn parse_variable_definition() {
    let input = "let sasuke = \"sasuke\";";

    let tokens = TokenParser::parse(input.into()).unwrap();

    let expected_tokens = vec![
        token(TokenVariant::Let, 1, 1),
        token(TokenVariant::Identifier("sasuke".into()), 1, 5),
        token(TokenVariant::Equal, 1, 12),
        token(TokenVariant::String("sasuke".into()), 1, 14),
        token(TokenVariant::Semicolon, 1, 22),
    ];

    assert_eq!(expected_tokens, tokens);
}

#[test]
fn parse_function_definition() {
    let input = "fn myfunction(a: number) -> number {}";

    let tokens = TokenParser::parse(input.into()).unwrap();

    let expected_tokens = vec![
        token(TokenVariant::Fn, 1, 1),
        token(TokenVariant::Identifier("myfunction".into()), 1, 4),
        token(TokenVariant::LeftParen, 1, 14),
        token(TokenVariant::Identifier("a".into()), 1, 15),
        token(TokenVariant::Colon, 1, 16),
        token(TokenVariant::Identifier("number".into()), 1, 18),
        token(TokenVariant::RightParen, 1, 24),
        token(TokenVariant::RightArrow, 1, 26),
        token(TokenVariant::Identifier("number".into()), 1, 29),
        token(TokenVariant::LeftBrace, 1, 36),
        token(TokenVariant::RightBrace, 1, 37),
    ];

    assert_eq!(expected_tokens, tokens);
}

#[test]
fn parse_function_call() {
    let input = "myfunction(a, b);";

    let tokens = TokenParser::parse(input.into()).unwrap();

    let expected_tokens = vec![
        token(TokenVariant::Identifier("myfunction".into()), 1, 1),
        token(TokenVariant::LeftParen, 1, 11),
        token(TokenVariant::Identifier("a".into()), 1, 12),
        token(TokenVariant::Comma, 1, 13),
        token(TokenVariant::Identifier("b".into()), 1, 15),
        token(TokenVariant::RightParen, 1, 16),
        token(TokenVariant::Semicolon, 1, 17),
    ];

    assert_eq!(expected_tokens, tokens);
}
