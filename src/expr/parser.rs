// Michael Sheely <msheely@hmc.edu>
// Parsing assinment for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use expr::Expr;
use expr::BinOp;

use nom::digit;

use std::str;
use std::mem;
use std::str::FromStr;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(parse_isize<isize>,
    map_res!(
      map_res!(digit, str::from_utf8),
      FromStr::from_str
    )
);

named!(isize_literal<Expr>,
    map!(parse_isize, |x| Expr::Literal(x))
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
                 map!(preceded!(tag!("*"), term), |prod: Expr| {
                     match acc {
                         Expr::Literal(num) => {
                           acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Times, Box::new(prod));
                         },
                         Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                             // swap out each of acc's values
                             let xx = mem::replace(x, Box::new(prod));
                             let oper = mem::replace(op, BinOp::Times);
                             let yy = mem::replace(y, Box::new(Expr::Literal(0)));
                             mem::replace(y, Box::new(Expr::BinOp(xx, oper, yy)));
                         }
                     }}) |
                 map!(preceded!(tag!("/"), term), |div: Expr| {
                     match acc {
                         Expr::Literal(num) => {
                           acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Over, Box::new(div));
                         },
                         Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                           let yy = mem::replace(x, Box::new(Expr::Literal(0)));
                           let oper = mem::replace(op, BinOp::Over);
                           let xx = mem::replace(y, Box::new(div));
                           mem::replace(x, Box::new(Expr::BinOp(xx, oper, yy)));
                         }
                 }})
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
                 map!(preceded!(tag!("+"), term), |add: Expr| {
                     match acc {
                         Expr::Literal(num) => {
                           acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Plus, Box::new(add));
                         },
                         Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                             // swap out each of acc's values
                             let xx = mem::replace(x, Box::new(add));
                             let oper = mem::replace(op, BinOp::Plus);
                             let yy = mem::replace(y, Box::new(Expr::Literal(0)));
                             mem::replace(y, Box::new(Expr::BinOp(xx, oper, yy)));
                         }
                     }}) |
                 map!(preceded!(tag!("-"), term), |sub: Expr| {
                     match acc {
                         Expr::Literal(num) => {
                           acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Minus, Box::new(sub));
                         },
                         Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                           let yy = mem::replace(x, Box::new(Expr::Literal(0)));
                           let oper = mem::replace(op, BinOp::Minus);
                           let xx = mem::replace(y, Box::new(sub));
                           mem::replace(x, Box::new(Expr::BinOp(xx, oper, yy)));
                         }
                 }})
               )
             ),
    || { return acc }
  )
);
