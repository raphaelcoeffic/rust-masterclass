#![allow(unused_variables)]

// Option<T> - built into the language
fn rust_option_example() {
    let maybe_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    match maybe_value {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    // Or use if let for single case
    if let Some(value) = maybe_value {
        println!("Value: {}", value);
    }
}

fn main() {
    rust_option_example()
}
