extern crate dotenv;
use askitty;
use askitty::cli::config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = askitty::run(config).await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
