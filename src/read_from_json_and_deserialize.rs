use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    
    let mut file = File::open("persons.json").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    // serialize content json buffer
    let persons: Vec<Person> = serde_json::from_str(&content).unwrap();

    println!("{:?}", persons);
}
