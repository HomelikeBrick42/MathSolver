package solver

import "core:io"
import "core:fmt"
import "core:strings"

main :: proc() {
	SetupFormatters()
	equality := Equality {
		left = Expression{{2, "x"}, {-3}},
		right = {{5}},
	}
	fmt.println("Original:", equality)
	SimplifyEquality(&equality)
	fmt.println("Output:  ", equality)
	return
}

SetupFormatters :: proc() {
	@(static)
	formatters: map[typeid]fmt.User_Formatter
	fmt.set_user_formatters(&formatters)

	WriteVariable :: proc(fi: ^fmt.Info, arg: any, verb: rune) -> bool {
		io.write_string(fi.writer, cast(string)arg.(Variable))
		return true
	}
	fmt.register_user_formatter(Variable, WriteVariable)

	WriteNumber :: proc(fi: ^fmt.Info, arg: any, verb: rune) -> bool {
		str := fmt.tprintf("%.16f", cast(f64)arg.(Number))
		str = strings.trim_right(str, "0")
		if str[len(str) - 1] == '.' {
			str = str[:len(str) - 1]
		}
		io.write_string(fi.writer, str)
		return true
	}
	fmt.register_user_formatter(Number, WriteNumber)

	WriteFraction :: proc(fi: ^fmt.Info, arg: any, verb: rune) -> bool {
		fraction := arg.(Fraction)
		io.write_string(
			fi.writer,
			fmt.tprintf("({})/({})", fraction.numerator, fraction.denominator),
		)
		return true
	}
	fmt.register_user_formatter(Fraction, WriteFraction)

	WriteTerm :: proc(fi: ^fmt.Info, arg: any, verb: rune) -> bool {
		term := arg.(Term)
		for atom, i in term {
			if i > 0 do io.write_string(fi.writer, "*")
			io.write_string(fi.writer, fmt.tprintf("{}", atom))
		}
		return true
	}
	fmt.register_user_formatter(Term, WriteTerm)

	WriteExpression :: proc(fi: ^fmt.Info, arg: any, verb: rune) -> bool {
		expression := arg.(Expression)
		for term, i in expression {
			if i > 0 do io.write_string(fi.writer, " + ")
			io.write_string(fi.writer, fmt.tprintf("{}", term))
		}
		return true
	}
	fmt.register_user_formatter(Expression, WriteExpression)

	WriteEquality :: proc(fi: ^fmt.Info, arg: any, verb: rune) -> bool {
		equality := arg.(Equality)
		io.write_string(fi.writer, fmt.tprintf("{}", equality.left))
		io.write_string(fi.writer, " = ")
		io.write_string(fi.writer, fmt.tprintf("{}", equality.right))
		return true
	}
	fmt.register_user_formatter(Equality, WriteEquality)
}
