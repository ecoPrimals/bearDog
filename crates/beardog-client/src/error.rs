//! Error types for BearDog client

use thiserror::Error;

/// Result type for BearDog client operations
pub type ClientResult<T> = Result<T, BearDogClientError>;

/// BearDog client errors
#[derive(Debug, Error)]
pub enum BearDogClientError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API returned an error
    #[error("API error: {0}")]
    ApiError(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Invalid response from API
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Timeout
    #[error("Request timeout")]
    Timeout,

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

