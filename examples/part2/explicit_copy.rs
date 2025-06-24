#![allow(dead_code)]

// If you actually want to copy, you must be explicit
#[derive(Debug, Clone)] // Must implement Clone trait
struct CopyableResource {
    small_data: i32,
}

fn rust_copy() {
    let copyable = CopyableResource { small_data: 42 };
    let copied = copyable.clone(); // Explicit copy
    println!("Original: {:?}, Copy: {:?}", copyable, copied); // Both valid
}

fn main() -> () {
    rust_copy()
}
