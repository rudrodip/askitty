use serde::{Deserialize, Serialize};

// llm completion response
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

#[derive(Serialize, Deserialize)]
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

// image generation response
#[derive(Serialize, Deserialize)]
pub struct ImageGenResponse {
    pub id: String,
    pub model: String,
    pub version: String,
    pub input: Input,
    pub logs: String,
    pub output: Option<Vec<String>>,
    pub error: Option<String>,
    pub status: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub urls: Urls,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub guidance_scale: f32,
    pub height: i32,
    pub num_inference_steps: i32,
    pub num_outputs: i32,
    pub prompt: String,
    pub scheduler: String,
    pub width: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Urls {
    pub cancel: String,
    pub get: String,
}
