// SPDX-License-Identifier: AGPL-3.0-only

//! System limits and buffer sizes configuration
//!
//! This module provides configuration for system-wide limits, buffer sizes,
//! and resource constraints. All values are configurable via environment
//! variables or configuration files.
//!
//! # Environment Variables
//!
//! - `BEARDOG_BUFFER_SIZE` - I/O buffer size in bytes (default: 8192)
//! - `BEARDOG_MAX_CONNECTIONS` - Maximum concurrent connections (default: 100)
//! - `BEARDOG_MAX_RETRIES` - Maximum retry attempts (default: 3)
//! - `BEARDOG_MAX_MESSAGE_SIZE` - Maximum message size in bytes (default: 1MB)
//! - `BEARDOG_QUEUE_SIZE` - Queue size for async operations (default: 1000)
//! - `BEARDOG_THREAD_POOL_SIZE` - Thread pool size (default: num_cpus)
//!
//! # Examples
//!
//! ```
//! use beardog_config::domains::limits::{LimitsConfig, DEFAULT_BUFFER_SIZE};
//!
//! // Use defaults
//! let config = LimitsConfig::default();
//! assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
//!
//! // Load from environment (reads process environment at call time)
//! let config = LimitsConfig::from_env();
//! ```

use crate::error::ConfigError;
use serde::{Deserialize, Serialize};

/// Default buffer size for I/O operations (8 KB)
pub const DEFAULT_BUFFER_SIZE: usize = 8192;

/// Default maximum concurrent connections
pub const DEFAULT_MAX_CONNECTIONS: usize = 100;

/// Default maximum retry attempts
pub const DEFAULT_MAX_RETRIES: usize = 3;

/// Default backoff duration in milliseconds
pub const DEFAULT_BACKOFF_MS: u64 = 100;

/// Default maximum message size (1 MB)
pub const DEFAULT_MAX_MESSAGE_SIZE: usize = 1_048_576;

/// Default queue size for async operations
pub const DEFAULT_QUEUE_SIZE: usize = 1000;

/// Default minimum thread pool size
pub const DEFAULT_MIN_THREADS: usize = 2;

/// Default maximum thread pool size
pub const DEFAULT_MAX_THREADS: usize = 128;

/// Default operation timeout in seconds
pub const DEFAULT_OPERATION_TIMEOUT_SECS: u64 = 30;

/// System limits and resource constraints configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LimitsConfig {
    /// Buffer size for I/O operations in bytes
    pub buffer_size: usize,

    /// Maximum number of concurrent connections
    pub max_connections: usize,

    /// Maximum number of retry attempts for failed operations
    pub max_retries: usize,

    /// Backoff duration in milliseconds between retries
    pub backoff_ms: u64,

    /// Maximum message size in bytes
    pub max_message_size: usize,

    /// Queue size for async operations
    pub queue_size: usize,

    /// Thread pool size (0 = automatic based on CPU count)
    pub thread_pool_size: usize,

    /// Default operation timeout in seconds
    pub operation_timeout_secs: u64,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            buffer_size: env_or_default("BEARDOG_BUFFER_SIZE", DEFAULT_BUFFER_SIZE),
            max_connections: env_or_default("BEARDOG_MAX_CONNECTIONS", DEFAULT_MAX_CONNECTIONS),
            max_retries: env_or_default("BEARDOG_MAX_RETRIES", DEFAULT_MAX_RETRIES),
            backoff_ms: env_or_default("BEARDOG_BACKOFF_MS", DEFAULT_BACKOFF_MS),
            max_message_size: env_or_default("BEARDOG_MAX_MESSAGE_SIZE", DEFAULT_MAX_MESSAGE_SIZE),
            queue_size: env_or_default("BEARDOG_QUEUE_SIZE", DEFAULT_QUEUE_SIZE),
            thread_pool_size: env_or_default("BEARDOG_THREAD_POOL_SIZE", 0), // 0 = auto-detect
            operation_timeout_secs: env_or_default(
                "BEARDOG_OPERATION_TIMEOUT_SECS",
                DEFAULT_OPERATION_TIMEOUT_SECS,
            ),
        }
    }
}

