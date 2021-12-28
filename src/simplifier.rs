use std::{error::Error, fmt};

use crate::{ast::AST, tokens::BinOp};

#[derive(Debug)]
pub enum EvaluationError {
    ZeroDivisionError,
    WrongArguments
}
impl Error for EvaluationError {}

impl fmt::Display for EvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "rip");
    }
}

impl AST {
    pub fn simplify(&mut self) -> Result<bool, EvaluationError> {
        let mut simplified = false;
        loop {
            let changed = self.simplify_once()?;
            if changed {
                simplified = true;
                continue;
            }
            break;
        }
        return Ok(simplified);
    }
    fn simplify_once(&mut self) -> Result<bool, EvaluationError> {
        match self {
            AST::Literal { .. } | AST::SymbolicConstant { .. } | AST::Variable { .. } => {
                return Ok(false)
            }
            AST::Function { name, args } => {
                let mut simplified_args = false;
                for arg in args.iter_mut() {
                    simplified_args = arg.simplify()? | simplified_args;
                }
                match get_standard_value(name, args)? {
                    Some(value) => *self = value,
                    _ => (),
                }
                return Ok(simplified_args);
            }
            AST::BinOp { op, left, right } => {
                left.simplify()?;
                right.simplify()?;
                match (*left.clone(), *right.clone()) {
                    (AST::Literal { value: val1 }, AST::Literal { value: val2 }) => {
                        *self = AST::Literal {
                            value: get_func_from_op(*op)(val1, val2),
                        };
                        return Ok(true);
                    }
                    _ => (),
                }
                match op {
                    BinOp::Add | BinOp::Sub => {
                        match **left {
                            AST::Literal { value } => {
                                if value == 0. {
                                    *self = *right.clone();
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        match **right {
                            AST::Literal { value } => {
                                if value == 0. {
                                    *self = *left.clone();
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        return Ok(false);
                    }
                    BinOp::Mul => {
                        match **left {
                            AST::Literal { value } => {
                                if value == 1. {
                                    *self = *right.clone();
                                    return Ok(true);
                                } else if value == 0. {
                                    *self = AST::Literal { value: 0. };
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        match **right {
                            AST::Literal { value } => {
                                if value == 1. {
                                    *self = *left.clone();
                                    return Ok(true);
                                } else if value == 0. {
                                    *self = AST::Literal { value: 0. };
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        return Ok(false);
                    }
                    BinOp::Div => {
                        match **left {
                            AST::Literal { value } => {
                                if value == 0. {
                                    *self = AST::Literal { value: 0. };
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        match **right {
                            AST::Literal { value } => {
                                if value == 1. {
                                    *self = *left.clone();
                                    return Ok(true);
                                } else if value == 0. {
                                    return Err(EvaluationError::ZeroDivisionError);
                                }
                            }
                            _ => (),
                        }
                        return Ok(false);
                    }
                    BinOp::Pow => {
                        match **left {
                            AST::Literal { value } => {
                                if value == 1. {
                                    *self = AST::Literal { value: 1. };
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        match **right {
                            AST::Literal { value } => {
                                if value == 0. {
                                    *self = AST::Literal { value: 1. };
                                    return Ok(true);
                                } else if value == 1. {
                                    *self = *left.clone();
                                    return Ok(true);
                                }
                            }
                            _ => (),
                        }
                        return Ok(false);
                    }
                }
            }
        }
    }
}
fn get_func_from_op(op: BinOp) -> fn(f64, f64) -> f64 {
    return match op {
        BinOp::Add => |x, y| x + y,
        BinOp::Sub => |x, y| x - y,
        BinOp::Mul => |x, y| x * y,
        BinOp::Div => |x, y| x / y,
        BinOp::Pow => |x, y| x.powf(y),
    };
}

fn get_standard_value(name: &str, args: &Vec<AST>) -> Result<Option<AST>, EvaluationError> {
    // match value[..] {
    //     AST::Literal { .. } | AST::SymbolicConstant { .. } => (),
    //     _ => return None,
    // // }
    // if !args
    //     .iter()
    //     .all(|arg| matches!(arg, AST::Literal { .. } | AST::SymbolicConstant { .. }))
    // {
    //     return None;
    // }
    match name {
        "sin" => match &args[..] {
            [AST::Literal { value }] if *value == 0. => return Ok(Some(AST::Literal { value: 0. })),
            [AST::SymbolicConstant { name }] if name == "pi" => {
                return Ok(Some(AST::Literal { value: 0. }))
            }
            [_] => return Ok(None),
            _ => return Err(EvaluationError::WrongArguments)
        },
        "ln" => match &args[..] {
            [AST::Literal { value }] if *value == 0. => return Ok(Some(AST::Literal { value: 0. })),
            [AST::SymbolicConstant { name }] if name == "e" => {
                return Ok(Some(AST::Literal { value: 1. }))
            }
            [_] => return Ok(None),
            _ => return Err(EvaluationError::WrongArguments)
        },
        _ => return Ok(None),
    };
}
