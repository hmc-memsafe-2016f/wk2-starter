// Adam Dunlap <adunlap@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.
use std::cmp::max;

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
            &Expr::Literal(v) => v,
            &Expr::BinOp(ref e1, ref op, ref e2) => {
                let v1 = e1.evaluate();
                let v2 = e2.evaluate();
                match op {
                    &BinOp::Plus  => v1+v2,
                    &BinOp::Minus => v1-v2,
                    &BinOp::Times => v1*v2,
                    &BinOp::Over  => v1/v2,
                }
            }
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match self {
            &Expr::Literal(_) => 0,
            &Expr::BinOp(ref e1, _, ref e2) => {
                e1.operation_count() + e2.operation_count() + 1
            }
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self {
            &Expr::Literal(_) => 0,
            &Expr::BinOp(ref e1, _, ref e2) => {
                1+ max(e1.depth(), e2.depth())
            }
        }
    }
}

