use crate::tokenizer::{Token, TokenType};
use pipeline::{HandlerResult, SimpleError};
use lookahead_buffer::LookaheadBuffer;

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
    Bool(bool)
}

#[derive(Debug, PartialEq)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Exponent
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEq,
    GreaterThanEq
}

#[derive(Debug, PartialEq)]
pub enum BooleanOperator {
    And,
    Or
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    UAdd,
    USub,
    Not
}

pub fn parser(input: Vec<Token>) -> HandlerResult<Expression> {
    let mut buffer = LookaheadBuffer::new(input);
    expression(&mut buffer)
}

fn expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    ternary_expression(buffer)
}

fn ternary_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    or_expression(buffer)
}

fn or_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = and_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        match token.token_type {
            TokenType::Or => {
                buffer.advance();
                let right = and_expression(buffer)?;
                left = Expression::BooleanOp(
                    Box::new(left),
                    BooleanOperator::Or,
                    Box::new(right)
                )
            },
            _ => break
        };
    }

    Ok(left)
}

fn and_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = not_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        match token.token_type {
            TokenType::And => {
                buffer.advance();
                let right = not_expression(buffer)?;
                left = Expression::BooleanOp(
                    Box::new(left),
                    BooleanOperator::And,
                    Box::new(right)
                )
            },
            _ => break
        };
    }

    Ok(left)
}

fn not_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;

    match token.token_type {
        TokenType::Not => {
            buffer.advance();
            let operand = not_expression(buffer)?;
            Ok(Expression::UnaryOp(UnaryOperator::Not, Box::new(operand)))
        },
        _ => equality_expression(buffer)
    }
}

fn equality_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = relational_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        let op = match token.token_type {
            TokenType::EqualEqual => Some(ComparisonOperator::Equals),
            TokenType::BangEqual => Some(ComparisonOperator::NotEquals),
            _ => None
        };

        match op {
            Some(op) => {
                buffer.advance();
                let right = relational_expression(buffer)?;
                left = Expression::Comparison(
                    Box::new(left),
                    op,
                    Box::new(right)
                )
            },
            None => break
        }
    }

    Ok(left)
}

fn relational_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let left = additive_expression(buffer)?;

    let token = get_current_token(buffer)?;
    let op = match token.token_type {
        TokenType::LessEqual => Some(ComparisonOperator::LessThanEq),
        TokenType::Less => Some(ComparisonOperator::LessThan),
        TokenType::GreaterEqual => Some(ComparisonOperator::GreaterThanEq),
        TokenType::Greater => Some(ComparisonOperator::GreaterThan),
        _ => None
    };

    match op {
        Some(op) => {
            buffer.advance();
            let right = additive_expression(buffer)?;
            Ok(Expression::Comparison(
                Box::new(left),
                op,
                Box::new(right)
            ))
        },
        None => Ok(left)
    }
}

fn additive_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = multiplicative_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        let op = match token.token_type {
            TokenType::Plus => Some(MathOperator::Add),
            TokenType::Minus => Some(MathOperator::Subtract),
            _ => None
        };

        match op {
            Some(op) => {
                buffer.advance();
                let right = multiplicative_expression(buffer)?;
                left = Expression::BinaryOp(
                    Box::new(left),
                    op,
                    Box::new(right)
                )
            },
            None => break
        }
    }

    Ok(left)
}

fn multiplicative_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = prefix_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        let op = match token.token_type {
            TokenType::Star => Some(MathOperator::Multiply),
            TokenType::Slash => Some(MathOperator::Divide),
            TokenType::Percent => Some(MathOperator::Mod),
            _ => None
        };

        match op {
            Some(op) => {
                buffer.advance();
                let right = prefix_expression(buffer)?;
                left = Expression::BinaryOp(
                    Box::new(left),
                    op,
                    Box::new(right)
                )
            },
            None => break
        }
    }

    Ok(left)
}

fn prefix_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;

    let op = match token.token_type {
        TokenType::Minus => Some(UnaryOperator::USub),
        TokenType::Plus => Some(UnaryOperator::UAdd),
        _ => None
    };

    match op {
        Some(op) => {
            buffer.advance();
            let operand = prefix_expression(buffer)?;
            Ok(Expression::UnaryOp(op, Box::new(operand)))
        },
        _ => exponential_expression(buffer)
    }
}

