use serde::{ser, Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
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
        }
    ];
    
    let sertalized_data = serde_json::to_string(&persons).unwrap();
    println!("{}", sertalized_data);

    let json_string = r#"[{"name":"Sunny","age":10},{"name":"Bunny","age":20}]"#;
    let serialized_data: Vec<Person> = serde_json::from_str(&json_string).unwrap();

    println!("{:?}", serialized_data);

}
