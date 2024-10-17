
// Take persons array, serialize it

use std::{any::type_name, fs::File, io::Write};

use serde::Serialize;

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    age: u8
}
fn main() {
    let persons = vec![
        Person {
            name: String::from("Sunny"),
            age: 10
        },
        Person {
            name: String::from("Bunny"),
            age: 20
        },
    ];

    let ser_data = serde_json::to_string(&persons).unwrap();

    // create file and write data
    let mut file = File::create("persons.json").unwrap();
    file.write_all(&ser_data.as_bytes()).unwrap();
}
