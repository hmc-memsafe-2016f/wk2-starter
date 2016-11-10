// Adam Dunlap <adunlap@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr`

use nom::{digit, space};

use std::str;
use std::str::FromStr;
use expr::{Expr, BinOp};

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

named!(pub factor<Expr>,
  complete!(terminated!(preceded!(opt!(space),
    alt!(
        i64_literal
      | parens
    )),
    opt!(space)))
);

named!(pub term<Expr>,
  complete!(terminated!(preceded!(opt!(space),
  chain!(
      first: factor
    ~ rest: many0!(pair!(alt!(char!('*') | char!('/')), factor)),
    || {
      let mut ret: Expr = first;
      for (op, fac) in rest {
        ret = Expr::BinOp(Box::new(ret), 
                          match op {'*' => BinOp::Times,
                                    '/' => BinOp::Over,
                                    _   => unreachable!()},
                          Box::new(fac));
      }
      ret})),
    opt!(space)))
);

named!(pub expr<Expr>,
  complete!(terminated!(preceded!(opt!(space),
  chain!(
      first: term
    ~ rest: many0!(pair!(alt!(char!('+') | char!('-')), term)),
    || {
      let mut ret: Expr = first;
      for (op, e) in rest {
        ret = Expr::BinOp(Box::new(ret), 
                          match op {'+' => BinOp::Plus,
                                    '-' => BinOp::Minus,
                                    _   => unreachable!()},
                          Box::new(e));
      }
      ret
    })),
    opt!(space)))
);

//named!(pub term<Expr>,
//   alt_complete!(
//       chain!(left: factor ~ op: alt!(char!('*') | char!('/')) ~ right: term,
//         || Expr::BinOp(Box::new(left),
//                        match op {'*' => BinOp::Times,
//                                  '/' => BinOp::Over,
//                                  _   => unreachable!()},
//                        Box::new(right)))
//       | factor
//   )
//);
//
//
//named!(pub expr<Expr>,
//  alt_complete!(
//      chain!(left: term ~ op: alt!(char!('+') | char!('-')) ~ right: expr,
//        || Expr::BinOp(Box::new(left),
//                       match op {'+' => BinOp::Plus,
//                                 '-' => BinOp::Minus,
//                                 _   => unreachable!()},
//                       Box::new(right)))
//    | term
//  )
//);
