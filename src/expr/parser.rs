// Eric Mueller 
// Based on starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use nom::digit;

use std::str;
use std::str::FromStr;

use expr::expr::Expr;
use expr::expr::BinOp;

use std::mem;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(i64_literal<Expr>,
       map_res!(
           map_res!(
               digit,
               str::from_utf8),
           Expr::from_str)
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
                       map!(preceded!(tag!("*"), factor),
                            |mul| {
                                let old = mem::replace(&mut acc, Expr::Literal(0));
                                mem::replace(&mut acc, Expr::BinOp(Box::new(old), BinOp::Times, Box::new(mul)));
                                ()
                            })
                           |
                       map!(preceded!(tag!("/"), factor),
                            |div| {
                                let old = mem::replace(&mut acc, Expr::Literal(0));
                                mem::replace(&mut acc, Expr::BinOp(Box::new(old), BinOp::Over, Box::new(div)));
                                ()
                            })
                           )),
           || { return acc }
           )
       );

named!(pub expr <Expr>,
       chain!(
           mut acc: term  ~
               many0!(
                   alt!(
                       map!(preceded!(tag!("+"), term),
                            |add| {
                                let old = mem::replace(&mut acc, Expr::Literal(0));
                                mem::replace(&mut acc, Expr::BinOp(Box::new(old), BinOp::Plus, Box::new(add)));
                                ()
                            })
                           |
                       map!(preceded!(tag!("-"), term),
                            |sub| {
                                let old = mem::replace(&mut acc, Expr::Literal(0));
                                mem::replace(&mut acc, Expr::BinOp(Box::new(old), BinOp::Minus, Box::new(sub)));
                                ()
                            })
                           )),
           || { return acc }
           )
       );
