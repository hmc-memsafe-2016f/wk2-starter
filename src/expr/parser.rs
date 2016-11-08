// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use nom::digit;

use expr::Expr;
use expr::BinOp;
use std::str;
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


named!(term<Expr>,
  alt!(
    factor 
  | chain!(
      t: term ~ tag!("*") ~ f: factor, 
      || {Expr::BinOp(Box::new(t), BinOp::Times, Box::new(f))}
    )
  | chain!(
      t: term ~ tag!("/") ~ f: factor, 
      || {Expr::BinOp(Box::new(t), BinOp::Over, Box::new(f))}
    )
  )
);

named!(pub expr<Expr>,
  alt!(
    term 
  | chain!(
      e: expr ~ tag!("+") ~ t: term, 
      || {Expr::BinOp(Box::new(e), BinOp::Plus, Box::new(t))}
    )
  | chain!(
      e: expr ~ tag!("-") ~ t: term, 
      || {Expr::BinOp(Box::new(e), BinOp::Minus, Box::new(t))}
    )
  )
);
