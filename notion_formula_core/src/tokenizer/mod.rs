mod util;

use lookahead_buffer::LookaheadBuffer;
use pipeline::{ HandlerResult };
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
    token_type: TokenType,
    line: u32,
    column: u32,
}
impl Token {
    pub fn new(token_type: TokenType, line: u32, column: u32) -> Self {
        Token { token_type, line, column }
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
            },
            '0'..='9' => {
                consume_number_literal(&mut buffer);
                let num_literal = buffer.get_slice().iter().collect();
                TokenType::NumberLiteral(num_literal)
            }
            'a'..='z' | 'A'..='Z' => {
                while let Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') = buffer.peek(0)
                {
                    buffer.advance();
                }

                check_keyword(&buffer.get_slice())
            }
            _ => TokenType::Unknown(value),
        };

        match token_type {
            TokenType::Ignored => (),
            TokenType::Unknown(value) => panic!(format!("Unknown character found {}", value as char)),
            _ => result.push(Token {
                token_type,
                line,
                column,
            }),
        }
        column = column + buffer.get_slice().len() as u32;
        buffer.commit();
    }

    result.push(Token { token_type: TokenType::Eof, line, column });

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
        _ => TokenType::Identifier(result)
    }
}

#[cfg(test)]
mod test {
    use super::TokenType::*;
    use super::*;

    #[test]
    fn test_can_handle_single_byte_tokens() {
        let input: Vec<char> = "( ) , ? : + - * % ^ /".chars().collect();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: LeftParen,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: RightParen,
                    line: 1,
                    column: 3
                },
                Token {
                    token_type: Comma,
                    line: 1,
                    column: 5
                },
                Token {
                    token_type: QuestionMark,
                    line: 1,
                    column: 7
                },
                Token {
                    token_type: Colon,
                    line: 1,
                    column: 9
                },
                Token {
                    token_type: Plus,
                    line: 1,
                    column: 11
                },
                Token {
                    token_type: Minus,
                    line: 1,
                    column: 13
                },
                Token {
                    token_type: Star,
                    line: 1,
                    column: 15
                },
                Token {
                    token_type: Percent,
                    line: 1,
                    column: 17
                },
                Token {
                    token_type: Caret,
                    line: 1,
                    column: 19
                },
                Token {
                    token_type: Slash,
                    line: 1,
                    column: 21
                },
                Token {
                    token_type: Eof,
                    line: 1,
                    column: 22
                }
            ],
            result
        )
    }

    #[test]
    fn test_can_handle_double_byte_tokens() {
        let input: Vec<char> = ">= <= > == < != ==".chars().collect();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: GreaterEqual,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: LessEqual,
                    line: 1,
                    column: 4
                },
                Token {
                    token_type: Greater,
                    line: 1,
                    column: 7
                },
                Token {
                    token_type: EqualEqual,
                    line: 1,
                    column: 9
                },
                Token {
                    token_type: Less,
                    line: 1,
                    column: 12
                },
                Token {
                    token_type: BangEqual,
                    line: 1,
                    column: 14
                },
                Token {
                    token_type: EqualEqual,
                    line: 1,
                    column: 17
                },
                Token {
                    token_type: Eof,
                    line: 1,
                    column: 19
                }
            ],
            result
        )
    }

    #[test]
    fn test_can_handle_string_literals() {
        let input: Vec<char> = "\"hello world 😀\"".chars().collect();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: StringLiteral("\"hello world 😀\"".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Eof,
                    line: 1,
                    column: 16
                }
            ],
            result
        )
    }

    #[test]
    fn test_can_handle_number_literals() {
        let input: Vec<char> = "123 123.456 2e3 2E4 2e-1 2e+4 1.2E+3".chars().collect();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: NumberLiteral("123".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: NumberLiteral("123.456".into()),
                    line: 1,
                    column: 5
                },
                Token {
                    token_type: NumberLiteral("2e3".into()),
                    line: 1,
                    column: 13
                },
                Token {
                    token_type: NumberLiteral("2E4".into()),
                    line: 1,
                    column: 17
                },
                Token {
                    token_type: NumberLiteral("2e-1".into()),
                    line: 1,
                    column: 21
                },
                Token {
                    token_type: NumberLiteral("2e+4".into()),
                    line: 1,
                    column: 26
                },
                Token {
                    token_type: NumberLiteral("1.2E+3".into()),
                    line: 1,
                    column: 31
                },
                Token {
                    token_type: Eof,
                    line: 1,
                    column: 37
                }
            ],
            result
        )
    }

    #[test]
    fn test_can_handle_identifiers() {
        let input: Vec<char> = "foo and Bar or baz not".chars().collect();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: Identifier("foo".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: And,
                    line: 1,
                    column: 5
                },
                Token {
                    token_type: Identifier("Bar".into()),
                    line: 1,
                    column: 9
                },
                Token {
                    token_type: Or,
                    line: 1,
                    column: 13
                },
                Token {
                    token_type: Identifier("baz".into()),
                    line: 1,
                    column: 16
                },
                Token {
                    token_type: Not,
                    line: 1,
                    column: 20
                },
                Token {
                    token_type: Eof,
                    line: 1,
                    column: 23
                }
            ],
            result
        )
    }

    #[test]
    fn test_multiple_lines() {
        let input: Vec<char> = "foo\nbar baz".chars().collect();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: Identifier("foo".into()),
                    line: 1,
                    column: 1,
                },
                Token {
                    token_type: Identifier("bar".into()),
                    line: 2,
                    column: 1,
                },
                Token {
                    token_type: Identifier("baz".into()),
                    line: 2,
                    column: 5,
                },
                Token {
                    token_type: Eof,
                    line: 2,
                    column: 8
                }
            ],
            result
        )
    }

    #[test]
    #[ignore]
    fn test_error_on_unclosed_string_literal() {
        let input: Vec<char> = "\"hello world".chars().collect();
        let _result = tokenizer(input).unwrap();

        // Not sure about assertion yet...
    }

    #[test]
    #[ignore]
    fn test_non_alpha_numeric_in_identifier_causes_error() {
        let input: Vec<char> = "😀".chars().collect();
        let result = tokenizer(input).unwrap();
    }
}
