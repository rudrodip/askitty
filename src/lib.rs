pub mod ai;
pub mod cli;
pub mod errors;
pub mod setup;
pub mod storage;
pub mod types;

use ai::im::client::Client as IMClient;
use ai::llm::client::Client as LLMClient;
use cli::config::Config;
use std::error::Error;
use storage::Storage;

pub async fn run(config: Config, storage: impl Storage) -> Result<(), Box<dyn Error>> {
    cli::run::<LLMClient, IMClient>(config, storage).await
}
