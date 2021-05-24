mod util;

use crate::tokenizer::Token;
use lookahead_buffer::LookaheadBuffer;
use pipeline::HandlerResult;

#[derive(Debug, PartialEq)]
struct Document {
    statements: Vec<Statement>
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    TableDef(String, Vec<Pair<Type>>),
    FormulaDef(String, Expression),
    Assignment(String, Expression),
    PrintStatement(Expression),
    AssertStatement(Expression)
}

pub enum Type {
    Str,
    Number,
    Bool,
    Formula(Expression)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOp(Box<Expression>, MathOperator, Box<Expression>),
    Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
    BooleanOp(Box<Expression>, BooleanOperator, Box<Expression>),
    UnaryOp(UnaryOperator, Box<Expression>),
    TernaryOp(Box<Expression>, Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Identifier(String),
    Access(Box<Expression>, Box<Expression>),
    TableInstance(String, Vec<Pair<Expression>>),
    Str(String),
    Number(String),
    Bool(bool),
}

struct Pair<T> {
    key: String,
    value: T
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
mod test;
