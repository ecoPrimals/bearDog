// SPDX-License-Identifier: AGPL-3.0-only

//! # Unified Configuration Trait System
//!
//! This module defines the unified `BearDogConfig` trait and comprehensive validation
//! utilities for all configuration types across the BearDog ecosystem.
//!
//! ## Architecture
//!
//! - **BearDogConfig trait**: Standard interface for all configurations
//! - **Metadata types**: ConfigMetadata, ConfigSource, ValidationStatus
//! - **ConfigLoader**: Utility for loading configurations from various sources
//! - **validation module**: Pedantic-level validation utilities

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// **UNIFIED CONFIGURATION TRAIT** - Common interface for all `BearDog` configurations
///
/// This trait provides a consistent interface that all configuration structs must implement,
/// ensuring uniform behavior across the entire ecosystem.
pub trait BearDogConfig:
    Send + Sync + Clone + Debug + Serialize + for<'de> Deserialize<'de>
{
    /// Validate the configuration
    ///
    /// This method should perform comprehensive validation of all configuration
    /// fields and return appropriate errors for invalid configurations.
    ///
    /// # Errors
    ///
    /// Returns an error if any field fails validation.
    fn validate(&self) -> Result<(), BearDogError>;

    /// Merge this configuration with another configuration
    ///
    /// The `other` configuration takes precedence in case of conflicts.
    /// This enables configuration layering and overrides.
    ///
    /// # Errors
    ///
    /// Returns an error if merging or deserializing the merged result fails.
    fn merge(&self, other: &Self) -> Result<Self, BearDogError>;

    /// Load configuration from environment variables
    ///
    /// This method should attempt to load configuration values from environment
    /// variables, falling back to defaults where appropriate.
    ///
    /// # Errors
    ///
    /// Returns an error if environment loading or validation fails.
    fn from_env() -> Result<Self, BearDogError>
    where
        Self: Sized;

    /// Export configuration to TOML format
    ///
    /// This provides a standardized way to serialize configurations for
    /// storage, debugging, and documentation purposes.
    ///
    /// # Errors
    ///
    /// Returns an error if TOML serialization fails or the `config` feature is disabled.
    fn to_toml(&self) -> Result<String, BearDogError>;

    /// Get the configuration domain name
    ///
    /// This returns a static string identifying the configuration domain
    /// (e.g., "adapter", "security", "monitoring").
    fn domain() -> &'static str
    where
        Self: Sized;

    /// Get configuration version for migration support
    ///
    /// This enables configuration versioning and migration between versions.
    fn version() -> u32
    where
        Self: Sized,
    {
        1
    }

    /// Apply environment-specific overrides
    ///
    /// This method allows configurations to be modified based on the deployment
    /// environment (development, staging, production).
    ///
    /// # Errors
    ///
    /// Returns an error if overrides cannot be applied; the default implementation never fails.
    fn apply_environment_overrides(&mut self, environment: &str) -> Result<(), BearDogError> {
        let _ = environment; // Suppress unused parameter warning
        Ok(())
    }

    /// Check if this configuration is compatible with a specific version
    ///
    /// This enables backward compatibility checking during configuration loading.
    fn is_compatible_with(&self, version: u32) -> bool {
        Self::version() == version
    }

    /// Get configuration metadata
    ///
    /// This provides additional information about the configuration instance.
    fn metadata(&self) -> ConfigMetadata {
        ConfigMetadata {
            domain: Self::domain().to_string(),
            version: Self::version(),
            created_at: std::time::SystemTime::now(),
            source: ConfigSource::Default,
            validation_status: ValidationStatus::Unknown,
        }
    }

    /// Get a human-readable summary of the configuration
    fn summary(&self) -> String {
        format!(
            "{}Config(domain={}, version={})",
            Self::domain(),
            Self::domain(),
            Self::version()
        )
    }
}

/// **Configuration Metadata** - Additional information about configuration instances
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigMetadata {
    /// Configuration domain name
    pub domain: String,

    /// Configuration version
    pub version: u32,

    /// Creation timestamp
    pub created_at: std::time::SystemTime,

    /// Configuration source
    pub source: ConfigSource,

    /// Validation status
    pub validation_status: ValidationStatus,
}

/// **Configuration Source** - Origin of configuration values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigSource {
    /// Default values from code
    Default,

    /// Loaded from environment variables
    Environment,

    /// Loaded from configuration file
    File(String),

    /// Merged from multiple sources
    Merged,

    /// Programmatically created
    Programmatic,
}

/// **Validation Status** - Configuration validation state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationStatus {
    /// Validation status unknown
    Unknown,

    /// Configuration is valid
    Valid,

    /// Configuration has validation errors
    Invalid(Vec<String>),

    /// Configuration has warnings but is usable
    Warning(Vec<String>),
}

/// **Configuration Builder Trait** - Fluent configuration construction
pub trait ConfigBuilder<T: BearDogConfig> {
    /// Create a new builder instance
    fn new() -> Self;

    /// Build the configuration with validation
    ///
    /// # Errors
    ///
    /// Returns an error if building or validation fails.
    fn build(self) -> Result<T, BearDogError>;

