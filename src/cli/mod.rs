pub mod config;

use crate::ai::{
    im::{client::Client as IMClient, traits::IM},
    llm::{client::Client as LLMClient, traits::LLM},
};
use crate::types::llm::Message;
use config::Config;
use std::{error::Error, fs};
use termimad;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        config::Command::Help => print_help(),
        config::Command::Version => print_version(),
        config::Command::Message(message) => {
            let client = LLMClient::new().unwrap();
            let chats = vec![Message {
                role: String::from("user"),
                content: message,
            }];

            let response = client.completion(chats).await.unwrap();
            termimad::print_text(&response);

            Ok(())
        }
        config::Command::Imagine(prompt) => {
            let client = IMClient::new().unwrap();
            Ok(client.generate(prompt).await.unwrap())
        }
    }
}

fn print_help() -> Result<(), Box<dyn Error>> {
    println!("Usage: askitty [FLAG] [MESSAGE]");
    println!();
    println!("Flags:");
    println!("  -h, --help       Display help message");
    println!("  -v, --version    Display version");
    println!("  -m, --message    Message to send to the model");
    println!("  -i, --imagine    Generate image from text");

    Ok(())
}

fn print_version() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("Cargo.toml").expect("Something went wrong reading the file");
    let version = contents
        .lines()
        .nth(2)
        .unwrap()
        .split(" = ")
        .nth(1)
        .unwrap();
    println!("Version: {}", version);
    Ok(())
}
