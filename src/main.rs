mod ast;
mod lexer;
mod parser;
mod tokens;
mod simplifier;
mod differentiator;

use std::{
    io::{self, Write},
};

use lexer::lex;
use parser::parse;

fn main() {
    let stdin = std::io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read stdin");
        input = input.trim().to_string();

        if input.to_lowercase() == "exit" {
            println!("bye");
            break;
        }

        let tokens = match lex(&input) {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("lex error {}", err);
                continue;
            }
        };
        println!("tokens read: {:?}", tokens);
        let mut ast = match parse(&tokens) {
            Ok(ast) => ast,
            Err(err) => {
                eprintln!("parse error {}", err);
                continue;
            }
        };

        println!("input read as: {ast}\n{ast:?}", ast=ast);
        match ast.simplify() {
            Err(err) => {
                eprintln!("evaluation error: {}", err);
                continue;
            }
            _ => (),
        }
        println!("input simplified to: {ast}\n{ast:?}", ast=ast);

        let mut derivative = ast.clone().differentiate();
        
        println!("derivative calculated: {derivative}\n{derivative:?}", derivative=derivative);
        match derivative.simplify() {
            Err(err) => {
                eprintln!("evaluation error: {}", err);
                continue;
            }
            _ => (),
        }
        println!("derivative simplified to: {derivative}\n{derivative:?}", derivative=derivative);
    }
}

