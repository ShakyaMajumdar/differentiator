use std::{error::Error, fmt};

use crate::tokens::{BinOp, Token};

#[derive(Debug)]
pub struct LexError;

impl Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "rip");
    }
}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0;
    loop {
        if let Some(current) = src.chars().nth(index) {
            tokens.push(match current {
                '(' => {
                    index += 1;
                    Token::OpenParen
                }
                ')' => {
                    index += 1;
                    Token::CloseParen
                }
                '-' => {
                    index += 1;
                    Token::BinOp(BinOp::Sub)
                }
                '+' => {
                    index += 1;
                    Token::BinOp(BinOp::Add)
                }
                '*' => {
                    index += 1;
                    Token::BinOp(BinOp::Mul)
                }
                '/' => {
                    index += 1;
                    Token::BinOp(BinOp::Div)
                }
                '^' => {
                    index += 1;
                    Token::BinOp(BinOp::Pow)
                }
                ',' => {
                    index += 1;
                    Token::Comma
                }
                ' ' => {
                    index += 1;
                    continue;
                }
                '0'..='9' => get_number_token_at(src, &mut index)?,
                'a'..='z' | 'A'..='Z' => get_const_variable_or_function_token_at(src, &mut index)?,
                _ => return Err(LexError),
            })
        } else {
            break;
        }
    }
    return Ok(tokens);
}

fn get_const_variable_or_function_token_at(
    source: &str,
    index: &mut usize,
) -> Result<Token, LexError> {
    let start = *index;
    for current in source.get(start..).unwrap().chars() {
        match current {
            'a'..='z' | 'A'..='Z' => {
                *index += 1;
            }
            _ => break,
        }
    }
    let identifier = source.get(start..*index).unwrap().to_string();
    return match source.chars().nth(*index) {
        Some('(') => Ok(Token::Function(identifier)),
        _ => match identifier.as_str() {
            "e" | "pi" => Ok(Token::SymbolicConstant(identifier)),
            _ => Ok(Token::Variable(identifier)),
        },
    };
}

fn get_number_token_at(source: &str, index: &mut usize) -> Result<Token, LexError> {
    let mut seen_dot = false;
    let start = *index;
    for current in source.get(start..).unwrap().chars() {
        match current {
            '0'..='9' => {}
            '.' => {
                if seen_dot {
                    return Err(LexError);
                }
                seen_dot = true;
            }
            _ => break,
        }
        *index += 1;
    }
    return Ok(Token::Literal(
        str::parse::<f64>(source.get(start..*index).unwrap()).unwrap(),
    ));
}
