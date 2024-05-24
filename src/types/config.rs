use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LLMConfig {
    pub host: String,
    pub api_key: String,
    pub llm_model: String,
}

#[derive(Debug, Deserialize)]
pub struct IMConfig {
    pub host: String,
    pub api_key: String,
    pub image_model: String,
}

#[derive(Debug, Deserialize)]
pub struct AIConfig {
    pub llm: LLMConfig,
    pub im: IMConfig,
}