use math::*;

fn main() {
    // let equation = Equation::Equality {
    //     left: Expression {
    //         terms: vec![
    //             Term {
    //                 atoms: vec![Atom::Number(-3.0), Atom::Variable("x".to_string())],
    //             },
    //             Term {
    //                 atoms: vec![Atom::Number(2.0), Atom::Variable("x".to_string())],
    //             },
    //             Term {
    //                 atoms: vec![Atom::Number(-3.0)],
    //             },
    //         ],
    //     },
    //     right: Expression {
    //         terms: vec![
    //             Term {
    //                 atoms: vec![Atom::Number(1.0)],
    //             },
    //             Term {
    //                 atoms: vec![Atom::Number(2.0)],
    //             },
    //         ],
    //     },
    // };
    let equation = Equation::Equality {
        left: Expression {
            terms: vec![Term {
                atoms: vec![Atom::Fraction {
                    numerator: Expression {
                        terms: vec![
                            Term {
                                atoms: vec![Atom::Number(909.0), Atom::Variable("y".to_string())],
                            },
                            Term {
                                atoms: vec![Atom::Number(5.0)],
                            },
                        ],
                    },
                    denominator: Expression {
                        terms: vec![Term {
                            atoms: vec![Atom::Number(116.0)],
                        }],
                    },
                }],
            }],
        },
        right: Expression {
            terms: vec![Term {
                atoms: vec![Atom::Number(1246.0)],
            }],
        },
    }; // should be 159
    println!("{}", equation);
    println!("{}", simplify(&equation));
}
