#![allow(unused_variables)]

fn main() {
    // declare here, initialize later
    let a_binding;

    {
        let x= 2;

        // initialize
        a_binding = x * x;
    }

    println!("a binding {}", a_binding);

    let another_binding;
    let another_binding_with_type: f32;

    // error - cannot infer type
    // println!("another binding {}", another_binding);

    // error - use of uninitialized binding
    // println!("another binding  with type {}", another_binding_with_type);

    another_binding = 1;
    println!("another binding {}", another_binding);
}