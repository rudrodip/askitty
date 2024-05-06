pub mod cli;
pub mod ai;
pub mod types;

use std::error::Error;
use cli::{Config, Command};
use ai::{LLMClient, ImageGenClient};
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

    let image_client = ImageGenClient::new(
        env::var("REPLICATE_HOST").unwrap(),
        env::var("IMAGE_MODEL_VERSION").unwrap(),
        env::var("REPLICATE_API_KEY").unwrap(),
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
            image_client.run(&message).await?;
            Ok(())
        },
    }
}