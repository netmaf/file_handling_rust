// use buffered reader

use std::{fs::File, io::{BufReader, Read, Result}};

fn main() -> Result<()> {
    let mut file = File::open("tasks.txt")?;
    let mut buffer = BufReader::new(file);
    let mut content = String::new();
    buffer.read_to_string(&mut content)?;
    println!("{}", content);
    Ok(())
}
