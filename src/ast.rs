use std::{
    fmt::{self, Debug},
    ops::{Add, Div, Mul, Sub},
};

use crate::tokens::BinOp;

pub enum AST {
    Literal {
        value: f64,
    },
    SymbolicConstant {
        name: String,
    },
    Variable {
        name: String,
    },
    Function {
        name: String,
        args: Vec<AST>,
    },
    BinOp {
        op: BinOp,
        left: Box<AST>,
        right: Box<AST>,
    },
}

impl AST {
    fn tree_str(&self) -> String {
        match self {
            AST::Literal { value } => value.to_string(),
            AST::SymbolicConstant { name } => name.to_string(),
            AST::Variable { name } => name.to_string(),
            AST::BinOp { op, left, right } => {
                let mut s = format!("{:?}\n", op);
                for (i, line) in left.tree_str().lines().enumerate() {
                    s += &format!(
                        "{prefix}{line}\n",
                        prefix = &(if i == 0 { " ├─" } else { " │  " }),
                        line = line
                    );
                }
                for (i, line) in right.tree_str().lines().enumerate() {
                    s += &format!(
                        "{prefix}{line}\n",
                        prefix = &(if i == 0 { " ╰─" } else { "    " }),
                        line = line
                    );
                }
                s
            }
            AST::Function { name, args } => {
                let mut s = name.clone() + "\n";
                for (arg_no, arg) in args.iter().enumerate() {
                    for (line_no, line) in arg.tree_str().lines().enumerate() {
                        s += &format!(
                            "{prefix}{line}\n",
                            prefix = &(if line_no == 0 {
                                if arg_no == args.len() - 1 {
                                    " ╰─"
                                } else {
                                    " ├─"
                                }
                            } else {
                                "    "
                            }),
                            line = line
                        );
                    }
                }
                s
            }
        }
    }
    fn flat_str(&self) -> String {
        match self {
            AST::Literal { value } => value.to_string(),
            AST::SymbolicConstant { name } => name.to_string(),
            AST::Variable { name } => name.to_string(),
            AST::BinOp { op, left, right } => {
                let is_paren_required_for_left = is_paren_required(*op, &left);
                let is_paren_required_for_right = is_paren_required(*op, &right);
                format!(
                    "{}{}{} {} {}{}{}",
                    if is_paren_required_for_left { "(" } else { "" },
                    left.flat_str(),
                    if is_paren_required_for_left { ")" } else { "" },
                    op,
                    if is_paren_required_for_right { "(" } else { "" },
                    right.flat_str(),
                    if is_paren_required_for_right { ")" } else { "" }
                )
            }
            AST::Function { name, args } => format!(
                "{}({})",
                name.to_string(),
                args.iter().map(|arg| arg.flat_str()).collect::<Vec<String>>().join(", ")
            ),
        }
    }
    pub fn pow(self, rhs: Self) -> AST {
        return AST::BinOp {
            op: BinOp::Pow,
            left: Box::new(self),
            right: Box::new(rhs),
        };
    }
}
fn is_paren_required(root_op: BinOp, child_tree: &AST) -> bool {
    return match child_tree {
        AST::Literal { .. }
        | AST::SymbolicConstant { .. }
        | AST::Variable { .. }
        | AST::Function { .. } => false,
        AST::BinOp { op, .. } => root_op > *op,
    };
}
impl fmt::Display for AST {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.flat_str())
    }
}
impl Debug for AST {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.tree_str())
    }
}

impl Clone for AST {
    fn clone(&self) -> Self {
        match self {
            Self::Literal { value } => Self::Literal {
                value: value.clone(),
            },
            Self::SymbolicConstant { name } => Self::SymbolicConstant { name: name.clone() },
            Self::Variable { name } => Self::Variable { name: name.clone() },
            Self::Function { name, args } => Self::Function {
                name: name.clone(),
                args: args.clone()
            },
            Self::BinOp { op, left, right } => Self::BinOp {
                op: op.clone(),
                left: left.clone(),
                right: right.clone(),
            },
        }
    }
}
// impl Copy for AST {}

impl Add for AST {
    type Output = AST;

    fn add(self, rhs: Self) -> Self::Output {
        return AST::BinOp {
            op: BinOp::Add,
            left: Box::new(self),
            right: Box::new(rhs),
        };
    }
}
impl Sub for AST {
    type Output = AST;

    fn sub(self, rhs: Self) -> Self::Output {
        return AST::BinOp {
            op: BinOp::Sub,
            left: Box::new(self),
            right: Box::new(rhs),
        };
    }
}
impl Mul for AST {
    type Output = AST;

    fn mul(self, rhs: Self) -> Self::Output {
        return AST::BinOp {
            op: BinOp::Mul,
            left: Box::new(self),
            right: Box::new(rhs),
        };
    }
}
impl Div for AST {
    type Output = AST;

    fn div(self, rhs: Self) -> Self::Output {
        return AST::BinOp {
            op: BinOp::Div,
            left: Box::new(self),
            right: Box::new(rhs),
        };
    }
}
