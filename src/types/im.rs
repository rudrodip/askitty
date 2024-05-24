use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageData {
    pub revised_prompt: String,
    pub url: String,
}