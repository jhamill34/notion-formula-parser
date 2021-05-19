mod util;

use lookahead_buffer::LookaheadBuffer;
use pipeline::{ HandlerResult };
use util::*;
use std::str::Utf8Error;

#[derive(Debug, PartialEq)]
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
    Identifier(Vec<u8>),
    StringLiteral(Vec<u8>),
    NumberLiteral(Vec<u8>),
    True,
    False,
    And,
    Or,
    Not,
    Eof,
    Unknown(u8),
    Ignored,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: u32,
    column: u32,
}

pub fn tokenizer(input: Vec<u8>) -> HandlerResult<Vec<Token>> {
    let mut result: Vec<Token> = Vec::new();
    let mut buffer = LookaheadBuffer::new(input);
    let mut column = 1;

    while let Some(value) = buffer.peek(0) {
        buffer.advance();
        let token_type = match value {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b',' => TokenType::Comma,
            b'?' => TokenType::QuestionMark,
            b':' => TokenType::Colon,
            b'+' => TokenType::Plus,
            b'-' => TokenType::Minus,
            b'*' => TokenType::Star,
            b'/' => TokenType::Slash,
            b'%' => TokenType::Percent,
            b'^' => TokenType::Caret,
            b'>' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some(b'=') => {
                        buffer.advance();
                        TokenType::GreaterEqual
                    }
                    _ => TokenType::Greater,
                }
            }
            b'<' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some(b'=') => {
                        buffer.advance();
                        TokenType::LessEqual
                    }
                    _ => TokenType::Less,
                }
            }
            b'=' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some(b'=') => {
                        buffer.advance();
                        TokenType::EqualEqual
                    }
                    _ => TokenType::Unknown(value),
                }
            }
            b'!' => {
                let next_value = buffer.peek(0);
                match next_value {
                    Some(b'=') => {
                        buffer.advance();
                        TokenType::BangEqual
                    }
                    _ => TokenType::Unknown(value),
                }
            }
            b'"' => {
                loop {
                    match buffer.peek(0) {
                        Some(b'"') => {
                            buffer.advance();
                            break;
                        }
                        None => panic!("Couldn't find the end of the string, missing '\"'"),
                        _ => buffer.advance(),
                    }
                }
                TokenType::StringLiteral(buffer.get_slice())
            }
            b' ' | b'\r' | b'\t' | b'\n' => TokenType::Ignored,
            b'0'..=b'9' => {
                consume_number_literal(&mut buffer);
                TokenType::NumberLiteral(buffer.get_slice())
            }
            b'a'..=b'z' | b'A'..=b'Z' => {
                while let Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'0'..=b'9') = buffer.peek(0)
                {
                    buffer.advance();
                }

                match check_keyword(&buffer.get_slice()) {
                    Ok(v) => v,
                    Err(e) => panic!(e)
                }
            }
            _ => TokenType::Unknown(value),
        };

        match token_type {
            TokenType::Ignored => (),
            TokenType::Unknown(_) => (),
            _ => result.push(Token {
                token_type,
                line: 1,
                column,
            }),
        }
        column = column + buffer.get_slice().len() as u32;
        buffer.commit();
    }

    result.push(Token { token_type: TokenType::Eof, line: 1, column });

    Ok(result)
}

fn check_keyword(input: &[u8]) -> Result<TokenType, Utf8Error> {
    std::str::from_utf8(&input).map(|value| {
       match value {
           "or" => TokenType::Or,
           "and" => TokenType::And,
           "not" => TokenType::Not,
           "true" => TokenType::True,
           "false" => TokenType::False,
           _ => TokenType::Identifier(value.into()),
       }
    })
}

#[cfg(test)]
mod test {
    use super::TokenType::*;
    use super::*;

    #[test]
    fn test_can_handle_single_byte_tokens() {
        let input: Vec<u8> = "( ) , ? : + - * % ^ /".into();
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
        let input: Vec<u8> = ">= <= > == < != ==".into();
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
        let input: Vec<u8> = "\"hello world\"".into();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: StringLiteral("\"hello world\"".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Eof,
                    line: 1,
                    column: 14
                }
            ],
            result
        )
    }

    #[test]
    fn test_can_handle_number_literals() {
        let input: Vec<u8> = "123 123.456 2e3 2E4 2e-1 2e+4 1.2E+3".into();
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
        let input: Vec<u8> = "foo and Bar or baz not".into();
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
    #[ignore]
    fn test_error_on_unclosed_string_literal() {
        let input: Vec<u8> = "\"hello world".into();
        let _result = tokenizer(input).unwrap();

        // Not sure about assertion yet...
    }
}
