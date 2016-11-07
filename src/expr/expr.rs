// Eric Mueller
// Base on starter code for HMC's MemorySafe, week 2
//
// The definition of `Expr`, a type that represents arithmetic expressions involving +,-,*,/, in
// terms of those operations.

use std::fmt;
use std::str::FromStr;
use std::num;
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::BinOp(ref lhs, ref op, ref rhs) => {
                write!(f, "(")
                    .and_then(|()| 
                              lhs.fmt(f)
                    .and_then(|()| { match op {
                        &BinOp::Plus => write!(f, " + "),
                        &BinOp::Minus => write!(f, " - "),
                        &BinOp::Times => write!(f, " * "),
                        &BinOp::Over => write!(f, " / "),}}
                    .and_then(|()|
                              rhs.fmt(f)
                    .and_then(|()|
                              write!(f, ")")))))
            },
            Expr::Literal(l) => l.fmt(f)
        }
    }
}

impl FromStr for Expr {
    type Err = num::ParseIntError;

    fn from_str(src: &str) -> Result<Expr, num::ParseIntError> {
        isize::from_str(src).map(|i| Expr::Literal(i))
    }
}

impl Expr {
    pub fn evaluate(&self) -> isize {
        match *self {
            Expr::BinOp(ref lhs, ref op, ref rhs) => {
                let lval = lhs.evaluate();
                let rval = rhs.evaluate();
                match op {
                    &BinOp::Plus => lval + rval,
                    &BinOp::Minus => lval - rval,
                    &BinOp::Times => lval * rval,
                    &BinOp::Over => lval / rval,
                }
            },
            Expr::Literal(l) => l
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4-5` has two operations.
    pub fn operation_count(&self) -> usize {
        match *self {
            Expr::BinOp(ref lhs, _, ref rhs) => {
                lhs.operation_count() + rhs.operation_count() + 1
            },
            Expr::Literal(_) => 0
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+4*3` has depth 2
    pub fn depth(&self) -> usize {
        match *self {
            Expr::BinOp(ref lhs, _, ref rhs) => {
                1 + cmp::max(lhs.depth(), rhs.depth())
            },
            Expr::Literal(_) => 0
        }
    }
}

