extern crate dotenv;
use askitty;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = askitty::cli::Config::new(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments");
        std::process::exit(1);
    });

    askitty::run(config).await.unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    });
}
