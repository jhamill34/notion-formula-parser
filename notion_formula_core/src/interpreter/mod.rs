use crate::parser::BooleanOperator;
use crate::parser::ComparisonOperator;
use crate::parser::Expression;
use crate::parser::MathOperator;
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
        Expression::UnaryOp(_, _) => {
            unimplemented!()
        }
        Expression::TernaryOp(_, _, _) => {
            unimplemented!()
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
mod test {
    use super::*;

    #[test]
    fn test_simple_math() {
        let input = Expression::BinaryOp(
            Box::new(Expression::Number("2e2".into())),
            MathOperator::Add,
            Box::new(Expression::Number("2".into())),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Num(202.0), result);
    }

    #[test]
    fn test_complex_math() {
        let input = Expression::BinaryOp(
            Box::new(Expression::Number("1".into())),
            MathOperator::Add,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number("2".into())),
                MathOperator::Multiply,
                Box::new(Expression::Number("3".into())),
            )),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Num(7.0), result);
    }

    #[test]
    fn test_concat_strings() {
        let input = Expression::BinaryOp(
            Box::new(Expression::Str("hello".into())),
            MathOperator::Add,
            Box::new(Expression::Str(", world".into())),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Str("hello, world".into()), result);
    }

    #[test]
    fn test_comparison_of_numbers() {
        let input = Expression::Comparison(
            Box::new(Expression::Number("123".into())),
            ComparisonOperator::GreaterThan,
            Box::new(Expression::Number("10".into())),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Bool(true), result);
    }
    #[test]
    fn test_comparison_of_strings() {
        let input = Expression::Comparison(
            Box::new(Expression::Str("beta".into())),
            ComparisonOperator::Equals,
            Box::new(Expression::Str("beta".into())),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Bool(true), result);
    }
    #[test]
    fn test_comparison_of_booleans() {
        let input = Expression::Comparison(
            Box::new(Expression::Bool(false)),
            ComparisonOperator::LessThan,
            Box::new(Expression::Bool(true)),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Bool(true), result);
    }
    #[test]
    fn test_comparison_of_math_expressions() {
        let input = Expression::Comparison(
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number("1".into())),
                MathOperator::Divide,
                Box::new(Expression::Number("2".into())),
            )),
            ComparisonOperator::GreaterThan,
            Box::new(Expression::BinaryOp(
                Box::new(Expression::Number("1".into())),
                MathOperator::Divide,
                Box::new(Expression::Number("10".into())),
            )),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Bool(true), result);
    }
    #[test]
    fn test_and_operation() {
        let input = Expression::BooleanOp(
            Box::new(Expression::Bool(true)),
            BooleanOperator::And,
            Box::new(Expression::Bool(false)),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Bool(false), result);
    }
    #[test]
    fn test_or_operation() {
        let input = Expression::BooleanOp(
            Box::new(Expression::Bool(true)),
            BooleanOperator::Or,
            Box::new(Expression::Bool(false)),
        );
        let result = interpret(input).unwrap();

        assert_eq!(RuntimeType::Bool(true), result);
    }
}
