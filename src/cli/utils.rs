use std::{error::Error, fs};

pub fn print_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: askitty [FLAG] [MESSAGE]");
    println!();
    println!("Flags:");
    println!(" -h, --help Display help message");
    println!(" -v, --version Display version");
    println!(" -m, --message Message to send to the model");
    println!(" -i, --imagine Generate image from text");
    Ok(())
}

pub fn print_version() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Cargo.toml")?;
    let version = contents
        .lines()
        .nth(2)
        .ok_or_else(|| "Could not find version in Cargo.toml".to_owned())?
        .split(" = ")
        .nth(1)
        .ok_or_else(|| "Could not parse version from Cargo.toml".to_owned())?;
    println!("Version: {}", version);
    Ok(())
}

pub fn read_line() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}