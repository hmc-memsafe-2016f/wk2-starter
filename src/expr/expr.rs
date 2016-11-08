// Luis Viornery <lviornery@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

//allows us to use max
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
            //references makes us not-sad here - note that &self is a reference.
            &Expr::BinOp(ref left_box, BinOp::Plus, ref right_box) => (*left_box).evaluate() + (*right_box).evaluate(),
            &Expr::BinOp(ref left_box, BinOp::Minus, ref right_box) => (*left_box).evaluate() - (*right_box).evaluate(),
            &Expr::BinOp(ref left_box, BinOp::Times, ref right_box) => (*left_box).evaluate() * (*right_box).evaluate(),
            &Expr::BinOp(ref left_box, BinOp::Over, ref right_box) => (*left_box).evaluate() / (*&right_box).evaluate(),
            &Expr::Literal(number) => number,
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        //add one for each binop symbol
        match self {
            &Expr::BinOp(ref left_box, _, ref right_box) => (*left_box).operation_count() + (*right_box).operation_count() + 1,
            &Expr::Literal(_) => 0,
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match self {
            //add 1 to the max depth between the two sub-branches
            &Expr::BinOp(ref left_box, _, ref right_box) => max((*left_box).depth(), (*right_box).depth()) + 1,
            &Expr::Literal(_) => 0,
        }
    }
}

