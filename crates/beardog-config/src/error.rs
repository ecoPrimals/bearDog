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
    InvalidValue { field: String, message: String },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let error = ConfigError::validation("Invalid config");
        assert!(matches!(error, ConfigError::Validation(_)));
        assert_eq!(
            error.to_string(),
            "Configuration validation failed: Invalid config"
        );
    }

    #[test]
    fn test_invalid_value_error() {
        let error = ConfigError::invalid_value("port", "must be > 0");
        assert!(matches!(error, ConfigError::InvalidValue { .. }));
        assert!(error.to_string().contains("port"));
        assert!(error.to_string().contains("must be > 0"));
    }

    #[test]
    fn test_port_conflict_error() {
        let error = ConfigError::port_conflict("Port 8080 already in use");
        assert!(matches!(error, ConfigError::PortConflict(_)));
        assert!(error.to_string().contains("8080"));
    }

    #[test]
    fn test_file_not_found_error() {
        let error = ConfigError::FileNotFound("/path/to/config.toml".to_string());
        assert!(matches!(error, ConfigError::FileNotFound(_)));
        assert!(error.to_string().contains("/path/to/config.toml"));
    }

    #[test]
    fn test_invalid_format_error() {
        let error = ConfigError::InvalidFormat("Expected TOML".to_string());
        assert!(matches!(error, ConfigError::InvalidFormat(_)));
        assert!(error.to_string().contains("TOML"));
    }

    #[test]
    fn test_env_var_error() {
        let error = ConfigError::EnvVar("BEARDOG_PORT not set".to_string());
        assert!(matches!(error, ConfigError::EnvVar(_)));
        assert!(error.to_string().contains("BEARDOG_PORT"));
    }

    #[test]
    fn test_path_not_found_error() {
        let error = ConfigError::PathNotFound("/nonexistent/path".to_string());
        assert!(matches!(error, ConfigError::PathNotFound(_)));
        assert!(error.to_string().contains("/nonexistent/path"));
    }

    #[test]
    fn test_serialization_error() {
        let error = ConfigError::Serialization("Failed to serialize".to_string());
        assert!(matches!(error, ConfigError::Serialization(_)));
        assert!(error.to_string().contains("serialize"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let config_error: ConfigError = io_error.into();
        assert!(matches!(config_error, ConfigError::Io(_)));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_str = "{invalid json}";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let config_error: ConfigError = json_error.into();
        assert!(matches!(config_error, ConfigError::JsonParse(_)));
    }

    #[test]
    fn test_config_result_ok() {
        let result: ConfigResult<i32> = Ok(42);
        assert!(result.is_ok());
        if let Ok(val) = result {
            assert_eq!(val, 42);
        }
    }

    #[test]
    fn test_config_result_err() {
        let result: ConfigResult<i32> = Err(ConfigError::validation("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_debug_format() {
        let error = ConfigError::validation("test");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Validation"));
    }

    #[test]
    fn test_validation_error_with_empty_string() {
        let error = ConfigError::validation("");
        assert!(matches!(error, ConfigError::Validation(_)));
    }

    #[test]
    fn test_invalid_value_with_long_message() {
        let long_msg = "a".repeat(1000);
        let error = ConfigError::invalid_value("field", &long_msg);
        assert!(matches!(error, ConfigError::InvalidValue { .. }));
    }
}
