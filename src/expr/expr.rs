// Julien Chien <jchien17@cmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::cmp;

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
        match self {
            &Expr::Literal(e) => e,
            &Expr::BinOp(ref left, ref op, ref right) => {
                match op {
                    BinOp::Plus => Expr::evaluate(left) + Expr::evaluate(right),
                    BinOp::Minus => Expr::evaluate(left) - Expr::evaluate(right),
                    BinOp::Times => Expr::evaluate(left) * Expr::evaluate(right),
                    BinOp::Over => Expr::evaluate(left) / Expr::evaluate(right),
                }
            }
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match self {
            &Expr::Literal(_) => 0,
            &Expr::BinOp(ref left, _, ref right) => {
                left.operation_count() + right.operation_count() + 1
            }
       }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self {
            &Expr::Literal(_) => 0,
            &Expr::BinOp(ref left, _, ref right) => {
                cmp::max(left.depth(), right.depth())
            }
        }
    }
}

