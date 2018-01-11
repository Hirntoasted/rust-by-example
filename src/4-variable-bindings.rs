fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // warnings about unused bindings can be suppressed by prefixing with _
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
}