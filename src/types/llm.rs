use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Completion {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub logprobs: Option<Logprobs>,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Logprobs {
    pub token_logprobs: Vec<f32>,
    pub top_logprobs: Vec<TopLogprobs>,
}

#[derive(Serialize, Deserialize)]
pub struct TopLogprobs {
    pub token: String,
    pub logprob: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}