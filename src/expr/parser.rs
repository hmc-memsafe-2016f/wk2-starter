// Daniel Sonner
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use nom::digit;

use std::str;
use std::mem;
use std::str::FromStr;
use expr::Expr;
use expr::BinOp;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(i64_literal<Expr>,
  map!(
    map_res!(
        map_res!(
          digit,
          str::from_utf8
        ),
        FromStr::from_str
    ),
    |x: i64| Expr::Literal(x)
  )
);

named!(factor<Expr>,
  alt!(
    i64_literal
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <Expr>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 map!(preceded!(tag!("*"), factor), |mul| acc = 
                    Expr::BinOp(Box::new(mem::replace(&mut acc, Expr::Literal(0)))
                                , BinOp::Times, Box::new(mul))) |
                 map!(preceded!(tag!("/"), factor), |div| acc = 
                    Expr::BinOp(Box::new(mem::replace(&mut acc, Expr::Literal(0)))
                                , BinOp::Over, Box::new(div)))
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
                 map!(preceded!(tag!("+"), term), |add| acc = 
                    Expr::BinOp(Box::new(mem::replace(&mut acc, Expr::Literal(0)))
                                , BinOp::Plus, Box::new(add))) |
                 map!(preceded!(tag!("-"), term), |sub| acc = 
                    Expr::BinOp(Box::new(mem::replace(&mut acc, Expr::Literal(0)))
                                , BinOp::Minus, Box::new(sub)))
               )
             ),
    || { return acc }
  )
);
