// v0.5.1

// Fuel Consumption Calculator
// INTERFACES
//  INPUTS:    {_start_miles, _end_miles, _refilled_gallons}
//  OUTPUTS:   {_miles_driven, _refilled_gallons, _miles_per_gallon}
//
// IMPLEMENTATION:
//  Calculate:  _miles_driven = _end_miles - _start_miles
//              _miles_per_gallon = _miles_driven/_refilled_gallons

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

    // Collect command line arguments into an array
    let args: Vec<String> = env::args().collect();

    
    if args.len() > 1 { // Command line arguments exist

        let _start_miles:       u32   = args[1].parse::<u32>().unwrap_or(0);
        let _end_miles:         u32   = args[2].parse::<u32>().unwrap_or(0);
        let _refilled_gallons:  f32   = args[3].parse::<f32>().unwrap_or(0.0);
        let _miles_driven:      f32   = (_end_miles - _start_miles) as f32;
        let _miles_per_gallon:  f32   = _miles_driven/_refilled_gallons;
    
        println!("Start miles = {0}, End miles = {1}, Refill = {2} gallons.", _start_miles, _end_miles, _refilled_gallons);
    
        println!("Miles driven = {0}, and miles per gallon = {1}.", _miles_driven, _miles_per_gallon);
     
    }
    else { // Command line arguments do not exist

        println!("Usage: ./fuel_cc|fuel.exe [start miles (Must be a positive integer.)] [end miles (Must be a positive integer greater in value than the \'start miles\' value.)] [gallons refilled (Should contain decimal.)]");

    }
   
}
