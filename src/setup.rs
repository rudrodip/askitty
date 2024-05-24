use crate::storage::KvStore;
use config::{Config, File};
use dirs::config_dir;
use std::path::PathBuf;

pub fn initialize_storage() -> KvStore {
    let db_path = PathBuf::from("./data/db");

    if !db_path.parent().unwrap().exists() {
        std::fs::create_dir_all(db_path.parent().unwrap())
            .expect("Failed to create storage directory");
    }

    KvStore::new(db_path)
}

pub fn get_config() -> Config {
    let config_builder = Config::builder();
    let config_dir = config_dir().expect("Failed to get config directory");
    let config_path = config_dir.join(format!("{}/ai.toml", "askitty"));

    // if no config file exists, generate one
    if !config_path.exists() {
        config_setup_cli();
    }

    config_builder
        .add_source(File::from(config_path))
        .build()
        .expect("Failed to build config")
}

fn genetate_config(
    llm_host: &str,
    llm_api_key: &str,
    llm_model: &str,
    im_host: &str,
    im_api_key: &str,
    im_model: &str,
) {
    let config = format!(
        r#"
[llm]
host = "{}"
api_key = "{}"
llm_model = "{}"

[im]
host = "{}"
api_key = "{}"
image_model = "{}"
"#,
        llm_host, llm_api_key, llm_model, im_host, im_api_key, im_model
    );

    let config_dir = config_dir().expect("Failed to get config directory");
    let config_path = config_dir.join(format!("{}/ai.toml", "askitty"));
    if !config_path.parent().unwrap().exists() {
        std::fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");
    }

    std::fs::write(config_path, config).expect("Failed to write config file");
}

pub fn config_setup_cli() {
    let mut llm_host = String::from("https://api.openai.com/v1");
    let mut llm_model = String::from("gpt-4");
    let llm_api_key;

    let mut im_host = String::from("https://api.openai.com/v1");
    let mut im_model = String::from("dall-e-3");
    let im_api_key;

    println!("Welcome to the askitty setup");
    println!("Let's start by setting up the language model");

    println!("Default LLM host: {}", llm_host);
    println!("Enter the LLM host (default: {}): ", llm_host);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    if !input.is_empty() {
        llm_host = input.clone();
    }

    println!("Default LLM model: {}", llm_model);
    println!("Enter the LLM model (default: {}): ", llm_model);

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    if !input.is_empty() {
        llm_model = input.clone();
    }

    println!("Enter the LLM API key: ");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    llm_api_key = input.trim().to_string();


    println!("Let's start by setting up the image model");

    println!("Default IM host: {}", im_host);
    println!("Enter the IM host (default: {}): ", im_host);

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    if !input.is_empty() {
        im_host = input.trim().to_string();
    }

    println!("Default IM model: {}", im_model);
    print!("Enter the IM model (default: {}): ", im_model);
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    if !input.is_empty() {
        im_model = input.trim().to_string();
    }

    println!("Enter the IM API key (default: {}): ", llm_api_key);
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    if !input.trim().is_empty() {
        im_api_key = input.trim().to_string();
    } else {
        im_api_key = llm_api_key.clone();
    }

    genetate_config(&llm_host, &llm_api_key, &llm_model, &im_host, &im_api_key, &im_model);
}