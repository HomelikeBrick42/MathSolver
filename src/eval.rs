use num_rational::BigRational;

use crate::{Atom, Expression, Term};

pub fn eval_atom(atom: &Atom) -> BigRational {
    match atom {
        Atom::Number(value) => value.clone(),
        Atom::Variable(_) => unreachable!("cannot eval variables"),
        Atom::Group(expression) => eval_expression(expression),
        Atom::Fraction {
            numerator,
            denominator,
        } => eval_expression(numerator) / eval_expression(denominator),
    }
}

pub fn eval_term(term: &Term) -> BigRational {
    term.atoms.iter().map(eval_atom).product()
}

pub fn eval_expression(expression: &Expression) -> BigRational {
    expression.terms.iter().map(eval_term).sum()
}
