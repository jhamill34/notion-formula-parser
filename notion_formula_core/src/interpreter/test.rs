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
#[test]
fn test_unary_sub_operation() {
    let input = Expression::UnaryOp(
        UnaryOperator::USub,
        Box::new(Expression::Number("123".into())),
    );
    let result = interpret(input).unwrap();

    assert_eq!(RuntimeType::Num(-123.0), result);
}
#[test]
fn test_unary_add_operation_with_number() {
    let input = Expression::UnaryOp(
        UnaryOperator::UAdd,
        Box::new(Expression::Number("123".into())),
    );
    let result = interpret(input).unwrap();

    assert_eq!(RuntimeType::Num(123.0), result);
}
#[test]
fn test_unary_add_operation_with_string() {
    let input =
        Expression::UnaryOp(UnaryOperator::UAdd, Box::new(Expression::Str("123".into())));
    let result = interpret(input).unwrap();

    assert_eq!(RuntimeType::Num(123.0), result);
}
#[test]
fn test_unary_add_operation_with_bool() {
    let input = Expression::UnaryOp(UnaryOperator::UAdd, Box::new(Expression::Bool(true)));
    let result = interpret(input).unwrap();

    assert_eq!(RuntimeType::Num(1.0), result);
}
#[test]
fn test_not_operation_with_bool() {
    let input = Expression::UnaryOp(UnaryOperator::Not, Box::new(Expression::Bool(true)));
    let result = interpret(input).unwrap();

    assert_eq!(RuntimeType::Bool(false), result);
}
#[test]
fn test_ternary_operation() {
    let input = Expression::TernaryOp(
        Box::new(Expression::Comparison(
            Box::new(Expression::Number("1".into())),
            ComparisonOperator::Equals,
            Box::new(Expression::Number("2".into())),
        )),
        Box::new(Expression::Str("Cool".into())),
        Box::new(Expression::Str("Beans".into())),
    );
    let result = interpret(input).unwrap();

    assert_eq!(RuntimeType::Str("Beans".into()), result);
}
