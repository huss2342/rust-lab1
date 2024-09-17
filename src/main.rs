use std::env;

fn main() -> Result<(), u8>  {
    println!("Hello, world!");

    // return Ok(()) for success
    Ok(())
}

fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}