extern crate dotenv;
use askitty;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = askitty::Config::new(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments");
        std::process::exit(1);
    });

    askitty::run(config).await;
}
