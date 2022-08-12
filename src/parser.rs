use std::iter::Peekable;

use crate::Equation;

pub fn parse_equation(source: &mut Peekable<impl Iterator<Item = char>>) -> Equation {
    todo!()
}

fn parse_term(source: &mut Peekable<impl Iterator<Item = char>>) {}

fn skip_whitespace(source: &mut Peekable<impl Iterator<Item = char>>) {
    while source
        .peek()
        .map(|&chr| char::is_whitespace(chr))
        .unwrap_or(false)
    {
        source.next();
    }
}
