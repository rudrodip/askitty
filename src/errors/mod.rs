use std::{error::Error, fmt, io};
use bincode::Error as BincodeError;

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
            ImageGenError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
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

#[derive(Debug)]
pub enum StorageError {
    IOError(io::Error),
    SerializationError(String),
    SledError(sled::Error),
    BincodeError(BincodeError),
    Other(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::IOError(err) => write!(f, "IO Error: {}", err),
            StorageError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            StorageError::SledError(err) => write!(f, "Sled Error: {}", err),
            StorageError::BincodeError(err) => write!(f, "Bincode Error: {}", err),
            StorageError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl Error for StorageError {}

impl From<io::Error> for StorageError {
    fn from(err: io::Error) -> Self {
        StorageError::IOError(err)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}

impl From<sled::Error> for StorageError {
    fn from(err: sled::Error) -> Self {
        StorageError::SledError(err)
    }
}

impl From<BincodeError> for StorageError {
    fn from(err: BincodeError) -> Self {
        StorageError::BincodeError(err)
    }
}