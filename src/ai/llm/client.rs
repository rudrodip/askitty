use crate::ai::llm::traits::LLM;
use crate::errors::LLMError;
use crate::types::llm::{Completion, Message};
use reqwest::Client as HttpClient;

pub struct Client {
    pub host: String,
    pub model: String,
    pub api_key: String,
}

impl LLM for Client {
    fn new(host: String, model: String, api_key: String) -> Result<Self, LLMError> {
        Ok(Client {
            host,
            model,
            api_key,
        })
    }

    async fn completion(&self, chats: Vec<Message>) -> Result<String, LLMError> {
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
            .await?;
        let body = res.text().await?;
        let completion: Completion = serde_json::from_str(&body)?;
        let message = completion.choices[0].message.content.to_owned();
        Ok(message)
    }
}
