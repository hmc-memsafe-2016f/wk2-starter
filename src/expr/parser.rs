// Dan Obermiller <dobermiller16@cmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use nom::digit;

use expr::{Expr, BinOp};
use std::str;
use std::mem;
use std::str::FromStr;

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
    Expr::Literal
  )
);

named!(factor<Expr>,
  alt!(
    i64_literal
  | parens
  )
);

fn lambda_y(acc: &mut Expr, op: BinOp, e: Expr) {
    let tmp = mem::replace(acc, Expr::Literal(0));
    mem::replace(acc, Expr::BinOp(Box::new(tmp), op, Box::new(e)));
}

// we define acc as mutable to update its value whenever a new term is found
named!(term <Expr>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 map!(
                     preceded!(tag!("*"), factor),
                     |e: Expr| {
                        println!("factor e * {}", e);
                        println!("factor acc * {}", acc);
                         lambda_y(&mut acc, BinOp::Times, e)
                     }) |
                 map!(
                     preceded!(tag!("/"), factor),
                     |e: Expr| {
                        println!("factor e / {}", e);
                        println!("factor acc / {}", acc);lambda_y(&mut acc, BinOp::Over, e)})
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
                 map!(
                     preceded!(tag!("+"), factor),
                     |e: Expr| {
                        println!("expr e + {}", e);
                        println!("expr acc + {}", acc);lambda_y(&mut acc, BinOp::Plus, e)}) |
                 map!(
                     preceded!(tag!("-"), term),
                     |e: Expr| {
                        println!("expr e - {}", e);
                        println!("expr acc - {}", acc);lambda_y(&mut acc, BinOp::Minus, e)})
               )
             ),
    || { return acc }
  )
);
