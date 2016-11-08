// Luis Viornery <lviornery@hmc.edu>
// Starter code for HMC's MemorySafe, week 2
//
// The Read Evaluate Print Loop

#[macro_use]
extern crate nom;

mod expr;

use nom::IResult;
use std::io;

fn main() {
    let mut line = String::new();

    while io::stdin().read_line(&mut line).map(|l| l > 0).unwrap_or(false) {

        match expr::parse(line.as_str().trim().as_bytes()) {
            IResult::Done(rest, ref res) if rest.len() == 0 => println!("{}", res.evaluate()),
            _ => println!("Error"),
        }

        line.clear();
    }
}
