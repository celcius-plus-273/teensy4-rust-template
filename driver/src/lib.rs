#![allow(unused_assignments)] // this flag is very helpful for register name definitions
#![no_std] // basically always needed when writing embedded rust :)
#![crate_type = "lib"] // determines the type of this crate
#![crate_name = "example_driver"] // this name MUST match the corresping name in the Cargo.toml file

// just a basic example public funtion that increments an u32 number
pub fn increment(num: u32) -> u32 {
    num.wrapping_add(1)
}