impl LimitsConfig {
    /// Create a new limits configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            buffer_size: env_or_default("BEARDOG_BUFFER_SIZE", DEFAULT_BUFFER_SIZE),
            max_connections: env_or_default("BEARDOG_MAX_CONNECTIONS", DEFAULT_MAX_CONNECTIONS),
            max_retries: env_or_default("BEARDOG_MAX_RETRIES", DEFAULT_MAX_RETRIES),
            backoff_ms: env_or_default("BEARDOG_BACKOFF_MS", DEFAULT_BACKOFF_MS),
            max_message_size: env_or_default("BEARDOG_MAX_MESSAGE_SIZE", DEFAULT_MAX_MESSAGE_SIZE),
            queue_size: env_or_default("BEARDOG_QUEUE_SIZE", DEFAULT_QUEUE_SIZE),
            thread_pool_size: env_or_default("BEARDOG_THREAD_POOL_SIZE", 0),
            operation_timeout_secs: env_or_default(
                "BEARDOG_OPERATION_TIMEOUT_SECS",
                DEFAULT_OPERATION_TIMEOUT_SECS,
            ),
        }
    }

    /// Create a builder for constructing a `LimitsConfig`
    #[must_use]
    pub fn builder() -> LimitsConfigBuilder {
        LimitsConfigBuilder::default()
    }

    /// Create a new limits configuration with explicit values
    ///
    /// Prefer using `LimitsConfig::builder()` for cleaner construction.
    #[expect(
        clippy::too_many_arguments,
        reason = "construction params, builder would obscure required fields"
    )]
    pub const fn new(
        buffer_size: usize,
        max_connections: usize,
        max_retries: usize,
        backoff_ms: u64,
        max_message_size: usize,
        queue_size: usize,
        thread_pool_size: usize,
        operation_timeout_secs: u64,
    ) -> Self {
        Self {
            buffer_size,
            max_connections,
            max_retries,
            backoff_ms,
            max_message_size,
            queue_size,
            thread_pool_size,
            operation_timeout_secs,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.buffer_size == 0 {
            return Err(ConfigError::validation(
                "Buffer size must be greater than 0",
            ));
        }

        if self.max_connections == 0 {
            return Err(ConfigError::validation(
                "Maximum connections must be greater than 0",
            ));
        }

        if self.max_message_size == 0 {
            return Err(ConfigError::validation(
                "Maximum message size must be greater than 0",
            ));
        }

        if self.queue_size == 0 {
            return Err(ConfigError::validation("Queue size must be greater than 0"));
        }

        if self.thread_pool_size > DEFAULT_MAX_THREADS {
            return Err(ConfigError::validation(format!(
                "Thread pool size cannot exceed {DEFAULT_MAX_THREADS}"
            )));
        }

        Ok(())
    }

    /// Get the actual thread pool size (automatic if set to 0)
    #[must_use]
    pub fn effective_thread_pool_size(&self) -> usize {
        if self.thread_pool_size == 0 {
            // Use sensible default based on available CPUs
            let cpus = std::thread::available_parallelism()
                .map(std::num::NonZero::get)
                .unwrap_or(DEFAULT_MIN_THREADS);
            cpus.clamp(DEFAULT_MIN_THREADS, DEFAULT_MAX_THREADS)
        } else {
            self.thread_pool_size
        }
    }
}

/// Builder for constructing `LimitsConfig` with fluent API
#[derive(Debug, Clone, Default)]
pub struct LimitsConfigBuilder {
    buffer_size: Option<usize>,
    max_connections: Option<usize>,
    max_retries: Option<usize>,
    backoff_ms: Option<u64>,
    max_message_size: Option<usize>,
    queue_size: Option<usize>,
    thread_pool_size: Option<usize>,
    operation_timeout_secs: Option<u64>,
}

