pub mod config;
pub mod utils;

use crate::ai::{im::traits::IM, llm::traits::LLM};
use crate::types::llm::Message;
use crate::storage::Storage;
use config::Config;
use std::collections::VecDeque;
use std::error::Error;
use termimad;

pub async fn run<L, I>(config: Config, storage: impl Storage) -> Result<(), Box<dyn Error>>
where
    L: LLM + 'static,
    I: IM + 'static,
{
    let llm_client = L::new()?;
    let im_client = I::new()?;
    let mut chat_history: VecDeque<Message> = storage
        .get("CHAT_HISTORY")
        .unwrap_or_default()
        .unwrap_or_default();

    match config.command {
        config::Command::Help => utils::print_help(),
        config::Command::Version => utils::print_version(),
        config::Command::Message(message) => {
            let chat = Message {
                role: String::from("user"),
                content: message,
            };
            chat_history.push_back(chat);
            let chats: Vec<Message> = chat_history.iter().cloned().collect();
            let response = llm_client.completion(chats).await?;

            chat_history.push_back(Message {
                role: String::from("assistant"),
                content: response.clone(),
            });

            storage
                .set("CHAT_HISTORY", &chat_history)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;

            termimad::print_text(&response);
            Ok(())
        }
        config::Command::REPL => {
            loop {
                let prompt = utils::read_line()?;
                if prompt == "exit" {
                    break;
                }
                let chat = Message {
                    role: String::from("user"),
                    content: prompt.clone(),
                };
                chat_history.push_back(chat);
                let chats: Vec<Message> = chat_history.iter().cloned().collect();
                let response = llm_client.completion(chats).await?;

                chat_history.push_back(Message {
                    role: String::from("assistant"),
                    content: response.clone(),
                });

                storage
                    .set("CHAT_HISTORY", &chat_history)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;

                termimad::print_text(&response);
            }
            Ok(())
        }
        config::Command::Imagine(prompt) => {
            Ok(im_client.generate(prompt).await?)
        }
    }
}
