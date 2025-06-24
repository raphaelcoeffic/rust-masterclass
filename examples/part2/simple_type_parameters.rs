#![allow(dead_code)]
#![allow(unused_variables)]

struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

// Generic function - type inference works well
fn max_value<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn rust_generics_example() {
    let mut int_container: Container<i32> = Container::new();
    let mut string_container: Container<String> = Container::new();

    int_container.push(42);
    string_container.push("hello".to_string());

    // Type inference usually works
    let result1 = max_value(10, 20);
    let result2 = max_value(3.14, 2.71);

    // Explicit types when needed
    let result3 = max_value(10i32, 20i32);
}

fn main() {
    rust_generics_example()
}
