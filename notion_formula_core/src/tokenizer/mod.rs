mod util;

use lookahead_buffer::LookaheadBuffer;
use pipeline::HandlerResult;
use util::*;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,
    QuestionMark,
    Colon,
    Plus,
    Minus,
    Slash,
    Star,
    Percent,
    Caret,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    True,
    False,
    And,
    Or,
    Not,
    Eof,
    Unknown(char),
    Ignored,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub column: u32,
}
impl Token {
    pub fn new(token_type: TokenType, line: u32, column: u32) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}

pub fn tokenizer(input: Vec<char>) -> HandlerResult<Vec<Token>> {
    let mut result: Vec<Token> = Vec::new();
    let mut buffer = LookaheadBuffer::new(input);
    let mut column = 1;
    let mut line = 1;

    while let Some(value) = buffer.peek(0) {
        buffer.advance();
        let token_type = match value {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            ',' => TokenType::Comma,
            '?' => TokenType::QuestionMark,
            ':' => TokenType::Colon,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            '^' => TokenType::Caret,
            '>' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some('=') => {
                        buffer.advance();
                        TokenType::GreaterEqual
                    }
                    _ => TokenType::Greater,
                }
            }
            '<' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some('=') => {
                        buffer.advance();
                        TokenType::LessEqual
                    }
                    _ => TokenType::Less,
                }
            }
            '=' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some('=') => {
                        buffer.advance();
                        TokenType::EqualEqual
                    }
                    _ => TokenType::Unknown(value),
                }
            }
            '!' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some('=') => {
                        buffer.advance();
                        TokenType::BangEqual
                    }
                    _ => TokenType::Unknown(value),
                }
            }
            '"' => {
                loop {
                    match buffer.peek(0) {
                        Some('"') => {
                            buffer.advance();
                            break;
                        }
                        None => panic!("Couldn't find the end of the string, missing '\"'"),
                        _ => buffer.advance(),
                    }
                }

                let str_literal = buffer.get_slice().iter().collect();
                TokenType::StringLiteral(str_literal)
            }
            ' ' | '\r' | '\t' => TokenType::Ignored,
            '\n' => {
                line = line + 1;
                column = 0;
                TokenType::Ignored
            }
            '0'..='9' => {
                consume_number_literal(&mut buffer);
                let num_literal = buffer.get_slice().iter().collect();
                TokenType::NumberLiteral(num_literal)
            }
            'a'..='z' | 'A'..='Z' => {
                while let Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') = buffer.peek(0) {
                    buffer.advance();
                }

                check_keyword(&buffer.get_slice())
            }
            _ => TokenType::Unknown(value),
        };

        match token_type {
            TokenType::Ignored => (),
            TokenType::Unknown(value) => {
                panic!(format!("Unknown character found {}", value as char))
            }
            _ => result.push(Token {
                token_type,
                line,
                column,
            }),
        }
        column = column + buffer.get_slice().len() as u32;
        buffer.commit();
    }

    result.push(Token {
        token_type: TokenType::Eof,
        line,
        column,
    });

    Ok(result)
}

fn check_keyword(input: &[char]) -> TokenType {
    let result: String = input.iter().collect();
    match result.as_str() {
        "or" => TokenType::Or,
        "and" => TokenType::And,
        "not" => TokenType::Not,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Identifier(result),
    }
}

#[cfg(test)]
mod test;
