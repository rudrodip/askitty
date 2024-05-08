use crate::errors::LLMError;
use crate::types::llm::*;
use std::future::Future;

pub trait LLM {
    fn new() -> Result<Self, &'static str>
    where
        Self: Sized;
    fn completion(
        &self,
        chats: Vec<Message>,
    ) -> impl Future<Output = Result<String, LLMError>> + Send;
}
