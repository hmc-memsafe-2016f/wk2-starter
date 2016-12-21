// Max Howard <mhoward@hmc.edu> //
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value for the `Expr`)

use nom::digit;

use super::Expr;
use super::BinOp;

use std::mem;
use std::str;
use std::str::FromStr;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(isize_literal<isize>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(expr_literal<Expr>,
  map!(
    isize_literal,
    Expr::Literal
  )
);

named!(factor<Expr>,
  alt!(
    expr_literal
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
                    |mul| { let new_acc = mem::replace(&mut acc, Expr::Literal(0));
                            acc = Expr::BinOp(Box::new(new_acc), BinOp::Times, Box::new(mul))}) |
                 map!(preceded!(tag!("/"), factor),
                    |div| { let new_acc = mem::replace(&mut acc, Expr::Literal(0));
                            acc = Expr::BinOp(Box::new(new_acc), BinOp::Over, Box::new(div))})
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
                  |add| { let new_acc = mem::replace(&mut acc, Expr::Literal(0));
                            acc = Expr::BinOp(Box::new(new_acc), BinOp::Plus, Box::new(add))})|
                 map!(preceded!(tag!("-"), term),
                  |sub| { let new_acc = mem::replace(&mut acc, Expr::Literal(0));
                            acc = Expr::BinOp(Box::new(new_acc), BinOp::Minus, Box::new(sub))})
               )
             ),
    || { return acc }
  )
);
