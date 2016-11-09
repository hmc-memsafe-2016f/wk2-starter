// Robert "Skipper" Gonzalez <sgonzalez@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The parser for an `Expr` (currently just produces the value for the `Expr`)

use nom::digit;

use std::str;
use std::str::FromStr;

use expr::expr::{Expr, BinOp};


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




// we define acc as mutable to update its value whenever a new term is found
named!(term <Expr>,
  //chain!(
  

    map!(
      
      factor
      |acc| {
        
        many0!(
                   alt!(   
                    map!(
                      preceded!(tag!("*"), factor), 
                      |mul| {Expr::BinOp(Box::new(acc), BinOp::Times, Box::new(mul))}
                       
                    ) |
                    
                    map!(
                      preceded!(tag!("/"), factor), 
                      |div| {Expr::BinOp(Box::new(acc), BinOp::Over, Box::new(div))}
                       
                    )
                   )
                 )
                
      }
      )
        //|| {return acc }
      
      
  


);


//    mut acc: factor  ~
//             many0!(
//               alt!(   
//                map!(
//                  preceded!(tag!("*"), factor), 
//                  |mul| {Expr::BinOp(Box::new(acc), BinOp::Times, Box::new(mul))}
//                   
//                ) |
//                
//                map!(
//                  preceded!(tag!("/"), factor), 
//                  |div| {Expr::BinOp(Box::new(acc), BinOp::Over, Box::new(div))}
//                   
//                )
//               )
//             ),
//    || {return acc }

named!(pub expr <Expr>,
  chain!(
    mut acc: term  ~
             many0!(
               alt!(
              
              
              
              map!(
                preceded!(tag!("+"), term), 
                |add| {Expr::BinOp(Box::new(acc), BinOp::Plus, Box::new(add))}
                 
              ) |
              
              map!(
                preceded!(tag!("-"), term), 
                |sub| {Expr::BinOp(Box::new(acc), BinOp::Minus, Box::new(sub))}
                 
              )
              
              )
             ),
    || { return acc }
  )
  
);
