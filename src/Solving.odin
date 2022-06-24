package solver

import "core:reflect"

Atom_Equal :: proc(a, b: Atom) -> bool {
	if reflect.union_variant_typeid(a) != reflect.union_variant_typeid(b) do return false
	switch _ in a {
	case Variable:
		return true
	case Number:
		return a.(Number) == b.(Number)
	case Fraction:
		a_fraction := a.(Fraction)
		b_fraction := b.(Fraction)
		return Expression_Equal(
			a_fraction.numerator,
			b_fraction.numerator,
		) && Expression_Equal(a_fraction.denominator, b_fraction.denominator)
	case:
		unreachable()
	}
}

Term_Equal :: proc(a, b: Term) -> bool {
	if len(a) != len(b) do return false
	for i := 0; i < len(a); i += 1 {
		found := false
		for j := 0; j < len(b); j += 1 {
			if Atom_Equal(a[i], b[j]) {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

Expression_Equal :: proc(a, b: Expression) -> bool {
	if len(a) != len(b) do return false
	for i := 0; i < len(a); i += 1 {
		found := false
		for j := 0; j < len(b); j += 1 {
			if Term_Equal(a[i], b[j]) {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

Atom_Clone :: proc(atom: Atom) -> Atom {
	switch atom in atom {
	case Variable:
		return atom
	case Number:
		return atom
	case Fraction:
		return Fraction{
			numerator = Expression_Clone(atom.numerator),
			denominator = Expression_Clone(atom.denominator),
		}
	case:
		unreachable()
	}
}

Term_Clone :: proc(term: Term) -> Term {
	result: Term
	for atom in term {
		append(&result, Atom_Clone(atom))
	}
	return result
}

Expression_Clone :: proc(expression: Expression) -> Expression {
	result: Expression
	for term in expression {
		append(&result, Term_Clone(term))
	}
	return result
}

Term_HasVariable :: proc(term: Term, variable: Maybe(Variable) = nil) -> bool {
	for atom in term {
		switch atom in atom {
		case Variable:
			return true
		case Number:
			// do nothing
			break
		case Fraction:
			return Expression_HasVariable(
				atom.numerator,
			) || Expression_HasVariable(atom.denominator)
		case:
			unreachable()
		}
	}
	return false
}

Expression_HasVariable :: proc(
	expression: Expression,
	variable: Maybe(Variable) = nil,
) -> bool {
	for term in expression {
		if Term_HasVariable(term, variable) {
			return true
		}
	}
	return false
}

EvalAtom :: proc(atom: Atom) -> Number {
	switch atom in atom {
	case Variable:
		unreachable()
	case Number:
		return atom
	case Fraction:
		return EvalExpression(atom.numerator) / EvalExpression(atom.denominator)
	case:
		unreachable()
	}
}

EvalTerm :: proc(term: Term) -> Number {
	assert(!Term_HasVariable(term))
	if len(term) == 0 do return 0.0
	result := Number(1)
	for atom in term {
		result *= EvalAtom(atom)
	}
	return result
}

EvalExpression :: proc(expression: Expression) -> Number {
	assert(!Expression_HasVariable(expression))
	if len(expression) == 0 do return 0.0
	result := Number(0)
	for term in expression {
		result += EvalTerm(term)
	}
	return result
}

CompactTerm :: proc(term: ^Term) -> bool {
	changed := false
	not_variables: Term
	defer delete(not_variables)
	for i := 0; i < len(term); {
		switch atom in &term[i] {
		case Variable:
			i += 1
		case Number:
			append(&not_variables, atom)
			unordered_remove(term, i)
		case Fraction:
			changed |= CompactExpression(&atom.numerator)
			changed |= CompactExpression(&atom.denominator)
			if !Term_HasVariable(term^) {
				append(&not_variables, atom)
				unordered_remove(term, i)
			} else {
				i += 1
			}
		case:
			unreachable()
		}
	}
	if len(not_variables) > 0 {
		append(term, EvalTerm(not_variables))
		return changed || len(not_variables) > 1
	} else {
		return changed
	}
}

CompactExpression :: proc(expression: ^Expression) -> bool {
	changed := false
	for term in expression {
		changed |= CompactTerm(&term)
	}
	for i := 0; i < len(expression); i += 1 {
		for j := i + 1; j < len(expression); {
			IsLikeTerm :: proc(a, b: Term) -> bool {
				if !Term_HasVariable(a) && !Term_HasVariable(b) do return true
				if Term_Equal(a, b) do return true
				for i := 0; i < len(a); i += 1 {
					found := false
					if _, ok := a[i].(Number); ok do continue
					for j := 0; j < len(b); j += 1 {
						if _, ok := b[j].(Number); ok do continue
						if Atom_Equal(a[i], b[j]) {
							found = true
							break
						}
					}
					if !found {
						break
					}
				}
				return true
			}

			CombineLikeTerms :: proc(a, b: Term) -> Term {
				a_number := Number(1)
				for atom in a {
					if number, ok := atom.(Number); ok {
						a_number *= number
					}
				}
				b_number := Number(1)
				for atom in b {
					if number, ok := atom.(Number); ok {
						b_number *= number
					}
				}
				result: Term
				for term in a {
					if _, ok := term.(Number); !ok {
						append(&result, Atom_Clone(term))
					}
				}
				append(&result, a_number + b_number)
				return result
			}

			if IsLikeTerm(expression[i], expression[j]) {
				a := expression[i]
				b := expression[j]
				append(expression, CombineLikeTerms(a, b))
				ordered_remove(expression, i)
				unordered_remove(expression, j - 1)
				changed = true
			} else {
				j += 1
			}
		}
	}
	return changed
}

SimplifyEquality :: proc(equality: ^Equality) -> bool {
	did_something := false
	for {
		changed := false

		// Move terms that have no variable to their respective sides
		{
			collected_terms: [dynamic]Term
			defer delete(collected_terms)
			for i := 0; i < len(equality.left); {
				if !Term_HasVariable(equality.left[i]) {
					append(&collected_terms, equality.left[i])
					unordered_remove(&equality.left, i)
				} else {
					i += 1
				}
			}
			for collected_term in &collected_terms {
				append(&collected_term, -1)
				append(&equality.right, collected_term)
				changed = true
			}
		}
		{
			collected_terms: [dynamic]Term
			defer delete(collected_terms)
			for i := 0; i < len(equality.right); {
				if Term_HasVariable(equality.right[i]) {
					append(&collected_terms, equality.right[i])
					unordered_remove(&equality.right, i)
				} else {
					i += 1
				}
			}
			for collected_term in &collected_terms {
				append(&collected_term, -1)
				append(&equality.left, collected_term)
				changed = true
			}
		}

		changed |= CompactExpression(&equality.left)
		changed |= CompactExpression(&equality.right)

		for term in &equality.left {
			if len(term) > 1 {
				for i := 0; i < len(term); {
					if value, ok := term[i].(Number); ok && value == 1 {
						unordered_remove(&term, i)
						changed = true
					} else {
						i += 1
					}
				}
			}
		}
		if len(equality.right) > 1 {
			for i := 0; i < len(equality.right); {
				removed := false
				if len(equality.right[i]) == 1 {
					if value, ok := equality.right[i][0].(Number); ok && value == 0 {
						unordered_remove(&equality.right, i)
						removed = true
						changed = true
					}
				}
				if !removed {
					i += 1
				}
			}
		}

		if len(equality.left) == 1 {
			if len(equality.left[0]) > 1 {
				number_found := false
				value := Number(1)
				for i := 0; i < len(equality.left[0]); {
					if number, ok := equality.left[0][i].(Number); ok {
						number_found = true
						value *= number
						unordered_remove(&equality.left[0], i)
					} else {
						i += 1
					}
				}
				if number_found {
					for term in &equality.right {
						append(&term, 1.0 / value)
					}
					changed = true
				}
			}
		}

		if changed {
			did_something = true
		} else {
			break
		}
	}
	if len(equality.left) == 0 {
		append(&equality.left, Term{0})
	}
	if len(equality.right) == 0 {
		append(&equality.right, Term{0})
	}
	return did_something
}
