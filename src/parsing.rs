use derive_more::{Display, IsVariant};
use enum_as_inner::EnumAsInner;
use num_rational::BigRational;

use crate::{Atom, Equation, Expression, Lexer, LexerError, Term, Token, TokenKind};

#[derive(Clone, PartialEq, Debug, Display, IsVariant, EnumAsInner)]
pub enum ParsingError {
    LexerError(LexerError),
    #[display(fmt = "{}: Expected {expected}, but got {got}", "got.span")]
    ExpectedToken {
        expected: TokenKind,
        got: Token,
    },
    #[display(fmt = "{}: Expected atom, but got {got}", "got.span")]
    ExpectedAtom {
        got: Token,
    },
}

impl From<LexerError> for ParsingError {
    fn from(error: LexerError) -> ParsingError {
        ParsingError::LexerError(error)
    }
}

fn parse_atom(lexer: &mut Lexer) -> Result<Atom, ParsingError> {
    Ok(match lexer.peek_token()?.kind {
        TokenKind::Number => {
            let token = expect_token(lexer, TokenKind::Number)?;
            Atom::Number(token.data.into_number().unwrap())
        }

        TokenKind::Name => {
            let token = expect_token(lexer, TokenKind::Name)?;
            Atom::Variable(token.data.into_string().unwrap())
        }

        TokenKind::OpenParenthesis => {
            expect_token(lexer, TokenKind::OpenParenthesis)?;
            let expression = parse_expression(lexer)?;
            expect_token(lexer, TokenKind::CloseParenthesis)?;
            Atom::Group(expression)
        }

        _ => {
            return Err(ParsingError::ExpectedAtom {
                got: lexer.next_token()?,
            });
        }
    })
}

fn parse_term(lexer: &mut Lexer) -> Result<Term, ParsingError> {
    let mut atoms = vec![parse_atom(lexer)?];
    while matches!(
        lexer.peek_token()?.kind,
        TokenKind::Multiply | TokenKind::Divide
    ) {
        let operator = lexer.next_token()?;
        match operator.kind {
            TokenKind::Multiply => {
                let atom = parse_atom(lexer)?;
                atoms.push(atom);
            }
            TokenKind::Divide => {
                atoms = vec![Atom::Fraction {
                    numerator: Expression {
                        terms: vec![Term { atoms }],
                    },
                    denominator: Expression {
                        terms: vec![Term {
                            atoms: vec![parse_atom(lexer)?],
                        }],
                    },
                }];
            }
            _ => unreachable!(),
        }
    }
    Ok(Term { atoms })
}

fn parse_expression(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
    let mut terms = vec![parse_term(lexer)?];
    while matches!(
        lexer.peek_token()?.kind,
        TokenKind::Plus | TokenKind::Minus | TokenKind::Divide
    ) {
        let operator = lexer.next_token()?;
        match operator.kind {
            TokenKind::Plus => terms.push(parse_term(lexer)?),
            TokenKind::Minus => {
                let mut term = parse_term(lexer)?;
                term.atoms
                    .push(Atom::Number(BigRational::from_float(-1.0).unwrap()));
                terms.push(term);
            }
            _ => unreachable!(),
        }
    }
    Ok(Expression { terms })
}

pub fn parse_equation(lexer: &mut Lexer) -> Result<Equation, ParsingError> {
    let left = parse_expression(lexer)?;
    expect_token(lexer, TokenKind::Equal)?;
    let right = parse_expression(lexer)?;
    expect_token(lexer, TokenKind::EOF)?;
    Ok(Equation::Equality { left, right })
}

fn expect_token(lexer: &mut Lexer, kind: TokenKind) -> Result<Token, ParsingError> {
    let token = lexer.next_token()?;
    if token.kind != kind {
        return Err(ParsingError::ExpectedToken {
            expected: kind,
            got: token,
        });
    }
    Ok(token)
}
