// Using serde - automatic serialisation
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] // Procedural macros
struct Person {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    name: String,
    employees: Vec<Person>,
    founded: u32,
}

fn procedural_macro_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // Serialisation code generated automatically
    let json = serde_json::to_string(&person).unwrap();
    println!("JSON: {}", json);

    // Deserialisation also works
    let parsed: Person = serde_json::from_str(&json).unwrap();
    println!("Parsed: {:?}", parsed);
}

fn main() {
    procedural_macro_example()
}
