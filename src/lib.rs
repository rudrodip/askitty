pub mod ai;
pub mod cli;
pub mod errors;
pub mod types;

use cli::config::Config;
use std::error::Error;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(cli::run(config).await.unwrap())
}