fn exponential_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut stack= vec![];
    let mut left = atomic_expression(buffer)?;
    stack.push(left);

    loop {
        let token = get_current_token(buffer)?;

        match token.token_type {
            TokenType::Caret => {
                buffer.advance();
                left = atomic_expression(buffer)?;
                stack.push(left);
            },
            _ => break
        }
    }

    left = stack.pop().unwrap();
    while let Some(expr) = stack.pop() {
        left = Expression::BinaryOp(Box::new(expr), MathOperator::Exponent, Box::new(left));
    }

    Ok(left)
}

fn atomic_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;
    match token.token_type {
        TokenType::Identifier(id) => {
            buffer.advance();

            let token = get_current_token(buffer)?;
            let id = Expression::Identifier(id);
            match token.token_type {
                TokenType::LeftParen => {
                    buffer.advance();
                    function_call_expression(buffer, id)
                },
                _ => Ok(id)
            }
        },
        TokenType::LeftParen => {
            buffer.advance();
            let expr = expression(buffer);

            let token = get_current_token(buffer)?;
            match token.token_type {
                TokenType::RightParen => {
                    buffer.advance();
                    expr
                },
                _ => Err(SimpleError::new("Expected a closing parentheses".into()))
            }
        },
        _ => constant(buffer)
    }
}

fn function_call_expression(buffer: &mut LookaheadBuffer<Token>, id: Expression) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;

    match token.token_type {
        TokenType::RightParen => {
            buffer.advance();
            Ok(Expression::Call(Box::new(id), vec![]))
        },
        _ => Err(SimpleError::new("Expected a closing parentheses in function call".into()))
    }
}

fn constant(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;
    buffer.advance();
    match token.token_type {
         TokenType::NumberLiteral(value) => Ok(Expression::Number(value)),
         TokenType::StringLiteral(value) => Ok(Expression::Str(value)),
         TokenType::True => Ok(Expression::Bool(true)),
         TokenType::False => Ok(Expression::Bool(false)),
         _ => {
             Err(
                 SimpleError::new(
                     format!(
                         "Unexpected Token: {:?} on line: {}, column: {}",
                         token.token_type, token.line, token.column
                     )
                 )
             )
         }
    }
}

fn get_current_token(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Token> {
    let optional_token = buffer.peek(0);

    match optional_token {
        Some(token) => Ok(token),
        None => Err(SimpleError::new("No EOF token found...".into()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Expression::*;
    use super::MathOperator::*;
    use super::UnaryOperator::*;
    use super::BooleanOperator::*;
    use super::ComparisonOperator::*;
    use crate::tokenizer::TokenType;
    use crate::tokenizer::TokenType::NumberLiteral;

    #[test]
    fn test_number() {
        let input = vec![
            Token::new(TokenType::NumberLiteral("123".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            Number("123".into()),
            result
        );
    }

    #[test]
    fn test_string() {
        let input = vec![
            Token::new(TokenType::StringLiteral("\"hello\"".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            Str("\"hello\"".into()),
            result
        );
    }

    #[test]
    fn test_boolean_constant() {
        let input = vec![
            Token::new(TokenType::True, 1, 1),
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            Bool(true),
            result
        )
    }

    #[test]
    fn test_identifier() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            Identifier("foo".into()),
            result
        )
    }

    #[test]
    fn test_exponential_expressions() {
        let input = vec![
            Token::new(TokenType::NumberLiteral("2".into()), 1, 1),
            Token::new(TokenType::Caret, 1, 1),
            Token::new(TokenType::NumberLiteral("3".into()), 1, 1),
            Token::new(TokenType::Caret, 1, 1),
            Token::new(TokenType::Identifier("x".into()), 1, 1),
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
        let result = parser(input).unwrap();

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
        let result = parser(input).unwrap();

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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            UnaryOp(
                USub,
                Box::new(UnaryOp(
                    USub,
                    Box::new(Identifier("foo".into()))
                ))
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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            UnaryOp(
                Not,
                Box::new(UnaryOp(
                    Not,
                    Box::new(Identifier("baz".into()))
                ))
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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
            Token::new(TokenType::Eof, 1, 1)
        ];
        let result = parser(input).unwrap();

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
    #[ignore]
    fn test_ternary_expression() { }


    #[test]
    fn test_function_calls() {
        let input = vec![
            Token::new(TokenType::Identifier("foo".into()), 1, 1),
            Token::new(TokenType::LeftParen, 1, 1),
            Token::new(TokenType::RightParen, 1, 1),
            Token::new(TokenType::Eof, 1, 1),
        ];
        let result = parser(input).unwrap();

        assert_eq!(
            Call(
                Box::new(Identifier("foo".into())),
                vec![]
            ),
            result
        )
    }
}
