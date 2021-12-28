use crate::ast::AST;
use crate::tokens::{BinOp, Token};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    UnclosedParen,
    MissingParen,
    UnexpectedOperator,
    UnexpectedTokens,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "died parsing lol");
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<AST, ParseError> {
    let index = &mut 0;
    let res = parse_term(tokens, index);
    if *index != tokens.len() {
        return Err(ParseError::UnexpectedTokens);
    }
    return res;
}

fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<AST, ParseError> {
    let mut left = parse_factor(tokens, index)?;
    loop {
        let op_token = match consume_token(tokens, index) {
            Ok(&Token::BinOp(token)) => match token {
                BinOp::Add | BinOp::Sub => token,
                _ => {
                    *index -= 1;
                    break;
                }
            },
            _ => {
                *index -= 1;
                break;
            }
        };
        let right = parse_factor(tokens, index)?;
        left = AST::BinOp {
            op: op_token,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    return Ok(left);
}

fn parse_factor(tokens: &Vec<Token>, index: &mut usize) -> Result<AST, ParseError> {
    let mut left = parse_pow(tokens, index)?;
    loop {
        let op_token = match consume_token(tokens, index) {
            Ok(&Token::BinOp(token)) => match token {
                BinOp::Mul | BinOp::Div => token,
                _ => {
                    *index -= 1;
                    break;
                }
            },
            _ => {
                *index -= 1;
                break;
            }
        };
        let right = parse_pow(tokens, index)?;
        left = AST::BinOp {
            op: op_token,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    return Ok(left);
}

fn parse_pow(tokens: &Vec<Token>, index: &mut usize) -> Result<AST, ParseError> {
    let mut pows: Vec<AST> = vec![parse_unary(tokens, index)?];
    loop {
        match consume_token(tokens, index) {
            Ok(&Token::BinOp(token)) => match token {
                BinOp::Pow => (),
                _ => {
                    *index -= 1;
                    break;
                }
            },
            _ => {
                *index -= 1;
                break;
            }
        };
        pows.push(parse_unary(tokens, index)?);
    }
    let mut right = pows.pop().unwrap();
    while !pows.is_empty() {
        let left = pows.pop().unwrap();
        right = AST::BinOp {
            op: BinOp::Pow,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(right)
}

fn parse_unary(tokens: &Vec<Token>, index: &mut usize) -> Result<AST, ParseError> {
    return match tokens.get(*index) {
        Some(&Token::BinOp(BinOp::Sub)) => {
            *index += 1;
            Ok(AST::Function {
                name: "-".to_string(),
                args: vec![parse_unary(tokens, index)?],
            })
        }
        _ => parse_primary(tokens, index),
    };
}

fn parse_primary(tokens: &Vec<Token>, index: &mut usize) -> Result<AST, ParseError> {
    return match consume_token(tokens, index)? {
        Token::Literal(value) => Ok(AST::Literal { value: *value }),
        Token::SymbolicConstant(name) => Ok(AST::SymbolicConstant { name: name.clone() }),
        Token::Variable(name) => Ok(AST::Variable { name: name.clone() }),
        Token::CloseParen => Err(ParseError::UnclosedParen),
        Token::Comma => Err(ParseError::UnexpectedTokens),
        Token::Function(name) => {
            match consume_token(tokens, index)? {
                Token::OpenParen => (),
                _ => return Err(ParseError::MissingParen),
            };
            let mut args = vec![parse_term(tokens, index)?];
            loop {
                match consume_token(tokens, index) {
                    Ok(Token::Comma) => (),
                    _ => {
                        *index -= 1;
                        break;
                    }
                }
                args.push(parse_term(tokens, index)?);
            }
            match consume_token(tokens, index)? {
                Token::CloseParen => (),
                _ => return Err(ParseError::MissingParen),
            };
            Ok(AST::Function {
                name: name.clone(),
                args,
            })
        }
        Token::OpenParen => {
            let arg = parse_term(tokens, index)?;
            match consume_token(tokens, index)? {
                Token::CloseParen => (),
                _ => return Err(ParseError::MissingParen),
            };
            Ok(arg)
        }
        Token::BinOp(_) => Err(ParseError::UnexpectedOperator),
    };
}

fn consume_token<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> Result<&'a Token, ParseError> {
    *index += 1;
    return tokens.get(*index - 1).ok_or(ParseError::EndOfStream);
}
