// Zach Hauser <zachary.hauser@pomona.edu>
// Submission for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
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

impl BinOp {
    fn get_fn(&self) -> Box<Fn(isize, isize) -> isize> {
        match self {
            &BinOp::Plus => Box::new(isize::add),
            &BinOp::Minus => Box::new(isize::sub),
            &BinOp::Times => Box::new(isize::mul),
            &BinOp::Over => Box::new(isize::div)
        }
    }
}

impl Expr {
    pub fn evaluate(&self) -> isize {
        match self {
            &Expr::BinOp(ref e1, ref op, ref e2) => op.get_fn()(e1.evaluate(), e2.evaluate()),
            &Expr::Literal(x) => x
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match self {
            &Expr::BinOp(ref e1, _, ref e2) => 1 + e1.operation_count() + e2.operation_count(),
            &Expr::Literal(_) => 0
        }    
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self {
            &Expr::BinOp(ref e1, _, ref e2) => 1 + cmp::max(e1.depth(), e2.depth()),
            &Expr::Literal(_) => 0
        }  
    }
}

