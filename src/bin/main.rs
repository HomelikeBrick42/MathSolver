use math::*;
use num_rational::BigRational;

fn main() {
    let equation = Equation::Equality {
        left: Expression {
            terms: vec![Term {
                atoms: vec![Atom::Fraction {
                    numerator: Expression {
                        terms: vec![
                            Term {
                                atoms: vec![
                                    Atom::Number(BigRational::from_float(909.0).unwrap()),
                                    Atom::Variable("y".to_string()),
                                ],
                            },
                            Term {
                                atoms: vec![Atom::Number(BigRational::from_float(5.0).unwrap())],
                            },
                        ],
                    },
                    denominator: Expression {
                        terms: vec![Term {
                            atoms: vec![Atom::Number(BigRational::from_float(116.0).unwrap())],
                        }],
                    },
                }],
            }],
        },
        right: Expression {
            terms: vec![Term {
                atoms: vec![Atom::Number(BigRational::from_float(1246.0).unwrap())],
            }],
        },
    }; // should be 159
    println!("{}", equation);
    println!("{}", simplify(&equation));
}
