// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.
use std::cmp;

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Literal(isize),
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Over,
}

impl Expr {
    pub fn evaluate(&self) -> isize {
        match self {
            &Expr::Literal(ref value) => *value,
            &Expr::BinOp(ref expr1, BinOp::Plus ,ref expr2) => expr1.evaluate() + expr2.evaluate(),
            &Expr::BinOp(ref expr1, BinOp::Minus,ref expr2) => expr1.evaluate() - expr2.evaluate(),
            &Expr::BinOp(ref expr1, BinOp::Times,ref expr2) => expr1.evaluate() * expr2.evaluate(),
            &Expr::BinOp(ref expr1, BinOp::Over ,ref expr2) => expr1.evaluate() / expr2.evaluate(),
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match self {
            &Expr::Literal(_) => 0,
            &Expr::BinOp(ref expr1, _ , ref expr2) => 1 + expr1.operation_count() + expr2.operation_count(),
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self {
            &Expr::Literal(_) => 0,
            &Expr::BinOp(ref expr1, _ , ref expr2) =>  1 + cmp::max(expr1.depth(), expr2.depth())
        }
    }
}
