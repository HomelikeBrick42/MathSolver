package solver

Atom :: union {
	Variable,
	Number,
	Fraction,
}
Variable :: distinct string
Number :: distinct f64
Fraction :: struct {
	numerator:   Expression,
	denominator: Expression,
}
Term :: distinct [dynamic]Atom
Expression :: distinct [dynamic]Term

Equality :: struct {
	left:  Expression,
	right: Expression,
}
