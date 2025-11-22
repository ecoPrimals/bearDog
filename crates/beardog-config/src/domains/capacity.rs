//! Capacity Configuration
//!
//! Configuration for buffer sizes, connection pools, and resource limits.
//!
//! This module provides capacity-related configuration with support for:
//! - Channel and queue buffer sizes
//! - Connection pool limits
//! - Memory and message size limits
//! - Cache capacities
//!
//! ## Pure Defaults Pattern
//!
//! Following the modern concurrent-safe pattern:
//! - `Default` implementation provides pure static defaults
//! - `from_env()` method explicitly loads environment variables
//! - No environment variable reads in `Default`
//!
//! ## Environment Variables
//!
//! - `BEARDOG_CHANNEL_BUFFER` - Default channel buffer size (default: 1000)
//! - `BEARDOG_DISCOVERY_QUEUE_SIZE` - Discovery queue capacity (default: 100)
//! - `BEARDOG_EVENT_BUS_CAPACITY` - Event bus capacity (default: 10000)
//! - `BEARDOG_MAX_CONNECTIONS` - Maximum connections in pool (default: 100)
//! - `BEARDOG_MIN_IDLE_CONNECTIONS` - Minimum idle connections (default: 10)
//! - `BEARDOG_CONNECTION_POOL_TIMEOUT_SECS` - Pool acquire timeout (default: 30)
//! - `BEARDOG_MAX_MESSAGE_SIZE_BYTES` - Maximum message size (default: 10485760 = 10MB)
//! - `BEARDOG_BUFFER_POOL_SIZE` - Buffer pool capacity (default: 1024)
//! - `BEARDOG_CACHE_MAX_ENTRIES` - Maximum cache entries (default: 10000)

use serde::{Deserialize, Serialize};

/// Capacity configuration for buffers, pools, and limits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapacityConfig {
    /// Default channel buffer size for async channels
    ///
    /// Used when creating tokio::sync::mpsc channels throughout the system.
    /// Larger values reduce backpressure but increase memory usage.
    ///
    /// Default: 1000
    pub default_channel_buffer: usize,

    /// Discovery service queue capacity
    ///
    /// Maximum number of pending discovery requests.
    ///
    /// Default: 100
    pub discovery_queue_size: usize,

    /// Event bus capacity
    ///
    /// Maximum number of events that can be queued in the event bus.
    ///
    /// Default: 10000
    pub event_bus_capacity: usize,

    /// Maximum connections in connection pool
    ///
    /// The maximum number of connections (database, API, etc.) that can
    /// be maintained in connection pools.
    ///
    /// Default: 100
    pub max_connections: usize,

    /// Minimum idle connections to maintain
    ///
    /// The minimum number of idle connections to keep open for fast
    /// request handling.
    ///
    /// Default: 10
    pub min_idle_connections: usize,

    /// Connection pool acquire timeout in seconds
    ///
    /// How long to wait when acquiring a connection from the pool
    /// before timing out.
    ///
    /// Default: 30
    pub connection_pool_timeout_secs: u64,

    /// Maximum message size in bytes
    ///
    /// The maximum size of a single message (request/response) that
    /// will be accepted.
    ///
    /// Default: 10485760 (10 MB)
    pub max_message_size_bytes: usize,

    /// Buffer pool size
    ///
    /// Number of pre-allocated buffers to maintain in the buffer pool
    /// for zero-copy optimizations.
    ///
    /// Default: 1024
    pub buffer_pool_size: usize,

    /// Maximum cache entries
    ///
    /// Maximum number of entries in LRU caches used throughout the system.
    ///
    /// Default: 10000
    pub cache_max_entries: usize,
}

impl CapacityConfig {
    /// Pure constant defaults for capacity configuration
    ///
    /// These defaults are chosen to balance:
    /// - Resource usage (memory, connections)
    /// - Performance (queue depth, concurrency)
    /// - Operational stability (timeouts, limits)
    ///
    /// Production deployments should tune these values via environment
    /// variables or configuration files based on actual load patterns.
    pub const fn const_defaults() -> Self {
        Self {
            default_channel_buffer: 1000,
            discovery_queue_size: 100,
            event_bus_capacity: 10000,
            max_connections: 100,
            min_idle_connections: 10,
            connection_pool_timeout_secs: 30,
            max_message_size_bytes: 10 * 1024 * 1024, // 10 MB
            buffer_pool_size: 1024,
            cache_max_entries: 10000,
        }
    }

    /// Load capacity configuration from environment variables
    ///
    /// Starts with default values and overrides with environment variables
    /// where present. Invalid values are ignored (defaults retained).
    ///
    /// # Environment Variables
    ///
    /// - `BEARDOG_CHANNEL_BUFFER` - usize
    /// - `BEARDOG_DISCOVERY_QUEUE_SIZE` - usize
    /// - `BEARDOG_EVENT_BUS_CAPACITY` - usize
    /// - `BEARDOG_MAX_CONNECTIONS` - usize
    /// - `BEARDOG_MIN_IDLE_CONNECTIONS` - usize
    /// - `BEARDOG_CONNECTION_POOL_TIMEOUT_SECS` - u64
    /// - `BEARDOG_MAX_MESSAGE_SIZE_BYTES` - usize
    /// - `BEARDOG_BUFFER_POOL_SIZE` - usize
    /// - `BEARDOG_CACHE_MAX_ENTRIES` - usize
    ///
    /// # Example
    ///
    /// ```bash
    /// export BEARDOG_MAX_CONNECTIONS=200
    /// export BEARDOG_CACHE_MAX_ENTRIES=50000
    /// ```
    pub fn from_env() -> Self {
        Self::from_env_provider(|key| std::env::var(key).ok())
    }

