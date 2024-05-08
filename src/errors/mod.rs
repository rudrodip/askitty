use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum LLMError {
    APIError(String),
    SerializationError(String),
    ReqwestError(reqwest::Error),
    IOError(io::Error),
    Other(String),
}

impl fmt::Display for LLMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLMError::APIError(msg) => write!(f, "API Error: {}", msg),
            LLMError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            LLMError::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
            LLMError::IOError(err) => write!(f, "IO Error: {}", err),
            LLMError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl Error for LLMError {}

impl From<reqwest::Error> for LLMError {
    fn from(err: reqwest::Error) -> Self {
        LLMError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for LLMError {
    fn from(err: serde_json::Error) -> Self {
        LLMError::SerializationError(err.to_string())
    }
}

impl From<io::Error> for LLMError {
    fn from(err: io::Error) -> Self {
        LLMError::IOError(err)
    }
}

#[derive(Debug)]
pub enum ImageGenError {
    APIError(String),
    DownloadError(String),
    ReqwestError(reqwest::Error),
    SerializationError(String),
    IOError(io::Error),
    Other(String),
}

impl fmt::Display for ImageGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageGenError::APIError(msg) => write!(f, "API Error: {}", msg),
            ImageGenError::DownloadError(msg) => write!(f, "Download Error: {}", msg),
            ImageGenError::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
            ImageGenError::SerializationError(msg) =>  write!(f, "Serialization Error: {}", msg),
            ImageGenError::IOError(err) => write!(f, "IO Error: {}", err),
            ImageGenError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl Error for ImageGenError {}

impl From<reqwest::Error> for ImageGenError {
    fn from(err: reqwest::Error) -> Self {
        ImageGenError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for ImageGenError {
    fn from(err: serde_json::Error) -> Self {
        ImageGenError::SerializationError(err.to_string())
    }
}

impl From<io::Error> for ImageGenError {
    fn from(err: io::Error) -> Self {
        ImageGenError::IOError(err)
    }
}