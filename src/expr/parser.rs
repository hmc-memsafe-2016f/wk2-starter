// Luis Viornery <lviornery@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value fo the `Expr`)


/*
Workflow:
start parsing additions => 
start parsing multiplications => 
start parsing literals => 
finish parsing literals => 
start parsing parens => 
nested expr parsing => 
finsish parsing parens => 
finish parsing multiplication => 
finish parsing additions
*/

use nom::digit;

use std::str;
use std::mem;
use std::str::FromStr;
use super::Expr;
use super::BinOp;

//Make sure everything's return type is an expr

named!(parens<Expr>,
  delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(literal_expr<Expr>,
  map!(
    map_res!(
        map_res!(
          digit,
          str::from_utf8
        ),
        FromStr::from_str
      ),
    //This wraps the isize in an expr literal
    Expr::Literal
  )
);

named!(literal_expr_wrapper<Expr>,
  alt!(
    //If we uncomment these, we get sad segfaults - this is a whitespace implementation attempt
    //preceded!(char!(' '),literal_expr_wrapper) |
    //terminated!(literal_expr_wrapper, char!(' ')) |
    literal_expr
  )
);

named!(factor<Expr>,
  alt!(
  literal_expr_wrapper
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <Expr>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 //We have to map! to return a parser
                 map!(preceded!(tag!("*"), factor), |next|{
                 //mem::swap trickery to avoid moving acc into the closure
                 let mut new_expr = Expr::Literal(0);
                 new_expr = mem::replace(&mut acc, new_expr);
                 mem::swap(&mut acc, &mut Expr::BinOp(Box::new(new_expr), BinOp::Times, Box::new(next)))}) |
                 map!(preceded!(tag!("/"), factor), |next|{
                 let mut new_expr = Expr::Literal(0);
                 new_expr = mem::replace(&mut acc, new_expr);
                 mem::swap(&mut acc, &mut Expr::BinOp(Box::new(new_expr), BinOp::Over, Box::new(next)))})
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
                 map!(preceded!(tag!("+"), term), |next|{
                 let mut new_expr = Expr::Literal(0);
                 new_expr = mem::replace(&mut acc, new_expr);
                 mem::swap(&mut acc, &mut Expr::BinOp(Box::new(new_expr), BinOp::Plus, Box::new(next)))}) |
                 map!(preceded!(tag!("-"), term), |next|{
                 let mut new_expr = Expr::Literal(0);
                 new_expr = mem::replace(&mut acc, new_expr);
                 mem::swap(&mut acc, &mut Expr::BinOp(Box::new(new_expr), BinOp::Minus, Box::new(next)))})
               )
             ),
    || { return acc }
  )
);
