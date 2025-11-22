//! Concurrent-Safe Limits Configuration Module
//!
//! Limits and timeout configuration for BearDog operations.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};

/// Limits and timeouts configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimitsConfig {
    /// Operation timeout in seconds
    pub operation_timeout_secs: u64,

    /// Connection timeout in seconds
    pub connection_timeout_secs: u64,

    /// Maximum retry attempts
    pub max_retries: u32,

    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,

    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,

    /// Request rate limit (requests per second, 0 = unlimited)
    pub rate_limit_per_sec: u32,

    /// Maximum request body size in bytes
    pub max_request_body_bytes: usize,

    /// Cache size (entries)
    pub cache_size: usize,

    /// Cache TTL in seconds
    pub cache_ttl_secs: u64,
}

impl LimitsConfig {
    /// Pure static defaults (no environment variable reads)
    pub const fn const_defaults() -> Self {
        Self {
            operation_timeout_secs: 30,
            connection_timeout_secs: 10,
            max_retries: 3,
            retry_delay_ms: 1000,
            max_concurrent_operations: 100,
            rate_limit_per_sec: 0,              // Unlimited
            max_request_body_bytes: 10_485_760, // 10 MB
            cache_size: 1000,
            cache_ttl_secs: 300, // 5 minutes
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            operation_timeout_secs: std::env::var("BEARDOG_OPERATION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.operation_timeout_secs),

            connection_timeout_secs: std::env::var("BEARDOG_CONNECTION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.connection_timeout_secs),

            max_retries: std::env::var("BEARDOG_MAX_RETRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.max_retries),

            retry_delay_ms: std::env::var("BEARDOG_RETRY_DELAY_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.retry_delay_ms),

            max_concurrent_operations: std::env::var("BEARDOG_MAX_CONCURRENT_OPERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.max_concurrent_operations),

            rate_limit_per_sec: defaults.rate_limit_per_sec,

            max_request_body_bytes: std::env::var("BEARDOG_MAX_REQUEST_BODY_BYTES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.max_request_body_bytes),

            cache_size: std::env::var("BEARDOG_CACHE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.cache_size),

            cache_ttl_secs: std::env::var("BEARDOG_CACHE_TTL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.cache_ttl_secs),
        }
    }

    /// Create a builder for flexible configuration construction
    pub fn builder() -> LimitsConfigBuilder {
        LimitsConfigBuilder::new()
    }

    /// Validate limits configuration
    pub fn validate(&self) -> ConfigResult<()> {
        if self.operation_timeout_secs == 0 {
            return Err(ConfigError::invalid_value(
                "limits.operation_timeout_secs",
                "Must be greater than 0",
            ));
        }

        if self.connection_timeout_secs == 0 {
            return Err(ConfigError::invalid_value(
                "limits.connection_timeout_secs",
                "Must be greater than 0",
            ));
        }

        if self.max_concurrent_operations == 0 {
            return Err(ConfigError::invalid_value(
                "limits.max_concurrent_operations",
                "Must be greater than 0",
            ));
        }

        if self.max_request_body_bytes == 0 {
            return Err(ConfigError::invalid_value(
                "limits.max_request_body_bytes",
                "Must be greater than 0",
            ));
        }

        if self.cache_size == 0 {
            return Err(ConfigError::invalid_value(
                "limits.cache_size",
                "Must be greater than 0",
            ));
        }

        Ok(())
    }
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for LimitsConfig
#[derive(Debug, Default)]
pub struct LimitsConfigBuilder {
    operation_timeout_secs: Option<u64>,
    connection_timeout_secs: Option<u64>,
    max_retries: Option<u32>,
    retry_delay_ms: Option<u64>,
    max_concurrent_operations: Option<usize>,
    rate_limit_per_sec: Option<u32>,
    max_request_body_bytes: Option<usize>,
    cache_size: Option<usize>,
    cache_ttl_secs: Option<u64>,
}

impl LimitsConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn operation_timeout_secs(mut self, secs: u64) -> Self {
        self.operation_timeout_secs = Some(secs);
        self
    }

    pub fn connection_timeout_secs(mut self, secs: u64) -> Self {
        self.connection_timeout_secs = Some(secs);
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    pub fn retry_delay_ms(mut self, ms: u64) -> Self {
        self.retry_delay_ms = Some(ms);
        self
    }

    pub fn max_concurrent_operations(mut self, max: usize) -> Self {
        self.max_concurrent_operations = Some(max);
        self
    }

    pub fn rate_limit_per_sec(mut self, limit: u32) -> Self {
        self.rate_limit_per_sec = Some(limit);
        self
    }

    pub fn max_request_body_bytes(mut self, bytes: usize) -> Self {
        self.max_request_body_bytes = Some(bytes);
        self
    }

    pub fn cache_size(mut self, size: usize) -> Self {
        self.cache_size = Some(size);
        self
    }

    pub fn cache_ttl_secs(mut self, secs: u64) -> Self {
        self.cache_ttl_secs = Some(secs);
        self
    }

    pub fn build(self) -> LimitsConfig {
        let defaults = LimitsConfig::const_defaults();

        LimitsConfig {
            operation_timeout_secs: self
                .operation_timeout_secs
                .unwrap_or(defaults.operation_timeout_secs),
            connection_timeout_secs: self
                .connection_timeout_secs
                .unwrap_or(defaults.connection_timeout_secs),
            max_retries: self.max_retries.unwrap_or(defaults.max_retries),
            retry_delay_ms: self.retry_delay_ms.unwrap_or(defaults.retry_delay_ms),
            max_concurrent_operations: self
                .max_concurrent_operations
                .unwrap_or(defaults.max_concurrent_operations),
            rate_limit_per_sec: self
                .rate_limit_per_sec
                .unwrap_or(defaults.rate_limit_per_sec),
            max_request_body_bytes: self
                .max_request_body_bytes
                .unwrap_or(defaults.max_request_body_bytes),
            cache_size: self.cache_size.unwrap_or(defaults.cache_size),
            cache_ttl_secs: self.cache_ttl_secs.unwrap_or(defaults.cache_ttl_secs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let config = LimitsConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.operation_timeout_secs, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_invalid_timeout() {
        let config = LimitsConfig::builder().operation_timeout_secs(0).build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_concurrent_ops() {
        let config = LimitsConfig::builder().max_concurrent_operations(0).build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_builder() {
        let config = LimitsConfig::builder()
            .operation_timeout_secs(60)
            .max_retries(5)
            .cache_size(2000)
            .build();

        assert_eq!(config.operation_timeout_secs, 60);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.cache_size, 2000);
    }

    #[test]
    fn test_high_throughput_config() {
        let config = LimitsConfig::builder()
            .max_concurrent_operations(500)
            .rate_limit_per_sec(1000)
            .max_request_body_bytes(104_857_600) // 100 MB
            .build();

        assert!(config.validate().is_ok());
        assert_eq!(config.max_concurrent_operations, 500);
    }
}
