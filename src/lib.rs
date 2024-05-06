extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::env;

/*
-h or --help: Display help message
-v or --version: Display version
-m or --message: Message to send to the model
-i or --imagine: Generate image from text
*/

pub enum Command {
    Help,
    Version,
    Message(String),
    Imagine(String),
}

pub struct Config {
    pub command: Command,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "-h" | "--help" => Command::Help,
                    "-v" | "--version" => Command::Version,
                    "-m" | "--message" => {
                        let message = match args.next() {
                            Some(message) => message,
                            None => return Err("No message provided"),
                        };
                        Command::Message(message)
                    },
                    "-i" | "--imagine" => {
                        let message = match args.next() {
                            Some(message) => message,
                            None => return Err("No message provided"),
                        };
                        Command::Imagine(message)
                    },
                    _ => return Err("Invalid flag"),
                }
            },
            None => return Err("No flag provided"),
        };

        Ok(Config { command })
    }
}

pub struct LLMClient {
    pub host: String,
    pub model: String,
    pub api_key: String,
}

impl LLMClient {
    pub fn new(host: String, model: String, api_key: String) -> Result<Self, &'static str> {
        Ok(LLMClient {
            host,
            model,
            api_key,
        })
    }

    pub async fn run(&self, query: &str) {
        let query = query.trim();
        let url = format!("{}/chat/completions", self.host);
        let data = format!("{{\"model\": \"{}\",\"messages\": [{{\"role\": \"system\",\"content\": \"You are a helpful assistant.\"}},{{\"role\": \"user\",\"content\": \"{}\"}}]}}", self.model, query);
        let client = Client::new();
        let res = client.post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(data)
            .send()
            .await;

        match res {
            Ok(res) => {
                let body = res.text().await.unwrap();
                let completion: Completion = serde_json::from_str(&body).unwrap();
                println!("{}", completion.choices[0].message.content);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}

pub async fn run(config: Config) {
    let client = LLMClient::new(
        env::var("LLM_HOST").unwrap(),
        env::var("LLM_MODEL").unwrap(),
        env::var("LLM_API_KEY").unwrap(),
    ).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    match config.command {
        Command::Help => {
            println!("Help message");
        },
        Command::Version => {
            println!("Version message");
        },
        Command::Message(message) => {
            let _ = client.run(&message).await;
        },
        Command::Imagine(message) => {
            println!("Imagine message: {}", message);
        },
    }
}

#[derive(Serialize, Deserialize)]
struct Completion {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    index: i32,
    message: Message,
    logprobs: Option<Logprobs>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Logprobs {
    token_logprobs: Vec<f32>,
    top_logprobs: Vec<TopLogprobs>,
}

#[derive(Serialize, Deserialize)]
struct TopLogprobs {
    token: String,
    logprob: f32,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}