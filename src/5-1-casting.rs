#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // no implicit conversion!
    // let integer: u8 = decimal;

    // ...but explicit conversions work!
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    
}