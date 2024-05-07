pub mod config;

use std::fs;
use config::Config;
use crate::ai::{llm::{client::Client as LLMClient, traits::LLM}, im::{client::Client as IMClient, traits::IM}};
use crate::types::llm::Message;
use termimad;

pub fn run(config: Config) {
    match config.command {
        config::Command::Help => {
            print_help();
        }
        config::Command::Version => {
            print_version();
        }
        config::Command::Message(message) => {
            let client = LLMClient::new();
            let chats = vec![Message { role: String::from("user"), content: message }];

            match client {
                Ok(client) => {
                    let response = client.completion(chats);
                    match response {
                        Ok(response) => {
                            termimad::print_text(&response);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }                
            }
        }
        config::Command::Imagine(prompt) => {
            let client = IMClient::new();
            match client {
                Ok(client) => {
                    let response = client.generate(prompt);
                    match response {
                        Ok(_) => {
                            println!("Image generated successfully");
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

fn print_help() {
    println!("Usage: askitty [FLAG] [MESSAGE]");
    println!();
    println!("Flags:");
    println!("  -h, --help       Display help message");
    println!("  -v, --version    Display version");
    println!("  -m, --message    Message to send to the model");
    println!("  -i, --imagine    Generate image from text");
}

fn print_version() {
  let contents = fs::read_to_string("Cargo.toml")
      .expect("Something went wrong reading the file");
  let version = contents.lines().nth(2).unwrap().split(" = ").nth(1).unwrap();
  println!("Version: {}", version);
}