//! Error types for configuration system

use std::io;
use thiserror::Error;

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Configuration error types
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Configuration validation error
    #[error("Configuration validation failed: {0}")]
    Validation(String),

    /// File I/O error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// TOML parsing error
    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// TOML serialization error
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    /// Generic serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvVar(String),

    /// Configuration file not found
    #[error("Configuration file not found: {0}")]
    FileNotFound(String),

    /// Invalid configuration format
    #[error("Invalid configuration format: {0}")]
    InvalidFormat(String),

    /// Port conflict error
    #[error("Port conflict: {0}")]
    PortConflict(String),

    /// Path does not exist
    #[error("Path does not exist: {0}")]
    PathNotFound(String),

    /// Invalid value
    #[error("Invalid value for '{field}': {message}")]
    InvalidValue {
        field: String,
        message: String,
    },
}

impl ConfigError {
    /// Create a validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::Validation(message.into())
    }

    /// Create an invalid value error
    pub fn invalid_value<S1: Into<String>, S2: Into<String>>(field: S1, message: S2) -> Self {
        Self::InvalidValue {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a port conflict error
    pub fn port_conflict<S: Into<String>>(message: S) -> Self {
        Self::PortConflict(message.into())
    }
}

