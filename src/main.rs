extern crate dotenv;
use askitty;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mut args = env::args();
    args.next();

    let query = args.collect::<Vec<String>>().join(" ");

    let client = askitty::LLMClient::new(
        env::var("LLM_HOST").unwrap(),
        env::var("LLM_MODEL").unwrap(),
        env::var("LLM_API_KEY").unwrap(),
    ).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    let _ = client.run(&query).await; // Add .await to use the future
}
