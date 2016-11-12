// Michael Sheely <msheely@hmc.edu>
// Parsing assinment for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (does not allow for whitespace).

use expr::Expr;
use expr::BinOp;

use nom::digit;

use std::mem;
use std::str;
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
                acc = Expr::BinOp(Box::new(Expr::Literal(num)),
                                  BinOp::Times, Box::new(prod));
              },
              Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                // swap out each of acc's values
                let inner_x = mem::replace(x, Box::new(prod));
                let inner_op = mem::replace(op, BinOp::Times);
                let inner_y = mem::replace(y, Box::new(Expr::Literal(0)));
                mem::replace(y, Box::new(
                        Expr::BinOp(inner_x, inner_op, inner_y)));
              }
            }
          }) |
          map!(preceded!(tag!("/"), term), |div: Expr| {
            match acc {
              Expr::Literal(num) => {
                acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Over,
                                  Box::new(div));
              },
              Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                let inner_x = mem::replace(x, Box::new(Expr::Literal(0)));
                let inner_op = mem::replace(op, BinOp::Over);
                let inner_y = mem::replace(y, Box::new(div));
                mem::replace(x, Box::new(
                        Expr::BinOp(inner_x, inner_op, inner_y)));
              }
            }
          })
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
                acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Plus,
                                  Box::new(add));
              },
              Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                // swap out each of acc's values
                let inner_x = mem::replace(x, Box::new(add));
                let inner_op = mem::replace(op, BinOp::Plus);
                let inner_y = mem::replace(y, Box::new(Expr::Literal(0)));
                mem::replace(y, Box::new(
                        Expr::BinOp(inner_x, inner_op, inner_y)));
              }
            }
          }) |
          map!(preceded!(tag!("-"), term), |sub: Expr| {
            match acc {
              Expr::Literal(num) => {
                acc = Expr::BinOp(Box::new(Expr::Literal(num)), BinOp::Minus,
                                  Box::new(sub));
              },
              Expr::BinOp(ref mut x, ref mut op, ref mut y) => {
                let inner_y = mem::replace(y, Box::new(sub));
                let inner_op = mem::replace(op, BinOp::Minus);
                let inner_x = mem::replace(x, Box::new(Expr::Literal(0)));
                mem::replace(x, Box::new(
                        Expr::BinOp(inner_x, inner_op, inner_y)));
              }
            }
          })
        )
      ),
    || { return acc }
  )
);
