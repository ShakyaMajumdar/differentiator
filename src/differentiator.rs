use crate::{ast::AST, tokens::BinOp};

impl AST {
    pub fn differentiate(&self) -> Self {
        return match self {
            AST::Literal { .. } | AST::SymbolicConstant {..} => AST::Literal { value: 0. },
            AST::Variable { .. } => AST::Literal { value: 1. },
            AST::BinOp { op, left, right } => get_binop_derivative(*op, left, right),
            AST::Function { name, args } => get_function_derivative(name, args),
        };
    }
}

fn get_function_derivative(name: &str, args: &Vec<AST>) -> AST {
    if args.len() != 1 {todo!();}
    let arg = args[0].clone();
    return arg.differentiate()
        * match name {
            "-" => AST::Literal{value: -1.},
            "sin" => AST::Function {
                name: "cos".to_string(),
                args: vec![arg],
            },
            "cos" => {
                AST::Literal { value: -1. }
                    * AST::Function {
                        name: "sin".to_string(),
                        args: vec![arg],
                    }
            }
            "tan" => AST::Function {
                name: "sec".to_string(),
                args: vec![arg],
            }
            .pow(AST::Literal { value: 2. }),
            "ln" => AST::Literal { value: 1. } / arg,
            _ => AST::Function { name: name.to_owned() + "'", args: vec![arg] },
        };
}

fn get_binop_derivative(op: BinOp, left: &AST, right: &AST) -> AST {
    return match op {
        BinOp::Add => left.differentiate() + right.differentiate(),
        BinOp::Sub => left.differentiate() - right.differentiate(),
        BinOp::Mul => left.clone() * right.differentiate() + right.clone() * left.differentiate(),
        BinOp::Div => {
            (right.clone() * left.differentiate() - left.clone() * right.differentiate())
                / right.clone().pow(AST::Literal { value: 2. })
        }
        BinOp::Pow => {
            left.clone().pow(right.clone())
                * (right.differentiate()
                    * AST::Function {
                        name: "ln".to_string(),
                        args: vec![left.clone()],
                    }
                    + right.clone() * left.differentiate() / left.clone())
        }
    };
}
