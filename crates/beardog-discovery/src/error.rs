//! Error types for discovery

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DiscoveryError>;

#[derive(Error, Debug)]
pub enum DiscoveryError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Discovery failed: {0}")]
    DiscoveryFailed(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Capability not found: {0}")]
    CapabilityNotFound(String),

    #[error("Announcement failed: {0}")]
    AnnouncementFailed(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid endpoint: {0}")]
    InvalidEndpoint(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<toml::de::Error> for DiscoveryError {
    fn from(err: toml::de::Error) -> Self {
        DiscoveryError::Config(err.to_string())
    }
}

impl From<reqwest::Error> for DiscoveryError {
    fn from(err: reqwest::Error) -> Self {
        DiscoveryError::Network(err.to_string())
    }
}

impl From<url::ParseError> for DiscoveryError {
    fn from(err: url::ParseError) -> Self {
        DiscoveryError::Parse(err.to_string())
    }
}

