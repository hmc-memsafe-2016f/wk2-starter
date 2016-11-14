// Jackson Warley
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::cmp::max;

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

impl Expr {
    pub fn evaluate(&self) -> isize {
        match *self {
            Expr::Literal(val) => return val,
            Expr::BinOp(ref left, ref op, ref right) => match *op {
                BinOp::Plus => return left.evaluate() + right.evaluate(),
                BinOp::Minus => return left.evaluate() - right.evaluate(),
                BinOp::Times => return left.evaluate() * right.evaluate(),
                BinOp::Over => return left.evaluate() / right.evaluate(),
            }
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match *self {
            Expr::Literal(_) => return 0,
            Expr::BinOp(ref left, _, ref right) =>
                                   return 1 + left.operation_count() + right.operation_count(),
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match *self {
            Expr::Literal(_) => return 0,
            Expr::BinOp(ref left, _, ref right) => return 1 + max(left.depth(), right.depth()),
        }
    }
}