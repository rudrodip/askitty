extern crate dotenv;
use askitty::cli::config::Config;
use askitty::setup;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let storage = setup::initialize_storage();

    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = askitty::run(config, storage).await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
