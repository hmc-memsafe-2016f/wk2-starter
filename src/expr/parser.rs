// Zach Hauser <zachary.hauser@pomona.edu>
// Submission for HMC's MemorySafe, week 2
//
// The parser for an `Expr` 

use nom::{digit, multispace};

use expr::Expr;
use expr::BinOp;
use std::str;
use std::str::FromStr;

named!(parens<Expr>, delimited!(
    preceded!(opt!(multispace), char!('(')),
    expr,
    preceded!(opt!(multispace), char!(')'))
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
    preceded!(opt!(multispace), i64_literal)
  | parens
  )
);

named!(term<Expr>,
  chain!(
    initial: factor ~
    result: fold_many0!(
              alt!(
                map!(
                  preceded!(preceded!(opt!(multispace), tag!("*")), factor), 
                  |fact| (BinOp::Times, fact)
                )
              | map!(
                  preceded!(preceded!(opt!(multispace), tag!("/")), factor), 
                  |fact| (BinOp::Over, fact)
                )
              ),
              initial,
              |e1, (op, e2)|  Expr::BinOp(Box::new(e1), op, Box::new(e2))
            ), 
    || { return result }
  )
);

named!(pub expr<Expr>,
  chain!(
    initial: term ~
    result: fold_many0!(
              alt!(
                map!(
                  preceded!(preceded!(opt!(multispace), tag!("+")), term), 
                  |term| (BinOp::Plus, term)
                )
              | map!(
                  preceded!(preceded!(opt!(multispace), tag!("-")), term), 
                  |term| (BinOp::Minus, term)
                )
              ),
              initial,
              |e1, (op, e2)|  Expr::BinOp(Box::new(e1), op, Box::new(e2))
            ), 
    || { return result }
  )
);
