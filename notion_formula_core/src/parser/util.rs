use super::*;

use crate::tokenizer::{Token, TokenType};
use lookahead_buffer::LookaheadBuffer;
use pipeline::{HandlerResult, SimpleError};

pub fn expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    ternary_expression(buffer)
}

fn ternary_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let test = or_expression(buffer)?;

    let token = get_current_token(buffer)?;
    match token.token_type {
        TokenType::QuestionMark => {
            buffer.advance();

            let accept = or_expression(buffer)?;
            let token = get_current_token(buffer)?;
            match token.token_type {
                TokenType::Colon => {
                    buffer.advance();
                    let reject = expression(buffer)?;
                    Ok(Expression::TernaryOp(
                        Box::new(test),
                        Box::new(accept),
                        Box::new(reject),
                    ))
                }
                _ => Err(SimpleError::new(
                    "Expected colon in ternary expression".into(),
                )),
            }
        }
        _ => Ok(test),
    }
}

fn or_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = and_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        match token.token_type {
            TokenType::Or => {
                buffer.advance();
                let right = and_expression(buffer)?;
                left = Expression::BooleanOp(Box::new(left), BooleanOperator::Or, Box::new(right))
            }
            _ => break,
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
                left = Expression::BooleanOp(Box::new(left), BooleanOperator::And, Box::new(right))
            }
            _ => break,
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
        }
        _ => equality_expression(buffer),
    }
}

fn equality_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = relational_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        let op = match token.token_type {
            TokenType::EqualEqual => Some(ComparisonOperator::Equals),
            TokenType::BangEqual => Some(ComparisonOperator::NotEquals),
            _ => None,
        };

        match op {
            Some(op) => {
                buffer.advance();
                let right = relational_expression(buffer)?;
                left = Expression::Comparison(Box::new(left), op, Box::new(right))
            }
            None => break,
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
        _ => None,
    };

    match op {
        Some(op) => {
            buffer.advance();
            let right = additive_expression(buffer)?;
            Ok(Expression::Comparison(Box::new(left), op, Box::new(right)))
        }
        None => Ok(left),
    }
}

fn additive_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut left = multiplicative_expression(buffer)?;

    loop {
        let token = get_current_token(buffer)?;
        let op = match token.token_type {
            TokenType::Plus => Some(MathOperator::Add),
            TokenType::Minus => Some(MathOperator::Subtract),
            _ => None,
        };

        match op {
            Some(op) => {
                buffer.advance();
                let right = multiplicative_expression(buffer)?;
                left = Expression::BinaryOp(Box::new(left), op, Box::new(right))
            }
            None => break,
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
            _ => None,
        };

        match op {
            Some(op) => {
                buffer.advance();
                let right = prefix_expression(buffer)?;
                left = Expression::BinaryOp(Box::new(left), op, Box::new(right))
            }
            None => break,
        }
    }

    Ok(left)
}

fn prefix_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;

    let op = match token.token_type {
        TokenType::Minus => Some(UnaryOperator::USub),
        TokenType::Plus => Some(UnaryOperator::UAdd),
        _ => None,
    };

    match op {
        Some(op) => {
            buffer.advance();
            let operand = prefix_expression(buffer)?;
            Ok(Expression::UnaryOp(op, Box::new(operand)))
        }
        _ => exponential_expression(buffer),
    }
}

fn exponential_expression(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let mut stack = vec![];
    let mut left = atomic_expression(buffer)?;
    stack.push(left);

    loop {
        let token = get_current_token(buffer)?;

        match token.token_type {
            TokenType::Caret => {
                buffer.advance();
                left = atomic_expression(buffer)?;
                stack.push(left);
            }
            _ => break,
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
                }
                _ => Ok(id),
            }
        }
        TokenType::LeftParen => {
            buffer.advance();
            let expr = expression(buffer);

            let token = get_current_token(buffer)?;
            match token.token_type {
                TokenType::RightParen => {
                    buffer.advance();
                    expr
                }
                _ => Err(SimpleError::new("Expected a closing parentheses".into())),
            }
        }
        _ => constant(buffer),
    }
}

fn function_call_expression(
    buffer: &mut LookaheadBuffer<Token>,
    id: Expression,
) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;

    match token.token_type {
        TokenType::RightParen => {
            buffer.advance();
            Ok(Expression::Call(Box::new(id), vec![]))
        }
        _ => {
            let args = function_args(buffer)?;

            let token = get_current_token(buffer)?;
            match token.token_type {
                TokenType::RightParen => {
                    buffer.advance();
                    Ok(Expression::Call(Box::new(id), args))
                }
                _ => Err(SimpleError::new(
                    "Expected a closing parentheses in function call".into(),
                )),
            }
        }
    }
}

fn function_args(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Vec<Expression>> {
    let mut args: Vec<Expression> = vec![];

    let next_arg = expression(buffer)?;
    args.push(next_arg);

    loop {
        let token = get_current_token(buffer)?;
        match token.token_type {
            TokenType::Comma => {
                buffer.advance();
                let next_arg = expression(buffer)?;
                args.push(next_arg);
            }
            _ => break,
        }
    }

    Ok(args)
}

fn constant(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Expression> {
    let token = get_current_token(buffer)?;
    buffer.advance();
    match token.token_type {
        TokenType::NumberLiteral(value) => Ok(Expression::Number(value)),
        TokenType::StringLiteral(value) => Ok(Expression::Str(value)),
        TokenType::True => Ok(Expression::Bool(true)),
        TokenType::False => Ok(Expression::Bool(false)),
        _ => Err(SimpleError::new(format!(
            "Unexpected Token: {:?} on line: {}, column: {}",
            token.token_type, token.line, token.column
        ))),
    }
}

fn get_current_token(buffer: &mut LookaheadBuffer<Token>) -> HandlerResult<Token> {
    let optional_token = buffer.peek(0);

    match optional_token {
        Some(token) => Ok(token),
        None => Err(SimpleError::new("No EOF token found...".into())),
    }
}
