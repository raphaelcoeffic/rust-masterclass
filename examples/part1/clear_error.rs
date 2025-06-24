fn rust_use_after_move_protection() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 moved to s2

    // These won't compile - use after move detected!
    println!("{}", s1);
    // s1.push_str(" World");

    println!("{}", s2); // Only s2 is valid
}

fn main() {
    rust_use_after_move_protection();
}
