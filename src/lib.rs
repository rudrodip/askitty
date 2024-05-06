pub mod cli;
pub mod ai;

use cli::{Config, Command};
use ai::LLMClient;
use std::env;

pub async fn run(config: Config) {
    let client = LLMClient::new(
        env::var("LLM_HOST").unwrap(),
        env::var("LLM_MODEL").unwrap(),
        env::var("LLM_API_KEY").unwrap(),
    ).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    match config.command {
        Command::Help => cli::help(),
        Command::Version => cli::version(),
        Command::Message(message) => client.run(&message).await,
        Command::Imagine(message) => println!("Imagine message: {}", message),
    }
}