fn main() {
    // this lives in the main function
    let long_lived_binding = 1;

    // this is a block
    {
        //this only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        //this shadowes the outer binding
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // block ends here

    // doesn't exist in this scope
    // prinln!("inner short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // this shadows the binding from the beginning
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}