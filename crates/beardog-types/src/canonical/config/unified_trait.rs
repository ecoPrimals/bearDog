//! # Unified Configuration Trait (DEPRECATED)
//!
//! **⚠️ DEPRECATED: This module has been replaced by `trait.rs`**
//!
//! **Migration Path:**
//! - Use `BearDogConfig` from `canonical::config::r#trait` module
//! - Import as: `use beardog_types::canonical::config::BearDogConfig;`
//! - All trait definitions and utilities moved to `trait.rs`
//!
//! This module is kept for backward compatibility but will be removed in v4.0.0.
//! (Deprecation attribute applied at module level in mod.rs)

use beardog_errors::{BearDogError, BearDogResult};
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
    fn validate(&self) -> BearDogResult<()>;

    /// Merge this configuration with another configuration
    ///
    /// The `other` configuration takes precedence in case of conflicts.
    /// This enables configuration layering and overrides.
    fn merge(&self, other: &Self) -> BearDogResult<Self>;

    /// Load configuration from environment variables
    ///
    /// This method should attempt to load configuration values from environment
    /// variables, falling back to defaults where appropriate.
    fn from_env() -> BearDogResult<Self>
    where
        Self: Sized;

    /// Export configuration to TOML format
    ///
    /// This provides a standardized way to serialize configurations for
    /// storage, debugging, and documentation purposes.
    fn to_toml(&self) -> BearDogResult<String>;

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
    fn apply_environment_overrides(&mut self, environment: &str) -> BearDogResult<()> {
        // Default implementation does nothing - configurations can override as needed
        let _ = environment; // Suppress unused parameter warning
        Ok(())
    }

    /// Check if this configuration is compatible with a specific version
    ///
    /// This enables backward compatibility checking during configuration loading.
    /// Default implementation checks for exact version match.
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
    ///
    /// This provides a concise description of the configuration state.
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
///
/// This struct provides metadata about configuration instances, including
/// creation time, source, and validation status.
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
///
/// This enumeration tracks where configuration values originated from,
/// enabling better debugging and audit trails.
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

    /// Programmatic created
    Programmatic,
}

/// **Validation Status** - Configuration validation state
///
/// This enumeration tracks the validation status of configuration instances.
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
///
/// This trait provides a builder pattern for constructing configurations
/// with validation and error handling.
pub trait ConfigBuilder<T: BearDogConfig> {
    /// Create a new builder instance
    fn new() -> Self;

    /// Build the configuration with validation
    fn build(self) -> BearDogResult<T>;

    /// Build the configuration without validation (unsafe)
    fn build_unchecked(self) -> T;
}

