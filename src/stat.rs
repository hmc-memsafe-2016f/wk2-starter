// Adam Dunlap <adunlap@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// Prints statistics about expressions

#[macro_use]
extern crate nom;

mod expr;

use nom::IResult;
use std::io;

fn main() {
    let mut line = String::new();

    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {

        //match expr::parse(line.as_str().trim().as_bytes()) {
        match expr::parse(line.as_str().as_bytes()) {
            IResult::Done(_, res) =>
                println!("{} {} {}",
                         res.evaluate(),
                         res.depth(),
                         res.operation_count()),
            e => println!("Error {:?}", e),
        }

        line.clear();
    }
}
