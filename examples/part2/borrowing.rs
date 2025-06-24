// Rust - This won't compile
fn safe_rust() -> &i32 {
    let local = 42;
    &local // Error: borrowed value does not live long enough
}

fn rust_borrowing() {
    let mut x = 5;
    let ref1 = &x; // Immutable borrow
    let ref2 = &x; // Multiple immutable borrows OK

    // let ref3 = &mut x; // Error: cannot borrow as mutable

    println!("{} {}", ref1, ref2);

    let ref_mut = &mut x; // Now we can have mutable borrow
    *ref_mut = 10;
    // println!("{}", ref1); // Error: immutable borrow used here
}

fn main() {
    let i = safe_rust();
    rust_borrowing()
}