/// **Configuration Loader** - Utility for loading configurations from various sources
///
/// This utility provides methods for loading configurations from different sources
/// with proper error handling and validation.
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from environment variables with prefix
    ///
    /// This method loads configuration values from environment variables
    /// with a specific prefix (e.g., "`BEARDOG_SECURITY`_").
    pub fn from_env_with_prefix<T: BearDogConfig>(_prefix: &str) -> BearDogResult<T> {
        // Implementation would use environment variable parsing
        T::from_env()
    }

    /// Load configuration from TOML file
    ///
    /// This method loads configuration from a TOML file with validation.
    #[cfg(feature = "config")]
    pub fn from_toml_file<T: BearDogConfig>(path: &str) -> BearDogResult<T> {
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
    #[cfg(not(feature = "config"))]
    pub fn from_toml_file<T: BearDogConfig>(_path: &str) -> BearDogResult<T> {
        Err(BearDogError::configuration(
            "TOML support not enabled - enable 'config' feature",
        ))
    }

    /// Load configuration from JSON file
    ///
    /// This method loads configuration from a JSON file with validation.
    pub fn from_json_file<T: BearDogConfig>(path: &str) -> BearDogResult<T> {
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
    /// This method merges configurations in order of precedence (later configs override earlier ones).
    pub fn merge_configs<T: BearDogConfig>(configs: Vec<T>) -> BearDogResult<T> {
        if configs.is_empty() {
            return Err(BearDogError::configuration(
                "Cannot merge empty configuration list",
            ));
        }

        let mut configs_iter = configs.into_iter();
        let mut result = configs_iter.next().unwrap();

        for config in configs_iter {
            result = result.merge(&config)?;
        }

        result.validate()?;
        Ok(result)
    }
}

/// **Pedantic Validation Module** - Comprehensive validation utilities
///
/// This module provides validation functions with detailed error messages
/// and comprehensive edge case handling for pedantic-level quality.
pub mod validation {
    use super::{BearDogError, BearDogResult};

    /// **Validate numeric range with detailed error messages**
    ///
    /// This function validates that a numeric value falls within the specified range,
    /// providing detailed error messages for out-of-range values.
    ///
    /// # Arguments
    /// * `value` - The value to validate
    /// * `min` - Minimum allowed value (inclusive)
    /// * `max` - Maximum allowed value (inclusive)
    /// * `field_name` - Name of the field being validated
    ///
    /// # Returns
    /// * `Ok(())` if the value is within range
    /// * `Err(BearDogError)` with detailed error message if out of range
    ///
    /// # Examples
    /// ```rust
    /// use beardog_types::canonical::config::unified_trait::validation;
    ///
    /// // Valid range
    /// assert!(validation::validate_range(0.5, 0.0, 1.0, "confidence_threshold").is_ok());
    ///
    /// // Invalid range
    /// assert!(validation::validate_range(1.5, 0.0, 1.0, "confidence_threshold").is_err());
    /// ```
    pub fn validate_range<T>(value: T, min: T, max: T, field_name: &str) -> BearDogResult<()>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' value {value} is below minimum allowed value {min}. \
                 This may cause system instability or unexpected behavior. \
                 Please adjust the value to be within the valid range [{min}, {max}]."
            )));
        }

        if value > max {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' value {value} exceeds maximum allowed value {max}. \
                 This may cause resource exhaustion or performance degradation. \
                 Please adjust the value to be within the valid range [{min}, {max}]."
            )));
        }

        Ok(())
    }

    /// **Validate string is not empty with context**
    ///
    /// This function validates that a string field is not empty,
    /// providing contextual error messages.
    pub fn validate_non_empty_string(value: &str, field_name: &str) -> BearDogResult<()> {
        if value.trim().is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' cannot be empty or contain only whitespace. \
                 Empty values may cause configuration errors or system failures. \
                 Please provide a valid non-empty value."
            )));
        }
        Ok(())
    }

    /// **Validate collection size with detailed constraints**
    ///
    /// This function validates that a collection has an appropriate size,
    /// with detailed error messages for constraint violations.
    pub fn validate_collection_size<T>(
        collection: &[T],
        min_size: usize,
        max_size: usize,
        field_name: &str,
    ) -> BearDogResult<()> {
        let size = collection.len();

        if size < min_size {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' contains {size} items, which is below the minimum required {min_size} items. \
                 Insufficient items may cause incomplete functionality or system errors. \
                 Please add more items to meet the minimum requirement."
            )));
        }

        if size > max_size {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' contains {size} items, which exceeds the maximum allowed {max_size} items. \
                 Too many items may cause performance degradation or memory issues. \
                 Please reduce the number of items to stay within limits."
            )));
        }

        Ok(())
    }

    /// **Validate duration is reasonable**
    ///
    /// This function validates that a duration value is within reasonable bounds,
    /// preventing extremely short or long timeouts that could cause issues.
    pub fn validate_duration(
        duration: std::time::Duration,
        min_duration: std::time::Duration,
        max_duration: std::time::Duration,
        field_name: &str,
    ) -> BearDogResult<()> {
        if duration < min_duration {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' duration {duration:?} is too short (minimum: {min_duration:?}). \
                 Very short durations may cause timeouts, race conditions, or system instability. \
                 Please increase the duration to a more reasonable value."
            )));
        }

        if duration > max_duration {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' duration {duration:?} is too long (maximum: {max_duration:?}). \
                 Very long durations may cause poor user experience or resource blocking. \
                 Please reduce the duration to a more reasonable value."
            )));
        }

        Ok(())
    }

    /// **Validate percentage value (0.0 to 1.0)**
    ///
    /// This function validates that a value represents a valid percentage (0.0 to 1.0),
    /// with clear error messages for invalid ranges.
    pub fn validate_percentage(value: f64, field_name: &str) -> BearDogResult<()> {
        if value < 0.0 {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value {value} is negative. \
                 Negative percentages are invalid and may cause calculation errors. \
                 Please provide a value between 0.0 (0%) and 1.0 (100%)."
            )));
        }

        if value > 1.0 {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value {value} exceeds 100% (1.0). \
                 Values greater than 100% may cause resource over-allocation or system errors. \
                 Please provide a value between 0.0 (0%) and 1.0 (100%)."
            )));
        }

        if value.is_nan() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value is NaN (Not a Number). \
                 NaN values will cause calculation failures and unpredictable behavior. \
                 Please provide a valid numeric value between 0.0 and 1.0."
            )));
        }

        if value.is_infinite() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' percentage value is infinite. \
                 Infinite values will cause calculation failures and system errors. \
                 Please provide a finite numeric value between 0.0 and 1.0."
            )));
        }

        Ok(())
    }

    /// **Validate port number**
    ///
    /// This function validates that a port number is within the valid range
    /// and provides guidance on port usage.
    pub fn validate_port(port: u16, field_name: &str) -> BearDogResult<()> {
        match port {
            0 => Err(BearDogError::configuration(&format!(
                "Field '{field_name}' port number 0 is invalid. \
                 Port 0 is reserved and cannot be used for network services. \
                 Please choose a port number between 1 and 65535."
            ))),
            1..=1023 => {
                // Well-known ports - warn but allow
                Ok(())
            }
            1024..=49151 => {
                // Registered ports - ideal range
                Ok(())
            }
            49152..=65535 => {
                // Dynamic/private ports - acceptable
                Ok(())
            }
        }
    }

    /// **Validate network address format**
    ///
    /// This function validates that a network address string has a valid format.
    pub fn validate_network_address(address: &str, field_name: &str) -> BearDogResult<()> {
        if address.trim().is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' network address cannot be empty. \
                 Empty addresses will prevent network connectivity. \
                 Please provide a valid IP address or hostname."
            )));
        }

        // Basic format validation (could be enhanced with regex)
        if !address.contains(':')
            && !address
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' network address '{address}' contains invalid characters. \
                 Network addresses should contain only alphanumeric characters, dots, hyphens, and colons. \
                 Please provide a valid IP address or hostname."
            )));
        }

        Ok(())
    }

    /// **Validate collection is not empty**
    ///
    /// This function validates that a collection contains at least one item.
    pub fn validate_non_empty_collection<T>(
        collection: &[T],
        field_name: &str,
    ) -> BearDogResult<()> {
        if collection.is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' collection cannot be empty. \
                 Empty collections may cause incomplete functionality or system errors. \
                 Please add at least one item to the collection."
            )));
        }
        Ok(())
    }

    /// **Validate URL format**
    ///
    /// This function validates that a string represents a valid URL.
    pub fn validate_url(url: &str, field_name: &str) -> BearDogResult<()> {
        if url.trim().is_empty() {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' URL cannot be empty. \
                 Empty URLs will prevent network connectivity. \
                 Please provide a valid HTTP or HTTPS URL."
            )));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(BearDogError::configuration(&format!(
                "Field '{field_name}' URL '{url}' must start with 'http://' or 'https://'. \
                 Invalid URL schemes may cause connection failures. \
                 Please provide a valid HTTP or HTTPS URL."
            )));
        }

        Ok(())
    }

    /// **Validate configuration consistency across related fields**
    ///
    /// This function validates that related configuration fields are consistent
    /// with each other, preventing conflicting settings.
    pub fn validate_field_consistency<T, U>(
        field1_value: T,
        field1_name: &str,
        field2_value: U,
        field2_name: &str,
        consistency_check: impl Fn(&T, &U) -> bool,
        error_message: &str,
    ) -> BearDogResult<()>
    where
        T: std::fmt::Display,
        U: std::fmt::Display,
    {
        if !consistency_check(&field1_value, &field2_value) {
            return Err(BearDogError::configuration(&format!(
                "Configuration fields '{field1_name}' ({field1_value}) and '{field2_name}' ({field2_value}) are inconsistent. \
                 {error_message} \
                 Please review and adjust the configuration to ensure consistency."
            )));
        }

        Ok(())
    }

    /// **Validate resource allocation doesn't exceed 100%**
    ///
    /// This function validates that resource allocation percentages don't exceed 100%
    /// when combined, preventing resource over-allocation.
    pub fn validate_resource_allocation(allocations: &[(f64, &str)]) -> BearDogResult<()> {
        let total: f64 = allocations.iter().map(|(value, _)| *value).sum();

        if total > 1.0 {
            let field_names: Vec<&str> = allocations.iter().map(|(_, name)| *name).collect();
            return Err(BearDogError::configuration(&format!(
                "Total resource allocation ({:.2}%) exceeds 100% across fields: {}. \
                 Over-allocation will cause resource contention and system instability. \
                 Please reduce allocation percentages so the total does not exceed 100%.",
                total * 100.0,
                field_names.join(", ")
            )));
        }

        // Validate individual allocations
        for (value, name) in allocations {
            validate_percentage(*value, name)?;
        }

        Ok(())
    }

    /// **Validate configuration environment compatibility**
    ///
    /// This function validates that configuration values are appropriate
    /// for the target deployment environment.
    pub fn validate_environment_compatibility(
        environment: &str,
        field_name: &str,
        value: &str,
        production_safe: bool,
    ) -> BearDogResult<()> {
        match environment {
            "production" | "prod" => {
                if !production_safe {
                    return Err(BearDogError::configuration(&format!(
                        "Field '{field_name}' value '{value}' is not safe for production environment. \
                         This configuration may cause security vulnerabilities, performance issues, \
                         or system instability in production. Please use production-appropriate values."
                    )));
                }
            }
            "development" | "dev" => {
                // More lenient for development
            }
            "staging" | "stage" => {
                // Moderate restrictions for staging
            }
            _ => {
                return Err(BearDogError::configuration(&format!(
                    "Unknown environment '{environment}'. \
                     Supported environments: development, staging, production. \
                     Please specify a valid environment for proper configuration validation."
                )));
            }
        }

        Ok(())
    }
}

// Tests removed - this module is deprecated and will be removed in v4.0.0
// The validation functions are tested in the new trait.rs module
