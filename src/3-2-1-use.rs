#![allow(dead_code)]

enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Soldier
}

fn main() {
    // explicitly use each name so they are available without manual scoping
    use Status::{Poor, Rich};
    // use each name inside Work
    use Work::*;

    let status = Poor; // => let status = Status::Poor
    let work = Civilian; // => let work = Work::Civilian

    match status {
        // Note no scoping here
        Rich => println!("Eat the rich"),
        Poor => println!("Something something poor")
    }

    match work {
        // Note no scoping here as well
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}