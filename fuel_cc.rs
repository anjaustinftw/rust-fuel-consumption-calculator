// v0.5.8

// Fuel Consumption Calculator
// INTERFACES
//  INPUTS:    {_start_miles, _end_miles, _refilled_gallons}
//  OUTPUTS:   {_miles_driven, _refilled_gallons, _miles_per_gallon}
//
// IMPLEMENTATION:
//  Calculate:  _miles_driven = _end_miles - _start_miles
//              _miles_per_gallon = _miles_driven/_refilled_gallons

use std::env;
use std::fmt::Display;
use std::collections::HashMap;

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
    
        /*  
            Rust noobs, prepare thyselves.
            Rust is gonna make you work for your Associative Arrays.

            I didn't figure this out on my own:
            https://www.simonewebdesign.it/rust-hashmap-insert-values-multiple-types/
        */

        let mut vehicle_data: HashMap<&str, Box<dyn Display + 'static>> = HashMap::new();

        vehicle_data.insert("start",    Box::new(_start_miles));        // miles at full tank
        vehicle_data.insert("end",      Box::new(_end_miles));          // miles at tank top off
        vehicle_data.insert("gl",       Box::new(_refilled_gallons));   // gallons to top off
        vehicle_data.insert("mi",       Box::new(_miles_driven));       // miles driven from _start_ to _end_miles
        vehicle_data.insert("mpg",      Box::new(_miles_per_gallon));   // average number of miles per gallon of consumed fuel

        println!("[start:{},end:{},gl:{},mi:{},mpg:{}]", vehicle_data["start"], vehicle_data["end"], vehicle_data["gl"], vehicle_data["mi"], vehicle_data["mpg"]);

    }
    else { // Command line arguments do not exist

        println!("Usage: ./[fuel_cc].exe [start miles (u32: Must be a positive integer.)] [end miles (u32: Must be a positive integer greater in value than the \'start miles\' value.)] [gallons refilled (f32: Should contain a decimal.)]\nTry, ./fuel_cc or fuel_cc.exe 91525 91624 4.73.");

    }
   
}
