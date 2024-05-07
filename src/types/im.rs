use serde::{Deserialize, Serialize};

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
