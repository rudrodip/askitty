pub mod config;
pub mod utils;

use crate::ai::{im::traits::IM, llm::traits::LLM};
use crate::types::llm::Message;
use config::Config;
use std::error::Error;
use termimad;

pub async fn run<L, I>(config: Config) -> Result<(), Box<dyn Error>>
where
    L: LLM + 'static,
    I: IM + 'static,
{
    match config.command {
        config::Command::Help => utils::print_help(),
        config::Command::Version => utils::print_version(),
        config::Command::Message(message) => {
            let client = L::new()?;
            let chats = vec![Message {
                role: String::from("user"),
                content: message,
            }];

            let response = client.completion(chats).await?;
            termimad::print_text(&response);

            Ok(())
        }
        config::Command::Imagine(prompt) => {
            let client = I::new()?;
            Ok(client.generate(prompt).await?)
        }
    }
}
