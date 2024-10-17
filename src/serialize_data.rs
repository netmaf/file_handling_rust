// create a struct and write it to file

use serde::{ser, Serialize};

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let p1 = Person {
        name: String::from("Sunny"),
        age: 20
    };

    let serialized = serde_json::to_string(&p1).unwrap();

    println!("{}", serialized)
}
