use brise_parser::Parser;
use brise_token::{BriseContext, Column, Line, Token, TokenVariant};

#[test]
fn parse_integer() {
    let numbers = [1, 5, 56, 891, 120003, 145560321];
    let expected_numbers = [1.0, 5.0, 56.0, 891.0, 120003.0, 145560321.0];

    for (num, result) in numbers.iter().zip(expected_numbers) {
        let tokens = Parser::parse(num.to_string()).unwrap();

        assert_eq!(
            vec![Token::new(
                TokenVariant::Number(result),
                BriseContext::new(None, Line::default(), Column::default())
            )],
            tokens
        );
    }
}

#[test]
fn parse_float() {
    let numbers = [2.0, 3.4, 8.41, 6.5982, 516.02, 123.981, 1482.0];

    for num in numbers {
        let tokens = Parser::parse(num.to_string()).unwrap();

        assert_eq!(
            vec![Token::new(
                TokenVariant::Number(num),
                BriseContext::new(None, Line::default(), Column::default())
            )],
            tokens
        );
    }
}

#[test]
fn parse_string() {
    let string = String::from("\"my string\"");

    let tokens = Parser::parse(string.clone()).unwrap();

    assert_eq!(
        vec![Token::new(
            TokenVariant::String(string.into()),
            BriseContext::new(None, Line::default(), Column::default())
        )],
        tokens
    );
}
