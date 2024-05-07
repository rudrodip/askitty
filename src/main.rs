extern crate dotenv;
use askitty;

fn main() {
    dotenv::dotenv().ok();
    askitty::run();
}
