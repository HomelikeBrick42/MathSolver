use num_rational::BigRational;

use crate::{eval_atom, eval_expression, eval_term, Atom, Equation, Expression, Term};

fn simplify_atom(atom: &Atom) -> Atom {
    match atom {
        Atom::Number(value) => Atom::Number(value.clone()),
        Atom::Variable(name) => Atom::Variable(name.clone()),
        Atom::Group(expression) => {
            if !expression.contains_variable() {
                Atom::Number(eval_expression(expression))
            } else {
                Atom::Group(simplify_expression(expression))
            }
        }
        Atom::Fraction {
            numerator,
            denominator,
        } => {
            if !numerator.contains_variable() && !denominator.contains_variable() {
                Atom::Number(eval_atom(atom))
            } else {
                Atom::Fraction {
                    numerator: simplify_expression(numerator),
                    denominator: simplify_expression(denominator),
                }
            }
        }
    }
}

fn simplify_term(term: &Term) -> Term {
    Term {
        atoms: if !term.contains_variable() {
            vec![Atom::Number(eval_term(term))]
        } else if term.atoms.iter().any(|atom| {
            atom.as_number().map_or(false, |number| {
                number == &BigRational::from_float(0.0).unwrap()
            })
        }) {
            vec![Atom::Number(BigRational::from_float(0.0).unwrap())]
        } else {
            let amount: BigRational = term
                .atoms
                .iter()
                .filter_map(|atom| (!atom.contains_variable()).then(|| eval_atom(atom)))
                .product();
            let other_atoms = term.atoms.iter().filter(|atom| atom.contains_variable());
            if amount == BigRational::from_float(1.0).unwrap() {
                other_atoms.map(simplify_atom).collect()
            } else {
                std::iter::once(&Atom::Number(amount))
                    .chain(other_atoms)
                    .map(simplify_atom)
                    .collect()
            }
        },
    }
}

fn collect_like_terms(terms: &[Term]) -> Vec<Vec<&Term>> {
    fn is_like_term(a: &Term, b: &Term) -> bool {
        let mut a_atoms = a
            .atoms
            .iter()
            .filter(|atom| atom.contains_variable())
            .collect::<Vec<_>>();
        a_atoms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut b_atoms = b
            .atoms
            .iter()
            .filter(|atom| atom.contains_variable())
            .collect::<Vec<_>>();
        b_atoms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        a_atoms == b_atoms
    }
    let mut like_terms = terms
        .iter()
        .map(|a| terms.iter().filter(|b| is_like_term(a, b)).collect())
        .collect::<Vec<Vec<_>>>();
    like_terms.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    like_terms.dedup();
    like_terms
}

fn simplify_expression(expression: &Expression) -> Expression {
    Expression {
        terms: if !expression.contains_variable() {
            vec![Term {
                atoms: vec![Atom::Number(eval_expression(expression))],
            }]
        } else {
            let like_terms = collect_like_terms(&expression.terms);
            let mut terms = like_terms
                .iter()
                .map(|terms| {
                    let amount: BigRational = terms
                        .iter()
                        .map::<BigRational, _>(|term| {
                            term.atoms
                                .iter()
                                .filter_map(|atom| {
                                    (!atom.contains_variable()).then(|| eval_atom(atom))
                                })
                                .product()
                        })
                        .sum();
                    Term {
                        atoms: std::iter::once(&Atom::Number(amount))
                            .chain(
                                terms[0]
                                    .atoms
                                    .iter()
                                    .filter(|atom| atom.contains_variable()),
                            )
                            .cloned()
                            .collect(),
                    }
                })
                .map(|term| simplify_term(&term))
                .collect::<Vec<_>>();
            let mut i = 0;
            while i < terms.len() {
                let term = &mut terms[i];
                if let Some(group) = term
                    .atoms
                    .iter()
                    .enumerate()
                    .find_map(|(i, atom)| atom.is_group().then_some(i))
                {
                    let mut atoms = term.atoms.clone();
                    let group = atoms.remove(group).into_group().unwrap();
                    terms.append(
                        &mut group
                            .terms
                            .iter()
                            .map(|term| Term {
                                atoms: term.atoms.iter().chain(atoms.iter()).cloned().collect(),
                            })
                            .collect(),
                    );
                    terms.remove(i);
                } else {
                    i = i + 1;
                }
            }
            terms
        },
    }
}

