pub mod cli;
pub mod ai;

use std::error::Error;
use cli::{Config, Command};
use ai::LLMClient;
use std::env;
use termimad;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let llm_client = LLMClient::new(
        env::var("LLM_HOST").unwrap(),
        env::var("LLM_MODEL").unwrap(),
        env::var("LLM_API_KEY").unwrap(),
    ).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    match config.command {
        Command::Help => cli::help(),
        Command::Version => cli::version(),
        Command::Message(message) => {
            let response = llm_client.run(&message).await?;
            termimad::print_text(&response);
            Ok(())
        
        },
        Command::Imagine(message) => {
            println!("Imagine: {}", message);
            Ok(())
        },
    }
}