pub mod ai;
pub mod cli;
pub mod errors;
pub mod types;

use ai::im::client::Client as IMClient;
use ai::llm::client::Client as LLMClient;
use cli::config::Config;
use std::error::Error;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    cli::run::<LLMClient, IMClient>(config).await
}
