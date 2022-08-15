use derive_more::Display;
use enum_as_inner::EnumAsInner;
use num_rational::BigRational;

use crate::SourceSpan;

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum TokenKind {
    EOF,
    Name,
    Number,
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
}

#[derive(Clone, PartialEq, Debug, EnumAsInner)]
pub enum TokenData {
    None,
    String(String),
    Number(BigRational),
}

#[derive(Clone, PartialEq, Debug, Display)]
#[display(fmt = "{kind}")]
pub struct Token {
    pub kind: TokenKind,
    pub data: TokenData,
    pub span: SourceSpan,
}
