use List::*;

enum List {
    // Box<T> creates a reference to a value of type T allocated on the heap (instead of the stack)
    Cons(u32, Box<List>), // tuple struct containg element and a pointer to the next value
    Nil // Nil signifies the end of the list
}

impl List {
    // create an empty list
    fn new() -> List {
        Nil
    }

    // consume a list and return the same list with the given element on its head
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // return the length of the list
    fn len(&self) -> u32 {
        // dereference self using *
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format! returns a string allocated on the heap
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}