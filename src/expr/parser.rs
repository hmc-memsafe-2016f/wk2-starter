// Ross Mawhorter <rmawhorter@g.hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value of the `Expr`)

use nom::digit;

use std::str;
use std::str::FromStr;

use super::Expr;
use super::BinOp;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);


named!(expr_literal<Expr>,
  map!(isize_literal,
    Expr::Literal
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

named!(i64_literal<i64>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
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
                 tap!(mul: preceded!(tag!("*"), factor) => acc = Expr::BinOp(Box::new(acc), BinOp::Times, Box::new(mul))) |
                 tap!(div: preceded!(tag!("/"), factor) => acc = Expr::BinOp(Box::new(acc), BinOp::Over, Box::new(div)))
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
                 tap!(add: preceded!(tag!("+"), term) => acc = Expr::BinOp(Box::new(acc), BinOp::Plus, Box::new(add))) |
                 tap!(sub: preceded!(tag!("-"), term) => acc = Expr::BinOp(Box::new(acc), BinOp::Minus, Box::new(sub)))
               )
             ),
    || { return acc }
  )
);
