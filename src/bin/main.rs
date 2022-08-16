use std::io::Write;

use math::*;

fn main() {
    'main_loop: loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let line = std::io::stdin()
            .lines()
            .next()
            .unwrap()
            .expect("read a line from the console");
        if line.len() > 0 && line.chars().nth(0).unwrap() == ':' {
            match &line as &str {
                ":exit" => break 'main_loop,
                _ => println!("Unknown command '{line}'"),
            }
        } else {
            let mut lexer = Lexer::new("stdin", &line);
            match parse_equation(&mut lexer) {
                Ok(equation) => {
                    println!("{equation}");
                    let equation = simplify(&equation);
                    let (left, right) = equation.as_equality().unwrap();
                    if !left.contains_variable() && !right.contains_variable() {
                        println!("{}", eval_expression(left) == eval_expression(right));
                    } else {
                        println!("{equation}");
                    }
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    }
}
