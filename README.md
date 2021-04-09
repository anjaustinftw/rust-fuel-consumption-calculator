# Fuel Consumption Calculator

This is my first command line interface program in Rust. Its function is to calculate the gas mileage of a car based on actual miles driven and the amount of fuel required to top off the vehicle's tank.
 
## How to calculate the milage of a vehicle

#### This is the core implementation of the calculator.

> `Calculate:  _miles_driven = _end_miles - _start_miles, _miles_per_gallon = _miles_driven/_refilled_gallons`

## Compile for your system and run locally
### Linux
> `~$ rustc fuel_cc.rs`

> `~$ sudo chmod +x fuel_cc`

#### Get usage at the command line by calling the executable without any arguments.
> `~$ ./fuel_cc`

> `Usage: ./[fuel_cc].exe [start miles (u32: Must be a positive integer.)] [end miles (u32: Must be a positive integer greater in value than the 'start miles' value.)] [gallons refilled (f32: Should contain decimal.)]`

> `~$  `

### Windows 10

> `C:\>rustc fuel_cc.rs`

#### Get usage at the command line by calling the executable without any arguments.
> `C:\>fuel_cc.exe`

> `Usage: ./[fuel_cc].exe [start miles (u32: Must be a positive integer.)] [end miles (u32: Must be a positive integer greater in value than the 'start miles' value.)] [gallons refilled (f32: Should contain decimal.)]`

> `C:\>  `

## License
The Fuel Consumption Calculator is licensed under the terms of the GNU General Public License version 3.

## Credits
##### Aaron N. Josserand-Austin
