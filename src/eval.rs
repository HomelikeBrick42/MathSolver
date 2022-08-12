use crate::{Atom, Expression, Term};

pub fn eval_atom(atom: &Atom) -> f64 {
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

pub fn eval_term(term: &Term) -> f64 {
    term.atoms.iter().map(eval_atom).product()
}

pub fn eval_expression(expression: &Expression) -> f64 {
    expression.terms.iter().map(eval_term).sum()
}
