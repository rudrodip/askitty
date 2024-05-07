pub mod cli;
pub mod ai;
pub mod types;
pub mod errors;

pub fn run() {
    let config = cli::config::Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    cli::run(config);
}