use derive_more::Display;
use enum_as_inner::EnumAsInner;
use num_rational::BigRational;

use crate::SourceSpan;

#[derive(Clone, PartialEq, Debug, Display, EnumAsInner)]
pub enum TokenKind {
    #[display(fmt = "EOF")]
    EOF,
    #[display(fmt = "name")]
    Name,
    #[display(fmt = "number")]
    Number,
    #[display(fmt = "(")]
    OpenParenthesis,
    #[display(fmt = ")")]
    CloseParenthesis,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "*")]
    Multiply,
    #[display(fmt = "/")]
    Divide,
    #[display(fmt = "=")]
    Equal,
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
