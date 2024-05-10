pub mod config;
mod utils;
mod repl;

use crate::ai::{im::traits::IM, llm::traits::LLM};
use crate::storage::Storage;
use crate::types::llm::Message;
use config::Config;
use std::error::Error;

pub async fn run<L, I>(config: Config, storage: impl Storage) -> Result<(), Box<dyn Error>>
where
    L: LLM + 'static,
    I: IM + 'static,
{
    let llm_client = L::new()?;
    let im_client = I::new()?;

    match config.command {
        config::Command::Help => utils::print_help(),
        config::Command::Version => utils::print_version(),
        config::Command::Message(message) => handle_message(&llm_client, message).await,
        config::Command::REPL => repl::start_repl(&llm_client, &storage).await,
        config::Command::ShowAllSessions => utils::show_all_sessions(&storage),
        config::Command::ShowSession(session_id) => utils::show_session(&storage, &session_id),
        config::Command::DeleteSession(session_id) => utils::delete_session(&storage, &session_id),
        config::Command::ClearAllSessions => utils::clear_sessions(&storage),
        config::Command::NewSession => repl::start_new_session(&llm_client, &storage).await,
        config::Command::GlobalSystemPrompt(system_prompt) => {
            utils::set_global_system_prompt(&storage, &system_prompt)
        },
        config::Command::SessionSystemPrompt(session_id, system_prompt) => {
            utils::set_session_system_prompt(&storage, &session_id, &system_prompt)
        },
        config::Command::ShowGlobalSystemPrompt => utils::show_global_system_prompt(&storage),
        config::Command::ClearGlobalSystemPrompt => utils::clear_global_system_prompt(&storage),
        config::Command::DeleteSessionSystemPrompt(session_id) => {
            utils::delete_session_system_prompt(&storage, &session_id)
        },
        config::Command::Imagine(prompt) => Ok(im_client.generate(prompt).await?),
    }
}

async fn handle_message<L: LLM>(llm_client: &L, message: String) -> Result<(), Box<dyn Error>> {
    let chat = Message {
        role: String::from("user"),
        content: message,
    };
    let response = llm_client.completion(vec![chat]).await?;
    utils::print_text(&response);
    Ok(())
}