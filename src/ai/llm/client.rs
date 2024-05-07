use crate::ai::llm::traits::LLM;
use crate::errors::LLMError;
use crate::types::llm::{Completion, Message};
use reqwest::Client as HttpClient;
use std::env::var;
use tokio::runtime::Runtime;

pub struct Client {
    pub host: String,
    pub model: String,
    pub api_key: String,
}

impl LLM for Client {
    fn new() -> Result<Self, &'static str> {
        let host = var("LLM_HOST").unwrap();
        let model = var("LLM_MODEL").unwrap();
        let api_key = var("LLM_API_KEY").unwrap();
        Ok(Client {
            host,
            model,
            api_key,
        })
    }
    fn completion(&self, chats: Vec<Message>) -> Result<String, LLMError> {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let data = format!(
                "{{\"model\": \"{}\",\"messages\": {}}}",
                self.model,
                serde_json::to_string(&chats).unwrap()
            );
            let client = HttpClient::new();
            let res = client
                .post(&format!("{}", format!("{}/chat/completions", self.host)))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .body(data)
                .send()
                .await;
            match res {
                Ok(res) => {
                    let body = res.text().await;
                    match serde_json::from_str::<Completion>(&body.unwrap()) {
                        Ok(output) => {
                            let response = output.choices[0].message.content.clone();
                            Ok(response)
                        }
                        Err(e) => Err(LLMError::SerializationError(e.to_string())),
                    }
                }
                Err(e) => Err(LLMError::APIError(e.to_string())),
            }
        })
    }
}
