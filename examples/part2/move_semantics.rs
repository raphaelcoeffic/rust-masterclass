#![allow(dead_code)]

#[derive(Debug)]
struct Resource {
    data: Vec<i32>,
    name: String,
}

impl Resource {
    fn new(size: usize, name: &str) -> Self {
        println!("Created {} with {} elements", name, size);
        Resource {
            data: vec![42; size],
            name: name.to_string(),
        }
    }
}

fn rust_move_semantics() {
    let r1 = Resource::new(1000000, "BigResource");

    // Move by default - always efficient!
    let r2 = r1; // r1 is moved, not copied

    // println!("{:?}", r1);  // Compile error: value used after move
    println!("{:?}", r2.name); // Only r2 is valid

    let mut vec = Vec::new();
    let temp = Resource::new(500000, "TempResource");

    // Move by default - no std::move needed!
    vec.push(temp); // temp is moved into vec

    // println!("{:?}", temp);  // Compile error: value used after move

    // If you actually want to copy, you must be explicit
    #[derive(Debug, Clone)] // Must implement Clone trait
    struct CopyableResource {
        small_data: i32,
    }

    let copyable = CopyableResource { small_data: 42 };
    let copied = copyable.clone(); // Explicit copy
    println!("Original: {:?}, Copy: {:?}", copyable, copied); // Both valid
}

fn main() -> () {
    rust_move_semantics()
}
