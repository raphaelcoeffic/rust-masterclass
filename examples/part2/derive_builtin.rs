#![allow(unused_variables)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn derive_example() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 2, y: 1 };

    // All of these work automatically:
    println!("{:?}", p1); // Debug
    println!("{}", p1 == p2); // PartialEq
    println!("{}", p1 < p3); // PartialOrd

    let cloned = p1.clone(); // Clone

    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(p1, "point1"); // Hash
}

fn main() {
    derive_example()
}
