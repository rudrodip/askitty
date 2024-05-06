use std::{error::Error, fs};

pub enum Command {
    Help,
    Version,
    Message(String),
    Imagine(String),
}

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "-h" | "--help" => Command::Help,
                    "-v" | "--version" => Command::Version,
                    "-m" | "--message" => {
                        let message = match args.next() {
                            Some(message) => message,
                            None => return Err("No message provided"),
                        };
                        Command::Message(message)
                    }
                    "-i" | "--imagine" => {
                        let message = match args.next() {
                            Some(message) => message,
                            None => return Err("No message provided"),
                        };
                        Command::Imagine(message)
                    }
                    _ => return Err("Invalid flag"),
                }
            }
            None => return Err("No flag provided"),
        };

        Ok(Config { command })
    }
}

pub fn help() -> Result<(), Box<dyn Error>> {
    println!("Usage: askitty [FLAG] [MESSAGE]");
    println!();
    println!("Flags:");
    println!("  -h, --help       Display help message");
    println!("  -v, --version    Display version");
    println!("  -m, --message    Message to send to the model");
    println!("  -i, --imagine    Generate image from text");

    Ok(())
}

pub fn version() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Cargo.toml")
        .expect("Something went wrong reading the file");
    let version = contents.lines().nth(2).unwrap().split(" = ").nth(1).unwrap();
    println!("Version: {}", version);

    Ok(())
}