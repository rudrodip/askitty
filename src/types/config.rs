use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

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

impl Display for LLMConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Host: {}\nAPI Key: {}\nModel: {}",
            self.host, self.api_key, self.llm_model
        )
    }
}

impl Display for IMConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Host: {}\nAPI Key: {}\nModel: {}",
            self.host, self.api_key, self.image_model
        )
    }
}

impl Display for AIConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LLM Config:\n{}\nIM Config:\n{}", self.llm, self.im)
    }
}
