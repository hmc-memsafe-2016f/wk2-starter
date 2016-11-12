// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

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
            Expr::BinOp(ref x, BinOp::Plus,  ref y) => x.evaluate() + y.evaluate(),
            Expr::BinOp(ref x, BinOp::Minus, ref y) => x.evaluate() - y.evaluate(),
            Expr::BinOp(ref x, BinOp::Times, ref y) => x.evaluate() * y.evaluate(),
            Expr::BinOp(ref x, BinOp::Over,  ref y) => x.evaluate() / y.evaluate(),
            Expr::Literal(num) => num
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match *self {
            Expr::Literal(_) => 0,
            Expr::BinOp(ref x, _, ref y) => 1 + x.operation_count() + y.operation_count()
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match *self {
            Expr::Literal(_) => 0,
            Expr::BinOp(ref x, _, ref y) => 1 + 
        }
    }
}

