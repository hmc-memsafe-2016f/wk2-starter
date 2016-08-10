// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Sample solution for HMC's MemorySafe, week 2
//
// The definition of `RExpr` (ReducedExpr), a type that represents arithmetic expressions involving
// +,-,*,/, in terms of Addition, Multiplication, Negation, and Division

#[macro_use]
extern crate nom;

mod expr;

use nom::IResult;
use std::{mem,cmp,io};
use expr::Expr;

pub enum RExpr {
    Negation(Box<RExpr>),
    BinOp(Box<RExpr>, BinOp, Box<RExpr>),
    Literal(isize),
}

#[derive(PartialEq,Eq)]
pub enum BinOp {
    Plus,
    Times,
    Over,
}

impl RExpr {
    pub fn evaluate(&self) -> isize {
        match *self {
            RExpr::Negation(ref expr) => -expr.evaluate(),
            RExpr::BinOp(ref left, BinOp::Plus, ref right) => left.evaluate() + right.evaluate(),
            RExpr::BinOp(ref left, BinOp::Times, ref right) => left.evaluate() * right.evaluate(),
            RExpr::BinOp(ref left, BinOp::Over, ref right) => left.evaluate() / right.evaluate(),
            RExpr::Literal(val) => val,
        }
    }

    /// Computes the number of binary operations.
    /// For example, `1+4+(-(5))` has three operations.
    pub fn operation_count(&self) -> usize {
        match *self {
            RExpr::Negation(ref expr) => 1 + expr.operation_count(),
            RExpr::BinOp(ref left, _, ref right) =>
                left.operation_count() + 1 + right.operation_count(),
            RExpr::Literal(_) => 0,
        }
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+(-(3))` has depth 2
    pub fn depth(&self) -> usize {
        match *self {
            RExpr::Negation(ref expr) => 1 + expr.operation_count(),
            RExpr::BinOp(ref left, _, ref right) =>
                1 + cmp::max(left.operation_count(), right.operation_count()),
            RExpr::Literal(_) => 0,
        }
    }

    /// Pulls the left child of this node up, setting `*self` to it.
    /// Will not change the value of the expression.
    ///
    /// ## Returns
    /// Returns whether the rotation was possible.
    pub fn rotate_right(&mut self) -> bool {
        use self::RExpr::*;
        use self::BinOp::*;
        let (this, rotated) = match mem::replace(self, Literal(0)) {
            l@Literal(_) => (l, false),
            n@Negation(_) => (n, false),
            d@BinOp(_, Over, _) => (d, false),
            BinOp(left, op, right) => {
                let unboxed = *left;
                match unboxed {
                    BinOp(l_left, l_op, l_right) => {
                        if l_op == op {
                            let new_right = BinOp(l_right, op, right);
                            let new_this = BinOp(l_left, l_op, Box::new(new_right));
                            (new_this, true)
                        } else {
                            let old_left = BinOp(l_left, l_op, l_right);
                            (BinOp(Box::new(old_left), op, right), false)
                        }
                    }
                    old_left => (BinOp(Box::new(old_left), op, right), false)
                }
            }
        };
        *self = this;
        rotated
    }

    /// Pulls the right child of this node up, setting `*self` to it.
    /// Will not change the value of the expression.
    ///
    /// ## Returns
    /// Returns whether the rotation was possible.
    pub fn rotate_left(&mut self) -> bool {
        use self::RExpr::*;
        use self::BinOp::*;
        let (this, rotated) = match mem::replace(self, Literal(0)) {
            l@Literal(_) => (l, false),
            n@Negation(_) => (n, false),
            d@BinOp(_, Over, _) => (d, false),
            BinOp(left, op, right) => {
                let unboxed = *right;
                match unboxed {
                    BinOp(r_left, r_op, r_right) => {
                        if r_op == op {
                            let new_left = BinOp(left, op, r_left);
                            let new_this = BinOp(Box::new(new_left), r_op, r_right);
                            (new_this, true)
                        } else {
                            let old_right = BinOp(r_left, r_op, r_right);
                            (BinOp(left, op, Box::new(old_right)), false)
                        }
                    }
                    old_right => (BinOp(left, op, Box::new(old_right)), false)
                }
            }
        };
        *self = this;
        rotated
    }

    pub fn minimize_depth(&mut self) {
        unimplemented!()
    }
}

impl From<Expr> for RExpr {
    fn from(expr: Expr) -> RExpr {
        match expr {
            Expr::Literal(val) => RExpr::Literal(val),
            Expr::BinOp(left, expr::BinOp::Plus, right) =>
                RExpr::BinOp(Box::new(Self::from(*left)), BinOp::Plus, Box::new(Self::from(*right))),
            Expr::BinOp(left, expr::BinOp::Minus, right) => {
                let neg = RExpr::Negation(Box::new(Self::from(*right)));
                RExpr::BinOp(Box::new(Self::from(*left)), BinOp::Plus, Box::new(neg))
            },
            Expr::BinOp(left, expr::BinOp::Times, right) =>
                RExpr::BinOp(Box::new(Self::from(*left)), BinOp::Times, Box::new(Self::from(*right))),
            Expr::BinOp(left, expr::BinOp::Over, right) =>
                RExpr::BinOp(Box::new(Self::from(*left)), BinOp::Over, Box::new(Self::from(*right))),
        }
    }
}

fn main() {
    let mut line = String::new();

    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {

        match expr::parse(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, res) => {
                if rest.len() == 0 {
                    let reduced = RExpr::from(res);
                    println!("{} {} {}", reduced.evaluate(),
                                         reduced.depth(),
                                         reduced.operation_count());
                } else {
                    println!("Error")
                }
            },
            _ => println!("Error"),
        }

        line.clear();
    }
}
