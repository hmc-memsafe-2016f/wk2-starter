// Dan Obermiller <dobermiller16@cmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use nom::digit;

use std::str;
use std::str::FromStr;

named!(parens<i64>, delimited!(
    char!('('),
    expr,
    char!(')')
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

named!(factor<i64>,
  alt!(
    i64_literal
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <i64>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                   tap!(mul: preceded!(tag!("*"), factor) => acc = acc * mul) |
                   tap!(div: preceded!(tag!("/"), factor) => acc = acc / div)
                //  tap!(mul: preceded!(tag!("*"), factor) => Expr::BinOp(Box::new(acc), BinOp::Times, Box::new(mul))) |
                //  tap!(div: preceded!(tag!("/"), factor) => Expr::BinOp(Box::new(acc), BinOp::Over, Box::new(div)))
               )
             ),
    || { return acc }
  )
);

named!(pub expr <i64>,
  chain!(
    mut acc: term  ~
             many0!(
               alt!(
                   tap!(add: preceded!(tag!("+"), factor) => acc = acc + add) |
                   tap!(sub: preceded!(tag!("-"), factor) => acc = acc - sub)
                //  tap!(add: preceded!(tag!("+"), term) => Expr::BinOp(Box::new(acc), BinOp::Plus, Box::new(add))) |
                //  tap!(sub: preceded!(tag!("-"), term) => Expr::BinOp(Box::new(acc), BinOp::Minus, Box::new(sub)))
               )
             ),
    || { return acc }
  )
);
