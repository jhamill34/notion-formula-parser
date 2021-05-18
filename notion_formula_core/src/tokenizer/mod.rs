mod util;

use lookahead_buffer::LookaheadBuffer;
use pipeline::{ HandlerResult };
use util::*;

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
                        None => panic!("Unterminated string literal"),
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
                check_keyword(&buffer.get_slice())
            }
            _ => TokenType::Unknown(value),
        };

        match token_type {
            TokenType::Ignored | TokenType::Unknown(_) => (),
            _ => result.push(Token {
                token_type,
                line: 1,
                column: 1,
            }),
        }
        buffer.commit();
    }

    Ok(result)
}

fn check_keyword(input: &[u8]) -> TokenType {
    match std::str::from_utf8(&input) {
        Ok("or") => TokenType::Or,
        Ok("and") => TokenType::And,
        Ok("not") => TokenType::Not,
        Ok("true") => TokenType::True,
        Ok("false") => TokenType::False,
        Ok(value) => TokenType::Identifier(value.into()),
        Err(e) => panic!(e),
    }
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
                    column: 1
                },
                Token {
                    token_type: Comma,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: QuestionMark,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Colon,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Plus,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Minus,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Star,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Percent,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Caret,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Slash,
                    line: 1,
                    column: 1
                },
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
                    column: 1
                },
                Token {
                    token_type: Greater,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: EqualEqual,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Less,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: BangEqual,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: EqualEqual,
                    line: 1,
                    column: 1
                },
            ],
            result
        )
    }

    #[test]
    fn test_can_handle_string_literals() {
        let input: Vec<u8> = "\"hello world\"".into();
        let result = tokenizer(input).unwrap();

        assert_eq!(
            vec![Token {
                token_type: StringLiteral("\"hello world\"".into()),
                line: 1,
                column: 1
            },],
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
                    column: 1
                },
                Token {
                    token_type: NumberLiteral("2e3".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: NumberLiteral("2E4".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: NumberLiteral("2e-1".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: NumberLiteral("2e+4".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: NumberLiteral("1.2E+3".into()),
                    line: 1,
                    column: 1
                },
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
                    column: 1
                },
                Token {
                    token_type: Identifier("Bar".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Or,
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Identifier("baz".into()),
                    line: 1,
                    column: 1
                },
                Token {
                    token_type: Not,
                    line: 1,
                    column: 1
                },
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
