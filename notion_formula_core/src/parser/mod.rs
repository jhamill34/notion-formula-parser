mod util;

use crate::tokenizer::{Token};
use lookahead_buffer::LookaheadBuffer;
use pipeline::{HandlerResult};

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOp(Box<Expression>, MathOperator, Box<Expression>),
    Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
    BooleanOp(Box<Expression>, BooleanOperator, Box<Expression>),
    UnaryOp(UnaryOperator, Box<Expression>),
    TernaryOp(Box<Expression>, Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Identifier(String),
    Str(String),
    Number(String),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Exponent,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq,
}

#[derive(Debug, PartialEq)]
pub enum BooleanOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    UAdd,
    USub,
    Not,
}

pub fn formula_parser(input: Vec<Token>) -> HandlerResult<Expression> {
    let mut buffer = LookaheadBuffer::new(input);
    util::expression(&mut buffer)
}

#[cfg(test)]
mod test {
    use super::BooleanOperator::*;
    use super::ComparisonOperator::*;
    use super::Expression::*;
    use super::MathOperator::*;
    use super::UnaryOperator::*;
    use super::*;
    use crate::tokenizer::TokenType;

    #[test]
    fn test_number() {
        let input = vec![
            Token::new(TokenType::NumberLiteral("123".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(Number("123".into()), result);
    }

    #[test]
    fn test_string() {
        let input = vec![
            Token::new(TokenType::StringLiteral("\"hello\"".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(Str("\"hello\"".into()), result);
    }

    #[test]
    fn test_boolean_constant() {
        let input = vec![
            Token::new(TokenType::True, 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(Bool(true), result)
    }

    #[test]
    fn test_identifier() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(Identifier("foo".into()), result)
    }

    #[test]
    fn test_exponential_expressions() {
        let input = vec![
            Token::new(TokenType::NumberLiteral("2".into()), 1, 1),
            Token::new(TokenType::Caret, 1, 1),
            Token::new(TokenType::NumberLiteral("3".into()), 1, 1),
            Token::new(TokenType::Caret, 1, 1),
            Token::new(TokenType::Identifier("x".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BinaryOp(
                Box::new(Number("2".into())),
                Exponent,
                Box::new(BinaryOp(
                    Box::new(Number("3".into())),
                    Exponent,
                    Box::new(Identifier("x".into()))
                )),
            ),
            result
        )
    }

    #[test]
    fn test_parenthetical_expressions() {
        let input = vec![
            Token::new(TokenType::LeftParen, 1, 1),
            Token::new(TokenType::Identifier("a".into()), 1, 1),
            Token::new(TokenType::Plus, 1, 1),
            Token::new(TokenType::Identifier("b".into()), 1, 1),
            Token::new(TokenType::RightParen, 1, 1),
            Token::new(TokenType::Star, 1, 1),
            Token::new(TokenType::Identifier("d".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BinaryOp(
                Box::new(BinaryOp(
                    Box::new(Identifier("a".into())),
                    Add,
                    Box::new(Identifier("b".into()))
                )),
                Multiply,
                Box::new(Identifier("d".into()))
            ),
            result
        )
    }

    #[test]
    fn test_order_of_operations() {
        let input = vec![
            Token::new(TokenType::Identifier("a".into()), 1, 1),
            Token::new(TokenType::Plus, 1, 1),
            Token::new(TokenType::Identifier("b".into()), 1, 1),
            Token::new(TokenType::Star, 1, 1),
            Token::new(TokenType::Identifier("d".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BinaryOp(
                Box::new(Identifier("a".into())),
                Add,
                Box::new(BinaryOp(
                    Box::new(Identifier("b".into())),
                    Multiply,
                    Box::new(Identifier("d".into()))
                )),
            ),
            result
        )
    }

    #[test]
    fn test_prefix_expresssion() {
        let input = vec![
            Token::new(TokenType::Minus, 1, 1),
            Token::new(TokenType::Minus, 1, 1),
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            UnaryOp(
                USub,
                Box::new(UnaryOp(USub, Box::new(Identifier("foo".into()))))
            ),
            result
        )
    }

    #[test]
    fn test_mult_expresssion() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::Star, 1, 1),
            Token::new(TokenType::Identifier("bar".into()), 1, 1),
            Token::new(TokenType::Slash, 1, 1),
            Token::new(TokenType::Identifier("baz".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BinaryOp(
                Box::new(BinaryOp(
                    Box::new(Identifier("foo".into())),
                    Multiply,
                    Box::new(Identifier("bar".into()))
                )),
                Divide,
                Box::new(Identifier("baz".into()))
            ),
            result
        )
    }

    #[test]
    fn test_additive_expresssion() {
        let input = vec![
            Token::new(TokenType::NumberLiteral("1".into()), 1, 1),
            Token::new(TokenType::Plus, 1, 1),
            Token::new(TokenType::NumberLiteral("2".into()), 1, 1),
            Token::new(TokenType::Minus, 1, 1),
            Token::new(TokenType::NumberLiteral("3".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BinaryOp(
                Box::new(BinaryOp(
                    Box::new(Number("1".into())),
                    Add,
                    Box::new(Number("2".into()))
                )),
                Subtract,
                Box::new(Number("3".into()))
            ),
            result
        )
    }

    #[test]
    fn test_relational_expresssion() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::GreaterEqual, 1, 1),
            Token::new(TokenType::Identifier("baz".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            Comparison(
                Box::new(Identifier("foo".into())),
                GreaterThanEq,
                Box::new(Identifier("baz".into()))
            ),
            result
        )
    }

    #[test]
    fn test_equality_expresssion() {
        let input = vec![
            Token::new(TokenType::StringLiteral("\"foo\"".into()), 1, 1),
            Token::new(TokenType::BangEqual, 1, 1),
            Token::new(TokenType::Identifier("baz".into()), 1, 1),
            Token::new(TokenType::EqualEqual, 1, 1),
            Token::new(TokenType::Identifier("bar".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            Comparison(
                Box::new(Comparison(
                    Box::new(Str("\"foo\"".into())),
                    NotEquals,
                    Box::new(Identifier("baz".into()))
                )),
                Equals,
                Box::new(Identifier("bar".into()))
            ),
            result
        )
    }

    #[test]
    fn test_not_expression() {
        let input = vec![
            Token::new(TokenType::Not, 1, 1),
            Token::new(TokenType::Not, 1, 1),
            Token::new(TokenType::Identifier("baz".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            UnaryOp(
                Not,
                Box::new(UnaryOp(Not, Box::new(Identifier("baz".into()))))
            ),
            result
        )
    }

    #[test]
    fn test_and_expresssion() {
        let input = vec![
            Token::new(TokenType::Identifier("a".into()), 1, 1),
            Token::new(TokenType::And, 1, 1),
            Token::new(TokenType::Identifier("b".into()), 1, 1),
            Token::new(TokenType::And, 1, 1),
            Token::new(TokenType::Identifier("c".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BooleanOp(
                Box::new(BooleanOp(
                    Box::new(Identifier("a".into())),
                    And,
                    Box::new(Identifier("b".into()))
                )),
                And,
                Box::new(Identifier("c".into()))
            ),
            result
        )
    }

    #[test]
    fn test_or_expresssion() {
        let input = vec![
            Token::new(TokenType::Identifier("a".into()), 1, 1),
            Token::new(TokenType::Or, 1, 1),
            Token::new(TokenType::Identifier("c".into()), 1, 1),
            Token::new(TokenType::Or, 1, 1),
            Token::new(TokenType::Identifier("e".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            BooleanOp(
                Box::new(BooleanOp(
                    Box::new(Identifier("a".into())),
                    Or,
                    Box::new(Identifier("c".into()))
                )),
                Or,
                Box::new(Identifier("e".into()))
            ),
            result
        )
    }

    #[test]
    fn test_ternary_expression() {
        let input = vec![
            Token::new(TokenType::Identifier("x".into()), 1, 1),
            Token::new(TokenType::QuestionMark, 1, 1),
            Token::new(TokenType::Identifier("y".into()), 1, 1),
            Token::new(TokenType::Colon, 1, 1),
            Token::new(TokenType::Identifier("z".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            TernaryOp(
                Box::new(Identifier("x".into())),
                Box::new(Identifier("y".into())),
                Box::new(Identifier("z".into())),
            ),
            result
        )
    }

    #[test]
    fn test_no_args_function_calls() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::LeftParen, 1, 1),
            Token::new(TokenType::RightParen, 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(Call(Box::new(Identifier("foo".into())), vec![]), result)
    }

    #[test]
    fn test_function_calls() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::LeftParen, 1, 1),
            Token::new(TokenType::Identifier("bar".into()), 1, 1),
            Token::new(TokenType::Comma, 1, 1),
            Token::new(TokenType::NumberLiteral("1".into()), 1, 1),
            Token::new(TokenType::Plus, 1, 1),
            Token::new(TokenType::NumberLiteral("2".into()), 1, 1),
            Token::new(TokenType::RightParen, 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = formula_parser(input).unwrap();

        assert_eq!(
            Call(
                Box::new(Identifier("foo".into())),
                vec![
                    Identifier("bar".into()),
                    BinaryOp(
                        Box::new(Number("1".into())),
                        Add,
                        Box::new(Number("2".into()))
                    )
                ]
            ),
            result
        )
    }
}
