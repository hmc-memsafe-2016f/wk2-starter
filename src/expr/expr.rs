// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::mem;
use std::cmp;

pub enum Expr {
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Literal(i64),
}

pub enum BinOp {
    Plus,
    Minus,
    Times,
    Over,
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        let theVal = match self {
            &Expr::Literal(i) => i,
            &Expr::BinOp(ref a, ref b, ref c) => 
                match b {
                    &BinOp::Plus => a.evaluate() + c.evaluate(),
                    &BinOp::Minus => a.evaluate() - c.evaluate(),
                    &BinOp::Times => a.evaluate() * c.evaluate(),
                    &BinOp::Over => a.evaluate() / c.evaluate()
                }
        };
        //print!("{:?}", theVal);
        theVal
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match self { //??
            &Expr::Literal(i) => 0,
            &Expr::BinOp(ref a, ref b, ref c) => 1 + a.operation_count() + c.operation_count()
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self { //??
            &Expr::Literal(i) => 0,
            &Expr::BinOp(ref a, ref b, ref c) => cmp::max(1 + a.operation_count(), 1 + c.operation_count())
        }
    }
}

