use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LLMError {
    APIError(String),
    SerializationError(String),
}

impl fmt::Display for LLMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLMError::APIError(msg) => write!(f, "API Error: {}", msg),
            LLMError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl Error for LLMError {}

#[derive(Debug)]
pub enum ImageGenError {
    APIError(String),
    DownloadError(String),
}

impl fmt::Display for ImageGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageGenError::APIError(msg) => write!(f, "API Error: {}", msg),
            ImageGenError::DownloadError(msg) => write!(f, "Download Error: {}", msg),
        }
    }
}

impl Error for ImageGenError {}
