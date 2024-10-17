use std::{fs::File, io::{Error, Read, Result}};



fn main() -> Result<()> {
    let mut file = File::open("tasks.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(())
}
