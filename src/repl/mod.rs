use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::evaluator::Evaluator;
use std::io::{stdin, stdout, Write};
use crate::evaluator::types::Type;

pub fn run() {
    let caret = String::from("> ");

    let mut parser = Parser::new();
    let mut evaluator = Evaluator::new();
    loop {
        print!("{}", caret);
        if let Err(err) = stdout().flush() {
            println!("{}", err);
            break;
        }

        let mut input = String::new();
        if let Err(err) = stdin().read_line(&mut input) {
            println!("{}", err);
            break;
        }
        if input.trim() == "exit" {
            break;
        }

        let tokens = tokenize(&input);
        let expr = parser.parse(tokens);
        match expr {
            Err(err) => println!("{}{}", caret, err),
            Ok(expr) => {
                let result = evaluator.evaluate(expr);
                println!("{}{}", caret, result);
            }
        }
    }
}