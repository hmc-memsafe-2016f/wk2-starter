// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Literal(i64),
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Over,
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        unimplemented!()
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        unimplemented!()
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        unimplemented!()
    }
}

