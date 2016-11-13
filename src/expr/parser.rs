// Jackson Warley
//
// The parser for an `Expr`

use nom::digit;
use std::str;
use std::str::FromStr;
use std::mem;
use expr::Expr;
use expr::BinOp;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(isize_digit<isize>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(literal<Expr>,
  map!(
    isize_digit,
    Expr::Literal
  )
);

named!(factor<Expr>,
  alt!(
    literal
  | parens
  )
);

named!(term <Expr>,
  chain!(
    mut acc: factor ~
             many0!(
               alt!(
                 map!(preceded!(tag!("*"), factor),
                      |mul| { match acc {
                          Expr::Literal(x) => acc =
                              Expr::BinOp(Box::new(Expr::Literal(x)), BinOp::Times, Box::new(mul)),
                          Expr::BinOp(ref mut left, ref mut op, ref mut right) => {
                              let temp = Expr::Literal(0);
                              let l = mem::replace(left, Box::new(temp));
                              let r = mem::replace(right, Box::new(mul));
                              let o = mem::replace(op, BinOp::Times);
                              mem::replace(left, Box::new(Expr::BinOp(l, o, r)));
                          }
                      }}) |
                 map!(preceded!(tag!("/"), factor),
                      |div| { match acc {
                          Expr::Literal(x) => acc =
                              Expr::BinOp(Box::new(Expr::Literal(x)), BinOp::Over, Box::new(div)),
                          Expr::BinOp(ref mut left, ref mut op, ref mut right) => {
                              let temp = Expr::Literal(0);
                              let l = mem::replace(left, Box::new(temp));
                              let r = mem::replace(right, Box::new(div));
                              let o = mem::replace(op, BinOp::Over);
                              mem::replace(left, Box::new(Expr::BinOp(l, o, r)));
                          }
                      }})
               )
             ),
      || { return acc }
  )
);


named!(pub expr <Expr>,
  chain!(
    mut acc: term ~
             many0!(
               alt!(
                 map!(preceded!(tag!("+"), term),
                      |add| { match acc {
                          Expr::Literal(x) => acc =
                              Expr::BinOp(Box::new(Expr::Literal(x)), BinOp::Plus, Box::new(add)),
                          Expr::BinOp(ref mut left, ref mut op, ref mut right) => {
                              let temp = Expr::Literal(0);
                              let l = mem::replace(left, Box::new(temp));
                              let r = mem::replace(right, Box::new(add));
                              let o = mem::replace(op, BinOp::Plus);
                              mem::replace(left, Box::new(Expr::BinOp(l, o, r)));
                          }
                      }}) |
                 map!(preceded!(tag!("-"), term),
                      |sub| { match acc {
                          Expr::Literal(x) => acc =
                              Expr::BinOp(Box::new(Expr::Literal(x)), BinOp::Minus, Box::new(sub)),
                          Expr::BinOp(ref mut left, ref mut op, ref mut right) => {
                              let temp = Expr::Literal(0);
                              let l = mem::replace(left, Box::new(temp));
                              let r = mem::replace(right, Box::new(sub));
                              let o = mem::replace(op, BinOp::Minus);
                              mem::replace(left, Box::new(Expr::BinOp(l, o, r)));
                          }
                      }})
               )
             ),
    || { return acc }
  )
);
