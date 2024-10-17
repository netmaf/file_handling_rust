use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    // take a raw string
    let jsonData = r#"{
        "name": "sunny",
        "age": 10
    }
    "#;

    let deserialized: Person = serde_json::from_str(&jsonData).unwrap();

    println!("{:?}", deserialized)
}
