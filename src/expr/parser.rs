// Julien Chien <jchien@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)

use nom::digit;

use std::str;
use std::str::FromStr;

named!(parens<Expr>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(i64_literal<Expr>,
  map (
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

// we define acc as mutable to update its value whenever a new term is found
named!(term <Expr>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 tap!(mul: preceded!(tag!("*"), factor) => acc = acc * mul) |
                 tap!(div: preceded!(tag!("/"), factor) => acc = acc / div)
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
                 tap!(add: preceded!(tag!("+"), term) => acc = acc + add) |
                 tap!(sub: preceded!(tag!("-"), term) => acc = acc - sub)
               )
             ),
    || { return acc }
  )
);
