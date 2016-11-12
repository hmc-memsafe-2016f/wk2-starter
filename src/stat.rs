// Michael Sheely <msheely@hmc.edu>
// Implements the statistics printing for HMC's MemorySafe, week 2
//
// The prints statistics for an expression withing a Read Evaluate Print Loop

#[macro_use]
extern crate nom;

mod expr;

use nom::IResult;
use std::io;

fn main() {
    let mut line = String::new();

    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {

        match expr::parse(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, ref res) if rest.len() == 0 =>
                println!("{} {} {}", res.evaluate(), res.depth(),
                         res.operation_count()),
            _ => println!("Error"),
        }

        line.clear();
    }
}
