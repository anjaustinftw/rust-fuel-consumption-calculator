# Fuel Consumption Calculator

This is my first command line interface program in Rust. Its function is to calculate the gas mileage of a vehicle based on actual miles driven and the amount of fuel required to top off the vehicle's tank.
 
## Calculate the rate of fuel consumption

#### Core Implementation

> `Calculate:  _miles_driven = _end_miles - _start_miles, _miles_per_gallon = _miles_driven/_refilled_gallons`

## Compile & Run
### Linux
> `$ rustc fuel_cc.rs`

> `$ sudo chmod +x fuel_cc`

#### Get usage at the command line by calling the executable without any arguments.
> `$ ./fuel_cc`

> `Usage: ./[fuel_cc].exe [start miles (u32: Must be a positive integer.)] [end miles (u32: Must be a positive integer greater in value than the 'start miles' value.)] [gallons refilled (f32: Should contain a decimal.)]`

> `Try, ./fuel_cc or fuel_cc.exe 91525 91624 4.73.`

> `$ ./fuel_cc 91525 91624 4.73`

> `[start:91525,end:91624,gl:4.73,mi:99,mpg:20.930233]`

> `$  `
 
 
### Windows 10

> `C:\>rustc fuel_cc.rs`

#### Get usage at the command line by calling the executable without any arguments.
> `C:\>fuel_cc.exe`

> `Usage: ./[fuel_cc].exe [start miles (u32: Must be a positive integer.)] [end miles (u32: Must be a positive integer greater in value than the 'start miles' value.)] [gallons refilled (f32: Should contain a decimal.)]`

> `Try, ./fuel_cc or fuel_cc.exe 91525 91624 4.73.`

> `C:\>fuel_cc 91525 91624 4.73`

> `[start:91525,end:91624,gl:4.73,mi:99,mpg:20.930233]`

> `C:\>  `
 
 
## License
The Fuel Consumption Calculator is licensed under the terms of the GNU General Public License version 3.
 

## Credits
##### Aaron N. Josserand-Austin
