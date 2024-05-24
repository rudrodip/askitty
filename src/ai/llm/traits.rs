use crate::errors::LLMError;
use crate::types::llm::*;
use std::future::Future;

pub trait LLM {
    fn new(host: String, model: String, api_key: String) -> Result<Self, LLMError>
    where
        Self: Sized;
    fn completion(
        &self,
        chats: Vec<Message>,
    ) -> impl Future<Output = Result<String, LLMError>> + Send;
}
