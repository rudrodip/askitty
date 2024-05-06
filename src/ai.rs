use reqwest::Client;
use serde::{Deserialize, Serialize};
use termimad;

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
                let message = &completion.choices[0].message.content;
                termimad::print_text(&message);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
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