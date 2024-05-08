pub mod cli;
pub mod ai;
pub mod types;
pub mod errors;

use std::error::Error;

use cli::config::Config;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(cli::run(config).await.unwrap())
}