fn simplify_equation(equation: &Equation) -> Equation {
    match equation {
        Equation::Equality { left, right } => {
            let (left, right) = {
                let left = simplify_expression(left);
                let right = simplify_expression(right);
                (
                    Expression {
                        terms: left
                            .terms
                            .iter()
                            .filter(|term| term.contains_variable())
                            .cloned()
                            .chain(
                                right
                                    .terms
                                    .iter()
                                    .filter(|term| term.contains_variable())
                                    .map(|term| Term {
                                        atoms: term
                                            .atoms
                                            .iter()
                                            .cloned()
                                            .chain(std::iter::once(Atom::Number(
                                                BigRational::from_float(-1.0).unwrap(),
                                            )))
                                            .collect(),
                                    }),
                            )
                            .collect(),
                    },
                    Expression {
                        terms: right
                            .terms
                            .iter()
                            .filter(|term| !term.contains_variable())
                            .cloned()
                            .chain(
                                left.terms
                                    .iter()
                                    .filter(|term| !term.contains_variable())
                                    .map(|term| Term {
                                        atoms: term
                                            .atoms
                                            .iter()
                                            .cloned()
                                            .chain(std::iter::once(Atom::Number(
                                                BigRational::from_float(-1.0).unwrap(),
                                            )))
                                            .collect(),
                                    }),
                            )
                            .collect(),
                    },
                )
            };
            let equality = if left.terms.len() == 1 {
                let term = &left.terms[0];
                if let Some(fraction) = term
                    .atoms
                    .iter()
                    .enumerate()
                    .find_map(|(i, atom)| atom.as_fraction().map(|_| i))
                {
                    let mut other_atoms = term.atoms.clone();
                    other_atoms.remove(fraction);
                    Equation::Equality {
                        left: Expression {
                            terms: vec![Term {
                                atoms: other_atoms
                                    .into_iter()
                                    .chain(std::iter::once(Atom::Group(
                                        term.atoms[fraction].as_fraction().unwrap().0.clone(),
                                    )))
                                    .collect(),
                            }],
                        },
                        right: Expression {
                            terms: right
                                .terms
                                .iter()
                                .map(|t| Term {
                                    atoms: t
                                        .atoms
                                        .iter()
                                        .cloned()
                                        .chain(std::iter::once(Atom::Group(
                                            term.atoms[fraction].as_fraction().unwrap().1.clone(),
                                        )))
                                        .collect(),
                                })
                                .collect(),
                        },
                    }
                } else {
                    let amount: BigRational = term
                        .atoms
                        .iter()
                        .filter_map(|atom| (!atom.contains_variable()).then(|| eval_atom(atom)))
                        .product();
                    let other_atoms = term
                        .atoms
                        .iter()
                        .filter(|atom| atom.contains_variable())
                        .cloned()
                        .collect::<Vec<_>>();
                    Equation::Equality {
                        left: Expression {
                            terms: vec![Term { atoms: other_atoms }],
                        },
                        right: if amount != BigRational::from_float(1.0).unwrap() {
                            Expression {
                                terms: vec![Term {
                                    atoms: vec![Atom::Fraction {
                                        numerator: right,
                                        denominator: Expression {
                                            terms: vec![Term {
                                                atoms: vec![Atom::Number(amount)],
                                            }],
                                        },
                                    }],
                                }],
                            }
                        } else {
                            right
                        },
                    }
                }
            } else {
                Equation::Equality { left, right }
            };
            Equation::Equality {
                left: simplify_expression(equality.as_equality().unwrap().0),
                right: simplify_expression(equality.as_equality().unwrap().1),
            }
        }
    }
}

pub fn simplify(equation: &Equation) -> Equation {
    let mut result = equation.clone();
    loop {
        let next = simplify_equation(&result);
        if next == result {
            return result;
        }
        result = next;
    }
}
