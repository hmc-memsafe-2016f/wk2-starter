// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The definition of `RExpr` (ReducedExpr), a type that represents arithmetic expressions involving
// +,-,*,/, in terms of Addition, Multiplication, Negation, and Reciprocal

pub enum RExpr {
    Negation(Box<RExpr>),
    BinOp(Box<RExpr>, BinOp, Box<RExpr>),
    Literal(isize),
}

pub enum BinOp {
    Plus,
    Times,
    Over,
}

impl RExpr {
    pub fn evaluate(&self) -> isize {
        unimplemented!()
    }

    /// Computes the number of binary operations.
    /// For example, `1+4+(-(5))` has three operations.
    pub fn operation_count(&self) -> usize {
        unimplemented!()
    }

    /// The depth, defined as `max{ # of operations from root to leaf }`.
    /// `1` has depth 0, `1+3` has depth 1, and `1+(-(3))` has depth 2
    pub fn depth(&self) -> usize {
        unimplemented!()
    }

    /// Pulls the left child of this node up, setting `*self` to it.
    /// Will not change the value of the expression.
    ///
    /// ## Returns
    /// Returns whether the rotation was possible.
    pub fn rotate_right(&mut self) -> bool {
        unimplemented!()
    }

    /// Pulls the right child of this node up, setting `*self` to it.
    /// Will not change the value of the expression.
    ///
    /// ## Returns
    /// Returns whether the rotation was possible.
    pub fn rotate_left(&mut self) -> bool {
        unimplemented!()
    }

    pub fn minimize_depth(&mut self) {
        unimplemented!()
    }
}

impl From<Expr> for RExpr {
    fn from(expr: Expr) -> RExpr {
        unimplemented!()
    }
}
