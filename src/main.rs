// create new file and write bytes to it

use std::{fs::File, io::{Result, Write}};

fn main() -> Result<()>{

    let mut file = File::create("tasks.txt")?;
    file.write_all(b"Hello")?;
    println!("{:?}", b"Hello".to_vec());

    Ok(())
}
