use std::env;
use std::error::Error;

#[derive(Debug)]
pub enum Command {
    Help,
    Version,
    Message(String),
    Imagine(String),
}

#[derive(Debug)]
pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("No command provided".into());
        }

        let command = match args[1].as_str() {
            "-h" | "--help" => Command::Help,
            "-v" | "--version" => Command::Version,
            "-m" | "--message" => {
                if args.len() < 3 {
                    return Err("No message provided".into());
                }
                Command::Message(args[2].clone())
            }
            "-i" | "--imagine" => {
                if args.len() < 3 {
                    return Err("No prompt provided".into());
                }
                Command::Imagine(args[2].clone())
            }
            _ => return Err("Invalid command".into()),
        };

        Ok(Config { command })
    }
}