    /// Load capacity configuration from a custom environment provider
    ///
    /// This method enables concurrent-safe testing by accepting any function
    /// that provides environment variable values, without touching global state.
    ///
    /// # Arguments
    ///
    /// * `env_provider` - A function that takes a variable name and returns its value
    ///
    /// # Example
    ///
    /// ```
    /// use beardog_config::domains::capacity::CapacityConfig;
    /// use std::collections::HashMap;
    ///
    /// let mut test_env = HashMap::new();
    /// test_env.insert("BEARDOG_MAX_CONNECTIONS", "200");
    ///
    /// let config = CapacityConfig::from_env_provider(|key| {
    ///     test_env.get(key).map(|s| s.to_string())
    /// });
    ///
    /// assert_eq!(config.max_connections, 200);
    /// ```
    pub fn from_env_provider<F>(env_provider: F) -> Self
    where
        F: Fn(&str) -> Option<String>,
    {
        let mut config = Self::default();

        if let Some(val) = env_provider("BEARDOG_CHANNEL_BUFFER") {
            if let Ok(parsed) = val.parse() {
                config.default_channel_buffer = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_DISCOVERY_QUEUE_SIZE") {
            if let Ok(parsed) = val.parse() {
                config.discovery_queue_size = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_EVENT_BUS_CAPACITY") {
            if let Ok(parsed) = val.parse() {
                config.event_bus_capacity = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_MAX_CONNECTIONS") {
            if let Ok(parsed) = val.parse() {
                config.max_connections = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_MIN_IDLE_CONNECTIONS") {
            if let Ok(parsed) = val.parse() {
                config.min_idle_connections = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_CONNECTION_POOL_TIMEOUT_SECS") {
            if let Ok(parsed) = val.parse() {
                config.connection_pool_timeout_secs = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_MAX_MESSAGE_SIZE_BYTES") {
            if let Ok(parsed) = val.parse() {
                config.max_message_size_bytes = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_BUFFER_POOL_SIZE") {
            if let Ok(parsed) = val.parse() {
                config.buffer_pool_size = parsed;
            }
        }

        if let Some(val) = env_provider("BEARDOG_CACHE_MAX_ENTRIES") {
            if let Ok(parsed) = val.parse() {
                config.cache_max_entries = parsed;
            }
        }

        config
    }
}

impl Default for CapacityConfig {
    /// Returns pure default capacity configuration
    ///
    /// Does NOT read environment variables. This ensures deterministic
    /// defaults and supports concurrent-safe testing.
    ///
    /// For environment-aware configuration, use `CapacityConfig::from_env()`.
    fn default() -> Self {
        Self::const_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_config_defaults() {
        let config = CapacityConfig::default();

        assert_eq!(config.default_channel_buffer, 1000);
        assert_eq!(config.discovery_queue_size, 100);
        assert_eq!(config.event_bus_capacity, 10000);
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.min_idle_connections, 10);
        assert_eq!(config.connection_pool_timeout_secs, 30);
        assert_eq!(config.max_message_size_bytes, 10 * 1024 * 1024);
        assert_eq!(config.buffer_pool_size, 1024);
        assert_eq!(config.cache_max_entries, 10000);
    }

    #[test]
    fn test_capacity_config_const_defaults() {
        let config = CapacityConfig::const_defaults();
        let default = CapacityConfig::default();

        assert_eq!(config, default);
    }

    #[test]
    fn test_capacity_config_from_env_with_overrides() {
        // Concurrent-safe testing: Use HashMap instead of global environment
        use std::collections::HashMap;

        let mut test_env = HashMap::new();
        test_env.insert("BEARDOG_CHANNEL_BUFFER", "2000");
        test_env.insert("BEARDOG_MAX_CONNECTIONS", "200");
        test_env.insert("BEARDOG_CACHE_MAX_ENTRIES", "50000");

        let config =
            CapacityConfig::from_env_provider(|key| test_env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 2000);
        assert_eq!(config.max_connections, 200);
        assert_eq!(config.cache_max_entries, 50000);

        // Unchanged values should remain default
        assert_eq!(config.discovery_queue_size, 100);
        assert_eq!(config.min_idle_connections, 10);
    }

    #[test]
    fn test_capacity_config_from_env_invalid_values_ignored() {
        // Concurrent-safe testing: Use HashMap instead of global environment
        use std::collections::HashMap;

        let mut test_env = HashMap::new();
        test_env.insert("BEARDOG_MAX_CONNECTIONS", "not_a_number");

        let config =
            CapacityConfig::from_env_provider(|key| test_env.get(key).map(|s| s.to_string()));

        // Should fall back to default when parsing fails
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_capacity_config_clone() {
        let config1 = CapacityConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_capacity_config_debug() {
        let config = CapacityConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("CapacityConfig"));
        assert!(debug_str.contains("default_channel_buffer"));
    }

    #[test]
    fn test_capacity_config_serialization() {
        let config = CapacityConfig::default();

        // Test JSON serialization
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: CapacityConfig =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(config, deserialized);
    }
}
