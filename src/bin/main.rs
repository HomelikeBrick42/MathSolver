use math::*;

fn main() {
    let equation = Equation::Equality {
        left: Expression {
            terms: vec![
                Term {
                    atoms: vec![Atom::Number(-3.0), Atom::Variable("x".to_string())],
                },
                Term {
                    atoms: vec![Atom::Number(2.0), Atom::Variable("x".to_string())],
                },
                Term {
                    atoms: vec![Atom::Number(-3.0)],
                },
            ],
        },
        right: Expression {
            terms: vec![
                Term {
                    atoms: vec![Atom::Number(1.0)],
                },
                Term {
                    atoms: vec![Atom::Number(2.0)],
                },
            ],
        },
    };
    println!("{}", equation);
    println!("{}", simplify(&equation));
}
