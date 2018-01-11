// #![allow(dead_code)]

enum WebEvent {
    // An enum may either be unit-like
    PageLoad,
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or like structures
    Click { x: i64, y: i64 }
}

// takes a WebEvent as an argument and return nothin
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure c from inside the enum
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted '{}'.", s),
        // Destructure Click into x and y
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y)
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned() creates an owned string from a string splice
    // string literals (e.g. "my text") are of type &'static str
    let pasted = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}