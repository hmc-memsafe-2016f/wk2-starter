// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::{cmp,mem};

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
            Expr::BinOp(ref left, BinOp::Plus, ref right) => left.evaluate() + right.evaluate(),
            Expr::BinOp(ref left, BinOp::Minus, ref right) => left.evaluate() - right.evaluate(),
            Expr::BinOp(ref left, BinOp::Times, ref right) => left.evaluate() * right.evaluate(),
            Expr::BinOp(ref left, BinOp::Over, ref right) => left.evaluate() / right.evaluate(),
            Expr::Literal(int) => int,
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match *self {
            Expr::BinOp(ref left, _, ref right) =>
                left.operation_count() + 1 + right.operation_count(),
            Expr::Literal(_) => 0,
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match *self {
            Expr::BinOp(ref left, _, ref right) => 1 + cmp::max(left.depth(), right.depth()),
            Expr::Literal(_) => 0,
        }
    }

    pub fn do_op(&mut self, operation: BinOp, operand: Expr) {
        let this = mem::replace(self, Expr::Literal(0));
        *self = Expr::BinOp(Box::new(this), operation, Box::new(operand));
    }

}

