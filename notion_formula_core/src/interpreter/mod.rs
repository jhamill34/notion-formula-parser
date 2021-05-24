use crate::parser::BooleanOperator;
use crate::parser::ComparisonOperator;
use crate::parser::Expression;
use crate::parser::MathOperator;
use crate::parser::UnaryOperator;
use pipeline::HandlerResult;
use pipeline::SimpleError;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum RuntimeType {
    Num(f64),
    Str(String),
    Bool(bool),
}

fn visit_expression(input: Expression) -> HandlerResult<RuntimeType> {
    use BooleanOperator::*;
    use ComparisonOperator::*;
    use MathOperator::*;
    use UnaryOperator::*;
    use RuntimeType::*;

    match input {
        Expression::BinaryOp(lhs, op, rhs) => {
            let left_result = visit_expression(*lhs)?;
            let right_result = visit_expression(*rhs)?;
            let pair = (left_result, right_result);

            match pair {
                (Num(left_value), Num(right_value)) => {
                    let result = match op {
                        Add => left_value + right_value,
                        Subtract => left_value - right_value,
                        Mod => left_value % right_value,
                        Multiply => left_value * right_value,
                        Divide => left_value / right_value,
                        Exponent => left_value.powf(right_value),
                    };
                    Ok(Num(result))
                }
                (Str(left_value), Str(right_value)) => {
                    let result = match op {
                        Add => format!("{}{}", left_value, right_value),
                        _ => {
                            return Err(SimpleError::new(format!(
                                "Invalid value {:?}, for binary operation",
                                left_value
                            )))
                        }
                    };
                    Ok(Str(result))
                }
                _ => Err(SimpleError::new(format!(
                    "Invalid values {:?}, {:?}, for binary operation",
                    pair.0, pair.1
                ))),
            }
        }
        Expression::Comparison(lhs, op, rhs) => {
            let left_result = visit_expression(*lhs)?;
            let right_result = visit_expression(*rhs)?;

            if !is_same_type(&left_result, &right_result) {
                return Err(SimpleError::new(format!(
                    "Can't compare two values of diferent types: {:?} and {:?}",
                    left_result, right_result
                )));
            }

            let result = match op {
                Equals => left_result == right_result,
                NotEquals => left_result != right_result,
                LessThan => left_result < right_result,
                LessThanEq => left_result <= right_result,
                GreaterThan => left_result > right_result,
                GreaterThanEq => left_result >= right_result,
            };

            Ok(Bool(result))
        }
        Expression::BooleanOp(lhs, op, rhs) => {
            let left_result = visit_expression(*lhs)?;
            let right_result = visit_expression(*rhs)?;
            let pair = (left_result, right_result);

            match pair {
                (Bool(left_value), Bool(right_value)) => {
                    let result = match op {
                        And => left_value && right_value,
                        Or => left_value || right_value,
                    };
                    Ok(Bool(result))
                }
                _ => Err(SimpleError::new(format!(
                    "Boolean operations only accept booleans: {:?}, {:?}",
                    pair.0, pair.1
                ))),
            }
        }
        Expression::UnaryOp(op, rhs) => {
            let result = visit_expression(*rhs)?;
            match op {
                UAdd => {
                    match result {
                        Str(value) => {
                            let result = value.parse::<f64>()?;
                            Ok(Num(result))
                        },
                        Bool(value) => Ok(Num(value as u8 as f64)),
                        _ => Ok(result)
                    }
                }
                USub => {
                    match result {
                        Num(value) => Ok(Num(-value)),
                        _ => Err(SimpleError::new(format!(
                            "Can't use unary minus on non number values: {:?}",
                            result
                        )))
                    }
                }
                Not => {
                    match result {
                        Bool(value) => Ok(Bool(!value)),
                        _ => Err(SimpleError::new(format!(
                            "Can't perform boolean operations on non boolean values: {:?}",
                            result
                        )))
                    }
                }
            }
        }
        Expression::TernaryOp(test, accept, reject) => {
            let test_result = visit_expression(*test)?;
            let accept_result = visit_expression(*accept)?;
            let reject_result = visit_expression(*reject)?;

            match test_result {
                Bool(test_value) => {
                    if !is_same_type(&accept_result, &reject_result) {
                        Err(SimpleError::new(format!(
                            "Each branch of a condition must be the same type: {:?} and {:?}",
                            accept_result, reject_result
                        )))
                    } else if test_value {
                        Ok(accept_result)
                    } else {
                        Ok(reject_result)
                    }
                }
                _ => Err(SimpleError::new(format!(
                    "Result of test needs to be a boolean: {:?}",
                    test_result
                )))
            }
        }
        Expression::Call(_, _) => {
            unimplemented!()
        }
        Expression::Identifier(_) => {
            unimplemented!()
        }
        Expression::Str(value) => Ok(Str(value)),
        Expression::Number(value) => Ok(Num(value.parse::<f64>()?)),
        Expression::Bool(value) => Ok(Bool(value)),
    }
}

fn is_same_type(a: &RuntimeType, b: &RuntimeType) -> bool {
    use RuntimeType::*;
    match (a, b) {
        (Num(_), Num(_)) | (Str(_), Str(_)) | (Bool(_), Bool(_)) => true,
        _ => false,
    }
}

pub fn interpret(input: Expression) -> HandlerResult<RuntimeType> {
    visit_expression(input)
}

#[cfg(test)]
mod test;
