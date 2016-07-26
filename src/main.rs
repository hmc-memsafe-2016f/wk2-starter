#[macro_use]
extern crate nom;

use nom::{IResult,digit};

// Parser definition

use std::io;
use std::str;
use std::str::FromStr;

named!(parens<i64>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(i64_digit<i64>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

// We transform an integer string into a i64
// we look for a digit suite, and try to convert it.
// if either str::from_utf8 or FromStr::from_str fail,
// the parser will fail
named!(factor<i64>,
  alt!(
    i64_digit
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <i64>,
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

named!(expr <i64>,
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

fn main() {
    let mut line = String::new();

    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {

        match expr(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, res) if rest.len() == 0 => println!("{}", res),
            _ => println!("Error"),
        }

        line.clear();
    }
}
