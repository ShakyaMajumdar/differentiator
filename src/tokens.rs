use std::fmt::{Debug, Display};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
// #[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Sub,
    Add,
    Mul,
    Div,
    Pow,
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sub => write!(f, "-"),
            Self::Add => write!(f, "+"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Pow => write!(f, "^"),
        }
    }
}


#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Literal(f64),
    SymbolicConstant(String),
    Variable(String),
    Function(String),
    BinOp(BinOp),
    Comma,
}
