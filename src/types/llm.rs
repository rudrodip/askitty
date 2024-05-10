use std::fmt::Display;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub history: Vec<Message>,
    pub system_prompt: String,
}

impl Session {
    pub fn new() -> Self {
        Session {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Session".to_string(),
            created_at: chrono::Utc::now().timestamp(),
            history: vec![],
            system_prompt: "".to_string(),
        }
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | Id: {} | Created: {} | ChatCount: {}",
            self.name,
            self.id,
            chrono::DateTime::from_timestamp(self.created_at, 0).expect("Invalid timestamp").to_rfc2822(),
            self.history.len()
        )
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.role,
            self.content
        )
    }
}