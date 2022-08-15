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
                    println!("{}", equation);
                    println!("{}", simplify(&equation));
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    }
}
