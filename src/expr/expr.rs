// Dan Obermiller <dobermiller16@cmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::cmp;
use std::fmt;

pub enum Expr {
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Literal(isize),
}

pub enum BinOp {
    Plus,
    Minus,
    Times,
    Over,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BinOp::Plus => write!(f, "+"),
            &BinOp::Minus => write!(f, "-"),
            &BinOp::Times => write!(f, "*"),
            &BinOp::Over => write!(f, "/"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expr::BinOp(ref left, ref operation, ref right) => write!(f, "{} {} {}", left, operation, right),
            &Expr::Literal(value) => write!(f, "{}", value),
        }
    }
}

impl Expr {
    pub fn evaluate(&self) -> isize {
        match self {
            &Expr::BinOp(ref left, ref operation, ref right) => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                match operation {
                    &BinOp::Plus => left_val + right_val,
                    &BinOp::Minus => left_val - right_val,
                    &BinOp::Times => left_val * right_val,
                    &BinOp::Over => left_val / right_val
                }
            },
            &Expr::Literal(value) => value
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match self {
            &Expr::BinOp(ref left, _, ref right) => 1 + left.operation_count() + right.operation_count(),
            &Expr::Literal(_) => 0
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self {
            &Expr::BinOp(ref left, _, ref right) => 1 + cmp::max(left.depth(), right.depth()),
            &Expr::Literal(_) => 0
        }
    }
}
