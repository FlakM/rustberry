use rustberry::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("hello world");

    let content = std::fs::read_to_string("/home/flakm/programming/flakm/rustberry/watering_demo/config.json")?;


    let setup = Setup::from_str(&content)?;

    println!("{:?}", setup);


    Ok(())
}