impl LimitsConfigBuilder {
    /// Set the buffer size for I/O operations
    #[must_use]
    pub const fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = Some(size);
        self
    }

    /// Set the maximum number of concurrent connections
    #[must_use]
    pub const fn max_connections(mut self, count: usize) -> Self {
        self.max_connections = Some(count);
        self
    }

    /// Set the maximum number of retry attempts
    #[must_use]
    pub const fn max_retries(mut self, count: usize) -> Self {
        self.max_retries = Some(count);
        self
    }

    /// Set the backoff duration in milliseconds between retries
    #[must_use]
    pub const fn backoff_ms(mut self, ms: u64) -> Self {
        self.backoff_ms = Some(ms);
        self
    }

    /// Set the maximum message size in bytes
    #[must_use]
    pub const fn max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = Some(size);
        self
    }

    /// Set the queue size for async operations
    #[must_use]
    pub const fn queue_size(mut self, size: usize) -> Self {
        self.queue_size = Some(size);
        self
    }

    /// Set the thread pool size (0 = automatic based on CPU count)
    #[must_use]
    pub const fn thread_pool_size(mut self, size: usize) -> Self {
        self.thread_pool_size = Some(size);
        self
    }

    /// Set the default operation timeout in seconds
    #[must_use]
    pub const fn operation_timeout_secs(mut self, secs: u64) -> Self {
        self.operation_timeout_secs = Some(secs);
        self
    }

    /// Build the `LimitsConfig`, using defaults for unset values
    #[must_use]
    pub fn build(self) -> LimitsConfig {
        let defaults = LimitsConfig::default();
        LimitsConfig {
            buffer_size: self.buffer_size.unwrap_or(defaults.buffer_size),
            max_connections: self.max_connections.unwrap_or(defaults.max_connections),
            max_retries: self.max_retries.unwrap_or(defaults.max_retries),
            backoff_ms: self.backoff_ms.unwrap_or(defaults.backoff_ms),
            max_message_size: self.max_message_size.unwrap_or(defaults.max_message_size),
            queue_size: self.queue_size.unwrap_or(defaults.queue_size),
            thread_pool_size: self.thread_pool_size.unwrap_or(defaults.thread_pool_size),
            operation_timeout_secs: self
                .operation_timeout_secs
                .unwrap_or(defaults.operation_timeout_secs),
        }
    }
}

/// Helper function to get environment variable or use default
fn env_or_default<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
#[path = "limits_comprehensive_tests.rs"]
mod limits_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let config = LimitsConfig::default();
        assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
        assert_eq!(config.max_retries, DEFAULT_MAX_RETRIES);
        assert_eq!(config.backoff_ms, DEFAULT_BACKOFF_MS);
        assert_eq!(config.max_message_size, DEFAULT_MAX_MESSAGE_SIZE);
        assert_eq!(config.queue_size, DEFAULT_QUEUE_SIZE);
    }

    #[test]
    fn test_validation_success() {
        let config = LimitsConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_zero_buffer() {
        let config = LimitsConfig {
            buffer_size: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_zero_connections() {
        let config = LimitsConfig {
            max_connections: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_too_many_threads() {
        let config = LimitsConfig {
            thread_pool_size: DEFAULT_MAX_THREADS + 1,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_effective_thread_pool_size() {
        let config = LimitsConfig {
            thread_pool_size: 0,
            ..Default::default()
        };
        assert!(config.effective_thread_pool_size() >= DEFAULT_MIN_THREADS);

        let config2 = LimitsConfig {
            thread_pool_size: 8,
            ..Default::default()
        };
        assert_eq!(config2.effective_thread_pool_size(), 8);
    }

    #[test]
    fn test_custom_config() {
        let config = LimitsConfig::new(16384, 200, 5, 200, 2_097_152, 2000, 16, 60);
        assert_eq!(config.buffer_size, 16384);
        assert_eq!(config.max_connections, 200);
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.backoff_ms, 200);
        assert_eq!(config.max_message_size, 2_097_152);
        assert_eq!(config.queue_size, 2000);
        assert_eq!(config.thread_pool_size, 16);
        assert_eq!(config.operation_timeout_secs, 60);
    }

    #[test]
    fn test_serialization() {
        let config = LimitsConfig::default();
        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: LimitsConfig = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_clone() {
        let config1 = LimitsConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn test_builder_pattern() {
        let config = LimitsConfig::builder()
            .buffer_size(16384)
            .max_connections(200)
            .max_retries(5)
            .build();

        assert_eq!(config.buffer_size, 16384);
        assert_eq!(config.max_connections, 200);
        assert_eq!(config.max_retries, 5);
        // Defaults for unset values
        assert_eq!(config.backoff_ms, DEFAULT_BACKOFF_MS);
        assert_eq!(config.max_message_size, DEFAULT_MAX_MESSAGE_SIZE);
    }

    #[test]
    fn test_builder_all_fields() {
        let config = LimitsConfig::builder()
            .buffer_size(32768)
            .max_connections(500)
            .max_retries(10)
            .backoff_ms(500)
            .max_message_size(4_194_304)
            .queue_size(5000)
            .thread_pool_size(32)
            .operation_timeout_secs(120)
            .build();

        assert_eq!(config.buffer_size, 32768);
        assert_eq!(config.max_connections, 500);
        assert_eq!(config.max_retries, 10);
        assert_eq!(config.backoff_ms, 500);
        assert_eq!(config.max_message_size, 4_194_304);
        assert_eq!(config.queue_size, 5000);
        assert_eq!(config.thread_pool_size, 32);
        assert_eq!(config.operation_timeout_secs, 120);
    }
}
