use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;
use phf::phf_map;

use derive_more::{Display, IsVariant};
use enum_as_inner::EnumAsInner;

use crate::{SourceLocation, SourceSpan, Token, TokenData, TokenKind};

static SINGLE_CHAR_TOKENS: phf::Map<char, TokenKind> = phf_map! {
    '\0' => TokenKind::EOF,
    '(' => TokenKind::OpenParenthesis,
    ')' => TokenKind::CloseParenthesis,
    '+' => TokenKind::Plus,
    '-' => TokenKind::Minus,
    '*' => TokenKind::Multiply,
    '/' => TokenKind::Divide,
    '=' => TokenKind::Equal,
};

#[derive(Clone, PartialEq, Debug, Display, IsVariant, EnumAsInner)]
pub enum LexerError {
    #[display(fmt = "{span}: Unexpected character '{character}'")]
    UnexpectedCharacter { span: SourceSpan, character: char },
}

#[derive(Clone)]
pub struct Lexer {
    filepath: String,
    source: Rc<Vec<char>>,
    position: usize,
    location: SourceLocation,
}

impl Lexer {
    pub fn new(filepath: &str, source: &str) -> Lexer {
        Lexer {
            filepath: filepath.to_string(),
            source: source.chars().collect::<Vec<_>>().into(),
            position: 0,
            location: SourceLocation { line: 1, column: 1 },
        }
    }

    pub fn peek_token(&self) -> Result<Token, LexerError> {
        self.clone().next_token()
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        while self.peek_char().is_whitespace() {
            self.next_char();
        }

        let start_location = self.location.clone();
        if self.peek_char().is_ascii_digit() {
            let base = BigInt::from(10);
            let mut value = BigInt::from(0);
            let mut denominator = BigInt::from(1);

            while self.peek_char().is_ascii_digit() {
                let digit_value = self.next_char() as u8 - '0' as u8;
                value *= base.clone();
                value += BigInt::from(digit_value);
            }
            if self.peek_char() == '.' {
                self.next_char();
                while self.peek_char().is_ascii_digit() {
                    let digit_value = self.next_char() as u8 - '0' as u8;
                    denominator *= base.clone();
                    value *= base.clone();
                    value += BigInt::from(digit_value);
                }
            }

            Ok(Token {
                kind: TokenKind::Number,
                data: TokenData::Number(BigRational::new(value, denominator)),
                span: SourceSpan {
                    filepath: self.filepath.clone(),
                    start: start_location,
                    end: self.location.clone(),
                },
            })
        } else if self.peek_char().is_alphanumeric() {
            let mut name = String::new();
            while self.peek_char().is_alphanumeric() {
                name.push(self.next_char());
            }
            Ok(Token {
                kind: TokenKind::Name,
                data: TokenData::String(name),
                span: SourceSpan {
                    filepath: self.filepath.clone(),
                    start: start_location,
                    end: self.location.clone(),
                },
            })
        } else {
            let chr = self.next_char();
            if SINGLE_CHAR_TOKENS.contains_key(&chr) {
                Ok(Token {
                    kind: SINGLE_CHAR_TOKENS[&chr].clone(),
                    data: TokenData::None,
                    span: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                })
            } else {
                Err(LexerError::UnexpectedCharacter {
                    span: SourceSpan {
                        filepath: self.filepath.clone(),
                        start: start_location,
                        end: self.location.clone(),
                    },
                    character: chr,
                })
            }
        }
    }

    fn peek_char(&self) -> char {
        if self.position < self.source.len() {
            self.source[self.position]
        } else {
            '\0'
        }
    }

    fn next_char(&mut self) -> char {
        let current = self.peek_char();
        if current != '\0' {
            self.position += 1;
            self.location.column += 1;
            if current == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            }
        }
        current
    }
}