    /// Build the configuration without validation (unchecked)
    fn build_unchecked(self) -> T;
}

/// **Configuration Loader** - Utility for loading configurations from various sources
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from environment variables with prefix
    ///
    /// # Errors
    ///
    /// Returns an error if [`BearDogConfig::from_env`] fails.
    pub fn from_env_with_prefix<T: BearDogConfig>(_prefix: &str) -> Result<T, BearDogError> {
        T::from_env()
    }

    /// Load configuration from TOML file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read, TOML parsing fails, or validation fails.
    #[cfg(feature = "config")]
    pub fn from_toml_file<T: BearDogConfig>(path: &str) -> Result<T, BearDogError> {
        use std::fs;

        let content = fs::read_to_string(path).map_err(|e| {
            BearDogError::configuration(&format!("Failed to read config file {path}: {e}"))
        })?;

        let config: T = toml::from_str(&content).map_err(|e| {
            BearDogError::configuration(&format!("Failed to parse TOML config: {e}"))
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Load configuration from TOML file (feature-gated fallback)
    ///
    /// # Errors
    ///
    /// Always returns an error because the `config` feature is disabled.
    #[cfg(not(feature = "config"))]
    pub fn from_toml_file<T: BearDogConfig>(_path: &str) -> Result<T, BearDogError> {
        Err(BearDogError::configuration(
            "TOML support not enabled - enable 'config' feature",
        ))
    }

    /// Load configuration from JSON file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read, JSON parsing fails, or validation fails.
    pub fn from_json_file<T: BearDogConfig>(path: &str) -> Result<T, BearDogError> {
        use std::fs;

        let content = fs::read_to_string(path).map_err(|e| {
            BearDogError::configuration(&format!("Failed to read config file {path}: {e}"))
        })?;

        let config: T = serde_json::from_str(&content).map_err(|e| {
            BearDogError::configuration(&format!("Failed to parse JSON config: {e}"))
        })?;

        config.validate()?;
        Ok(config)
    }

    /// Merge multiple configurations with precedence
    ///
    /// # Errors
    ///
    /// Returns an error if the list is empty, merge fails, or final validation fails.
    pub fn merge_configs<T: BearDogConfig>(configs: Vec<T>) -> Result<T, BearDogError> {
        if configs.is_empty() {
            return Err(BearDogError::configuration(
                "Cannot merge empty configuration list",
            ));
        }

        let mut configs_iter = configs.into_iter();
        let mut result = configs_iter
            .next()
            .ok_or_else(|| BearDogError::configuration("No configurations provided to merge"))?;

        for config in configs_iter {
            result = result.merge(&config)?;
        }

        result.validate()?;
        Ok(result)
    }
}

/// **Pedantic Validation Module** - Comprehensive validation utilities
pub mod validation {
    use super::BearDogError;

    /// Validate numeric range with detailed error messages
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is outside `[min, max]`.
    pub fn validate_range<T>(value: T, min: T, max: T, field_name: &str) -> Result<(), BearDogError>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' value {value} is below minimum allowed value {min}. \
                 Please adjust the value to be within the valid range [{min}, {max}]."
            )));
        }

        if value > max {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' value {value} exceeds maximum allowed value {max}. \
                 Please adjust the value to be within the valid range [{min}, {max}]."
            )));
        }

        Ok(())
    }

    /// Validate string is not empty with context
    ///
    /// # Errors
    ///
    /// Returns an error if `value` is empty or whitespace-only.
    pub fn validate_non_empty_string(value: &str, field_name: &str) -> Result<(), BearDogError> {
        if value.trim().is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' cannot be empty or contain only whitespace. \
                 Please provide a valid non-empty value."
            )));
        }
        Ok(())
    }

    /// Validate collection size with detailed constraints
    ///
    /// # Errors
    ///
    /// Returns an error if the collection length is outside `[min_size, max_size]`.
    pub fn validate_collection_size<T>(
        collection: &[T],
        min_size: usize,
        max_size: usize,
        field_name: &str,
    ) -> Result<(), BearDogError> {
        let size = collection.len();

        if size < min_size {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' contains {size} items, below minimum {min_size}. \
                 Please add more items to meet the minimum requirement."
            )));
        }

        if size > max_size {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' contains {size} items, exceeding maximum {max_size}. \
                 Please reduce the number of items to stay within limits."
            )));
        }

        Ok(())
    }

    /// Validate duration is reasonable
    ///
    /// # Errors
    ///
    /// Returns an error if `duration` is shorter than `min_duration` or longer than `max_duration`.
    pub fn validate_duration(
        duration: std::time::Duration,
        min_duration: std::time::Duration,
        max_duration: std::time::Duration,
        field_name: &str,
    ) -> Result<(), BearDogError> {
        if duration < min_duration {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' duration {duration:?} is too short (minimum: {min_duration:?}). \
                 Please increase the duration to a more reasonable value."
            )));
        }

        if duration > max_duration {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' duration {duration:?} is too long (maximum: {max_duration:?}). \
                 Please reduce the duration to a more reasonable value."
            )));
        }

        Ok(())
    }

    /// Validate percentage value (0.0 to 1.0)
    ///
    /// # Errors
    ///
    /// Returns an error if the value is out of range, NaN, or infinite.
    pub fn validate_percentage(value: f64, field_name: &str) -> Result<(), BearDogError> {
        if value < 0.0 {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value {value} is negative. \
                 Please provide a value between 0.0 (0%) and 1.0 (100%)."
            )));
        }

        if value > 1.0 {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value {value} exceeds 100% (1.0). \
                 Please provide a value between 0.0 (0%) and 1.0 (100%)."
            )));
        }

        if value.is_nan() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value is NaN. \
                 Please provide a valid numeric value between 0.0 and 1.0."
            )));
        }

        if value.is_infinite() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value is infinite. \
                 Please provide a finite numeric value between 0.0 and 1.0."
            )));
        }

        Ok(())
    }

    /// Validate port number
    ///
    /// # Errors
    ///
    /// Returns an error if `port` is zero.
    pub fn validate_port(port: u16, field_name: &str) -> Result<(), BearDogError> {
        if port == 0 {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' port number 0 is invalid. \
                 Please choose a port number between 1 and 65535."
            )));
        }
        Ok(())
    }

    /// Validate network address format
    ///
    /// # Errors
    ///
    /// Returns an error if the address is empty or contains invalid characters.
    pub fn validate_network_address(address: &str, field_name: &str) -> Result<(), BearDogError> {
        if address.trim().is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' network address cannot be empty. \
                 Please provide a valid IP address or hostname."
            )));
        }

        if !address.contains(':')
            && !address
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' network address '{address}' contains invalid characters. \
                 Please provide a valid IP address or hostname."
            )));
        }

        Ok(())
    }

    /// Validate collection is not empty
    ///
    /// # Errors
    ///
    /// Returns an error if `collection` is empty.
    pub fn validate_non_empty_collection<T>(
        collection: &[T],
        field_name: &str,
    ) -> Result<(), BearDogError> {
        if collection.is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' collection cannot be empty. \
                 Please add at least one item to the collection."
            )));
        }
        Ok(())
    }

    /// Validate URL format
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is empty or does not start with `http://` or `https://`.
    pub fn validate_url(url: &str, field_name: &str) -> Result<(), BearDogError> {
        if url.trim().is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' URL cannot be empty. \
                 Please provide a valid HTTP or HTTPS URL."
            )));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' URL '{url}' must start with 'http://' or 'https://'. \
                 Please provide a valid HTTP or HTTPS URL."
            )));
        }

        Ok(())
    }

    /// Validate configuration consistency across related fields
    ///
    /// # Errors
    ///
    /// Returns an error if the consistency predicate fails.
    pub fn validate_field_consistency<T, U>(
        field1_value: T,
        field1_name: &str,
        field2_value: U,
        field2_name: &str,
        consistency_check: impl Fn(&T, &U) -> bool,
        error_message: &str,
    ) -> Result<(), BearDogError>
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        if !consistency_check(&field1_value, &field2_value) {
            return Err(BearDogError::configuration(&format!(
                "Configuration fields '{field1_name}' ({field1_value}) and '{field2_name}' ({field2_value}) are inconsistent. \
                 {error_message}"
            )));
        }

        Ok(())
    }

    /// Validate resource allocation doesn't exceed 100%
    ///
    /// # Errors
    ///
    /// Returns an error if total allocation exceeds 100% or any entry fails percentage validation.
    pub fn validate_resource_allocation(allocations: &[(f64, &str)]) -> Result<(), BearDogError> {
        let total: f64 = allocations.iter().map(|(value, _)| *value).sum();

        if total > 1.0 {
            let field_names: Vec<&str> = allocations.iter().map(|(_, name)| *name).collect();
            return Err(BearDogError::configuration(&format!(
                "Total resource allocation ({:.2}%) exceeds 100% across fields: {}. \
                 Please reduce allocation percentages so the total does not exceed 100%.",
                total * 100.0,
                field_names.join(", ")
            )));
        }

        for (value, name) in allocations {
            validate_percentage(*value, name)?;
        }

        Ok(())
    }

    /// Validate configuration environment compatibility
    ///
    /// # Errors
    ///
    /// Returns an error if the environment is unknown, production safety is violated, or validation fails.
    pub fn validate_environment_compatibility(
        environment: &str,
        field_name: &str,
        value: &str,
        production_safe: bool,
    ) -> Result<(), BearDogError> {
        match environment {
            "production" | "prod" => {
                if !production_safe {
                    return Err(BearDogError::configuration(&format!(
                        "Field '{field_name}' value '{value}' is not safe for production environment. \
                         Please use production-appropriate values."
                    )));
                }
            }
            "development" | "dev" | "staging" | "stage" => {
                // More lenient for non-production
            }
            _ => {
                return Err(BearDogError::configuration(&format!(
                    "Unknown environment '{environment}'. \
                     Supported: development, staging, production."
                )));
            }
        }

        Ok(())
    }
}
