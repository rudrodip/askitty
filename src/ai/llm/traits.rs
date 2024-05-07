use crate::types::llm::*;
use crate::errors::LLMError;

pub trait LLM {
  fn new() -> Result<Self, &'static str> where Self: Sized;
  fn completion(&self, chats: Vec<Message>) -> Result<String, LLMError>;
}