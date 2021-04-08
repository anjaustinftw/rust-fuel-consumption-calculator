// Fuel Consumption Calculator
// INTERFACES
//  INPUTS:    {start_miles, end_miles, refilled_gallons}
//  OUTPUTS:   {miles_driven, refilled_gallons, miles_per_gallon}
//
// IMPLEMENTATION:
//  Calculate: miles_driven/refilled_gallons

use std::env;

fn main() {
    
    // println!("Welcome to the Fuel Consumption Calculator!\nPress <ENTER> to continue...");

    /* To enable minigrep to read the values of command line arguments we pass to it,
       we’ll need a function provided in Rust’s standard library,
       which is std::env::args. This function returns an iterator
       of the command line arguments that were given to minigrep.
       Iterators produce a series of values, and we can call the
       collect method on an iterator to turn it into a collection,
       such as a vector, containing all the elements the iterator
       produces.

       SOURCE: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    */

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

}
