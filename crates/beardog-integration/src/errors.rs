//! Integration-specific error types

use std::fmt;

/// Integration error type
#[derive(Debug)]
pub enum IntegrationError {
    /// Network/HTTP error
    Network(String),
    /// Configuration error
    Config(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationError::Network(msg) => write!(f, "Network error: {}", msg),
            IntegrationError::Config(msg) => write!(f, "Configuration error: {}", msg),
            IntegrationError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for IntegrationError {}

impl From<reqwest::Error> for IntegrationError {
    fn from(e: reqwest::Error) -> Self {
        IntegrationError::Network(e.to_string())
    }
}
