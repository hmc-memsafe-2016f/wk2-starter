// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use super::{Expr, BinOp};

use nom::digit;

use std::str;
use std::str::FromStr;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(isize_literal<Expr>,
  map!(
    map_res!(
      map_res!(
        digit,
        str::from_utf8
      ),
      FromStr::from_str
    ),
    Expr::Literal
  )
);

named!(factor<Expr>,
  alt!(
    isize_literal
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <Expr>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 map!(preceded!(tag!("*"), factor),
                      |mul| acc.do_op(BinOp::Times, mul)) |
                 map!(preceded!(tag!("/"), factor),
                      |div| acc.do_op(BinOp::Over, div))
               )
             ),
    || { return acc }
  )
);

named!(pub expr <Expr>,
  chain!(
    mut acc: term  ~
             many0!(
               alt!(
                 map!(preceded!(tag!("+"), term),
                      |add| acc.do_op(BinOp::Plus, add)) |
                 map!(preceded!(tag!("-"), term),
                      |sub| acc.do_op(BinOp::Minus, sub))
               )
             ),
    || { return acc }
  )
);
