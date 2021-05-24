use super::TokenType::*;
use super::*;

#[test]
fn test_can_handle_single_byte_tokens() {
    let input: Vec<char> = "( ) , ? : + - * % ^ / { } @ ;".chars().collect();
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
                token_type: LeftBracket,
                line: 1,
                column: 23
            },
            Token {
                token_type: RightBracket,
                line: 1,
                column: 25
            },
            Token {
                token_type: At,
                line: 1,
                column: 27
            },
            Token {
                token_type: SemiColon,
                line: 1,
                column: 29
            },
            Token {
                token_type: Eof,
                line: 1,
                column: 30
            }
        ],
        result
    )
}

#[test]
fn test_can_handle_double_byte_tokens() {
    let input: Vec<char> = ">= <= > == < != = ==".chars().collect();
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
                token_type: Equal,
                line: 1,
                column: 17
            },
            Token {
                token_type: EqualEqual,
                line: 1,
                column: 19
            },
            Token {
                token_type: Eof,
                line: 1,
                column: 21
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
    let input: Vec<char> = "foo and Bar or baz not assert print let table formula".chars().collect();
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
                token_type: Assert,
                line: 1,
                column: 24
            },
            Token {
                token_type: Print,
                line: 1,
                column: 31
            },
            Token {
                token_type: Let,
                line: 1,
                column: 37
            },
            Token {
                token_type: Table,
                line: 1,
                column: 41
            },
            Token {
                token_type: Formula,
                line: 1,
                column: 47
            },
            Token {
                token_type: Eof,
                line: 1,
                column: 54
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
    let _result = tokenizer(input).unwrap();
}
