use std::{cmp::Ordering, fmt::Display};

use derive_more::IsVariant;
use enum_as_inner::EnumAsInner;
use num_rational::BigRational;

#[derive(Clone, PartialEq, Debug, IsVariant, EnumAsInner)]
pub enum Atom {
    Number(BigRational),
    Variable(String),
    Group(Expression),
    Fraction {
        numerator: Expression,
        denominator: Expression,
    },
}

impl Atom {
    pub fn contains_variable(&self) -> bool {
        match self {
            Atom::Number(_) => false,
            Atom::Variable(_) => true,
            Atom::Group(expression) => expression.contains_variable(),
            Atom::Fraction {
                numerator,
                denominator,
            } => numerator.contains_variable() || denominator.contains_variable(),
        }
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        match self {
            Atom::Number(value) => match other {
                Atom::Number(other_value) => value.partial_cmp(other_value),
                Atom::Variable(_) => Some(Ordering::Less),
                Atom::Group(_) => Some(Ordering::Less),
                Atom::Fraction {
                    numerator: _,
                    denominator: _,
                } => Some(Ordering::Less),
            },
            Atom::Variable(name) => match other {
                Atom::Number(_) => Some(Ordering::Greater),
                Atom::Variable(other_name) => name.partial_cmp(other_name),
                Atom::Group(_) => Some(Ordering::Less),
                Atom::Fraction {
                    numerator: _,
                    denominator: _,
                } => Some(Ordering::Greater),
            },
            Atom::Group(expression) => match other {
                Atom::Number(_) => Some(Ordering::Greater),
                Atom::Variable(_) => Some(Ordering::Greater),
                Atom::Group(other_expression) => expression.partial_cmp(other_expression),
                Atom::Fraction {
                    numerator: _,
                    denominator: _,
                } => Some(Ordering::Greater),
            },
            Atom::Fraction {
                numerator,
                denominator,
            } => match other {
                Atom::Number(_) => Some(Ordering::Greater),
                Atom::Variable(_) => Some(Ordering::Less),
                Atom::Group(_) => Some(Ordering::Less),
                Atom::Fraction {
                    numerator: other_numerator,
                    denominator: other_denominator,
                } => [numerator, denominator].partial_cmp(&[other_numerator, other_denominator]),
            },
        }
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Atom::Number(value) => write!(f, "{}", value),
            Atom::Variable(name) => write!(f, "{}", name),
            Atom::Group(expression) => write!(f, "({})", expression),
            Atom::Fraction {
                numerator,
                denominator,
            } => write!(f, "(({})/({}))", numerator, denominator),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Term {
    pub atoms: Vec<Atom>,
}

impl Term {
    pub fn contains_variable(&self) -> bool {
        self.atoms.iter().any(Atom::contains_variable)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        self.atoms.partial_cmp(&other.atoms)
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.atoms.len() == 0 {
            return write!(f, "1");
        }
        for (i, atom) in self.atoms.iter().enumerate() {
            if i > 0 {
                write!(f, "*")?;
            }
            write!(f, "{}", atom)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    pub terms: Vec<Term>,
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Expression) -> Option<Ordering> {
        self.terms.partial_cmp(&other.terms)
    }
}

impl Expression {
    pub fn contains_variable(&self) -> bool {
        self.terms.iter().any(Term::contains_variable)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.terms.len() == 0 {
            return write!(f, "0");
        }
        for (i, term) in self.terms.iter().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{}", term)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Equation {
    Equality { left: Expression, right: Expression },
}

impl Equation {
    pub fn contains_variable(&self) -> bool {
        match self {
            Equation::Equality { left, right } => {
                left.contains_variable() || right.contains_variable()
            }
        }
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Equation::Equality { left, right } => write!(f, "{} = {}", left, right),
        }
    }
}
