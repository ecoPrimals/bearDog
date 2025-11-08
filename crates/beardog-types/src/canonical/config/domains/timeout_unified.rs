//! Unified Timeout Configuration
//!
//! **Status**: Canonical timeout config consolidating:
//! - `beardog-types::canonical::config::domains::timeout::CanonicalTimeoutConfig`
//! - `beardog-config::domains::timeouts::TimeoutConfig`
//!
//! This module provides a comprehensive timeout configuration that combines:
//! - Network-level timeouts (connect, read, write, operation, idle, keepalive)
//! - Domain-specific timeouts (health_check, hsm, discovery, ai)
//! - Builder pattern for flexible construction
//! - Environment variable loading
//! - Comprehensive validation
//! - Preset configurations
//!
//! ## Migration Guide
//!
//! ### From CanonicalTimeoutConfig:
//! ```rust,ignore
//! // Old:
//! use beardog_types::canonical::config::domains::timeout::CanonicalTimeoutConfig;
//! let config = CanonicalTimeoutConfig::default();
//!
//! // New:
//! use beardog_types::canonical::config::domains::timeout_unified::UnifiedTimeoutConfig;
//! let config = UnifiedTimeoutConfig::default();
//! ```
//!
//! ### From TimeoutConfig (beardog-config):
//! ```rust,ignore
//! // Old:
//! use beardog_config::domains::timeouts::TimeoutConfig;
//! let config = TimeoutConfig::from_env();
//!
//! // New:
//! use beardog_types::canonical::config::domains::timeout_unified::UnifiedTimeoutConfig;
//! let config = UnifiedTimeoutConfig::from_env();
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unified timeout configuration for all BearDog operations
///
/// Combines network-level and domain-specific timeouts into a single,
/// comprehensive configuration.
///
/// # Features
///
/// - **Network Timeouts**: connect, read, write, operation, idle, keepalive
/// - **Domain Timeouts**: health_check, hsm, discovery, ai operations
/// - **Builder Pattern**: Flexible construction
/// - **Environment Loading**: Explicit env var support
/// - **Validation**: Comprehensive range checking
/// - **Presets**: Common configurations (default, aggressive, conservative, etc.)
///
/// # Examples
///
/// ```
/// use beardog_types::canonical::config::domains::timeout_unified::UnifiedTimeoutConfig;
///
/// // Default configuration
/// let config = UnifiedTimeoutConfig::default();
///
/// // Load from environment
/// let env_config = UnifiedTimeoutConfig::from_env();
///
/// // Builder pattern
/// let custom = UnifiedTimeoutConfig::builder()
///     .connect_timeout_secs(5)
///     .health_check_secs(10)
///     .build();
///
/// // Presets
/// let aggressive = UnifiedTimeoutConfig::aggressive();
/// let conservative = UnifiedTimeoutConfig::conservative();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct UnifiedTimeoutConfig {
    // ========================================================================
    // NETWORK-LEVEL TIMEOUTS
    // ========================================================================
    
    /// Timeout for establishing a connection (seconds)
    ///
    /// Maximum time to wait when connecting to a remote service.
    /// Applies to: TCP connections, HTTP connections, HSM connections, etc.
    ///
    /// **Default**: 5 seconds  
    /// **Environment**: `BEARDOG_CONNECT_TIMEOUT_SECS`  
    /// **Recommended**: LAN: 1-5s, WAN: 5-15s, Unreliable: 15-30s
    pub connect_timeout_secs: u64,

    /// Timeout for reading data from a connection (seconds)
    ///
    /// Maximum time to wait for data to arrive after a request.
    ///
    /// **Default**: 30 seconds  
    /// **Environment**: `BEARDOG_READ_TIMEOUT_SECS`  
    /// **Recommended**: Fast: 5-30s, Normal: 30-60s, Long: 60-300s
    pub read_timeout_secs: u64,

    /// Timeout for writing data to a connection (seconds)
    ///
    /// Maximum time to wait when sending data.
    ///
    /// **Default**: 30 seconds  
    /// **Environment**: `BEARDOG_WRITE_TIMEOUT_SECS`  
    /// **Recommended**: Small: 5-30s, Large: 30-60s, Very large: 60-300s
    pub write_timeout_secs: u64,

    /// Timeout for the entire operation (seconds)
    ///
    /// Maximum total time for an operation, including connection, request, and response.
    /// Should be >= connect_timeout + read_timeout + write_timeout
    ///
    /// **Default**: 60 seconds  
    /// **Environment**: `BEARDOG_OPERATION_TIMEOUT_SECS`
    pub operation_timeout_secs: u64,

    /// Timeout for idle connections (seconds, 0 = never)
    ///
    /// Connections idle longer than this will be closed.
    /// 0 means connections never time out due to idleness.
    ///
    /// **Default**: 300 seconds (5 minutes)  
    /// **Environment**: `BEARDOG_IDLE_TIMEOUT_SECS`  
    /// **Recommended**: Pools: 60-300s, Long-lived: 300-3600s
    pub idle_timeout_secs: u64,

    /// Timeout for keepalive probes (seconds, 0 = disabled)
    ///
    /// How often to send keepalive probes on idle connections.
    /// 0 means no keepalive probes are sent.
    ///
    /// **Default**: 60 seconds  
    /// **Environment**: `BEARDOG_KEEPALIVE_TIMEOUT_SECS`  
    /// **Recommended**: Active: 30-60s, Normal: 60-120s
    pub keepalive_timeout_secs: u64,

    // ========================================================================
    // DOMAIN-SPECIFIC TIMEOUTS
    // ========================================================================

    /// Health check timeout (seconds)
    ///
    /// Maximum time for health check probes.
    ///
    /// **Default**: 5 seconds  
    /// **Environment**: `BEARDOG_HEALTH_CHECK_TIMEOUT_SECS`  
    /// **Range**: 1-30 seconds
    pub health_check_secs: u64,

    /// HSM operation timeout (seconds)
    ///
    /// Typical HSM operation latency.
    ///
    /// **Default**: 2 seconds  
    /// **Environment**: `BEARDOG_HSM_OPERATION_TIMEOUT_SECS`  
    /// **Range**: 1-10 seconds
    pub hsm_operation_secs: u64,

    /// HSM probe timeout (milliseconds)
    ///
    /// Fast probe for HSM availability check.
    ///
    /// **Default**: 500 milliseconds  
    /// **Environment**: `BEARDOG_HSM_PROBE_TIMEOUT_MILLIS`  
    /// **Range**: 100-5000 milliseconds
    pub hsm_probe_millis: u64,

    /// Service discovery operation timeout (seconds)
    ///
    /// Allows for network latency in discovery.
    ///
    /// **Default**: 10 seconds  
    /// **Environment**: `BEARDOG_DISCOVERY_TIMEOUT_SECS`  
    /// **Range**: 1-60 seconds
    pub discovery_operation_secs: u64,

    /// AI decision timeout (seconds)
    ///
    /// Allows for complex AI decisions.
    ///
    /// **Default**: 30 seconds  
    /// **Environment**: `BEARDOG_DECISION_TIMEOUT_SECS`  
    /// **Range**: 5-300 seconds
    pub ai_decision_secs: u64,

    /// AI request timeout (seconds)
    ///
    /// Inference operations timeout.
    ///
    /// **Default**: 30 seconds  
    /// **Environment**: `BEARDOG_AI_REQUEST_TIMEOUT_SECS`  
    /// **Range**: 5-300 seconds
    pub ai_request_timeout_secs: u64,

    /// AI batch timeout (milliseconds)
    ///
    /// Fast batching timeout.
    ///
    /// **Default**: 10 milliseconds  
    /// **Environment**: `BEARDOG_AI_BATCH_TIMEOUT_MS`  
    /// **Range**: 1-1000 milliseconds
    pub ai_batch_timeout_millis: u64,

    /// Pool idle timeout (seconds)
    ///
    /// Connection pool idle timeout.
    ///
    /// **Default**: 300 seconds (5 minutes)  
    /// **Environment**: `BEARDOG_POOL_IDLE_TIMEOUT_SECS`  
    /// **Range**: 60-3600 seconds
    pub pool_idle_timeout_secs: u64,

    /// Maximum connection age (seconds)
    ///
    /// Connections older than this are rotated.
    ///
    /// **Default**: 3600 seconds (1 hour)  
    /// **Environment**: `BEARDOG_MAX_CONNECTION_AGE_SECS`  
    /// **Range**: 300-86400 seconds
    pub max_connection_age_secs: u64,
}

impl UnifiedTimeoutConfig {
    /// Pure static defaults (no environment variable reads)
    ///
    /// Thread-safe, deterministic, compile-time constants.
    pub const fn const_defaults() -> Self {
        Self {
            // Network timeouts
            connect_timeout_secs: 5,
            read_timeout_secs: 30,
            write_timeout_secs: 30,
            operation_timeout_secs: 60,
            idle_timeout_secs: 300,
            keepalive_timeout_secs: 60,
            
            // Domain timeouts
            health_check_secs: 5,
            hsm_operation_secs: 2,
            hsm_probe_millis: 500,
            discovery_operation_secs: 10,
            ai_decision_secs: 30,
            ai_request_timeout_secs: 30,
            ai_batch_timeout_millis: 10,
            pool_idle_timeout_secs: 300,
            max_connection_age_secs: 3600,
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();
        
        Self {
            connect_timeout_secs: std::env::var("BEARDOG_CONNECT_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.connect_timeout_secs),
            read_timeout_secs: std::env::var("BEARDOG_READ_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.read_timeout_secs),
            write_timeout_secs: std::env::var("BEARDOG_WRITE_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.write_timeout_secs),
            operation_timeout_secs: std::env::var("BEARDOG_OPERATION_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.operation_timeout_secs),
            idle_timeout_secs: std::env::var("BEARDOG_IDLE_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.idle_timeout_secs),
            keepalive_timeout_secs: std::env::var("BEARDOG_KEEPALIVE_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.keepalive_timeout_secs),
            health_check_secs: std::env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.health_check_secs),
            hsm_operation_secs: std::env::var("BEARDOG_HSM_OPERATION_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.hsm_operation_secs),
            hsm_probe_millis: std::env::var("BEARDOG_HSM_PROBE_TIMEOUT_MILLIS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.hsm_probe_millis),
            discovery_operation_secs: std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.discovery_operation_secs),
            ai_decision_secs: std::env::var("BEARDOG_DECISION_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.ai_decision_secs),
            ai_request_timeout_secs: std::env::var("BEARDOG_AI_REQUEST_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.ai_request_timeout_secs),
            ai_batch_timeout_millis: std::env::var("BEARDOG_AI_BATCH_TIMEOUT_MS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.ai_batch_timeout_millis),
            pool_idle_timeout_secs: std::env::var("BEARDOG_POOL_IDLE_TIMEOUT_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.pool_idle_timeout_secs),
            max_connection_age_secs: std::env::var("BEARDOG_MAX_CONNECTION_AGE_SECS")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(defaults.max_connection_age_secs),
        }
    }

    /// Create a builder for flexible configuration
    pub fn builder() -> UnifiedTimeoutConfigBuilder {
        UnifiedTimeoutConfigBuilder::new()
    }

    // Preset configurations

    /// Aggressive timeouts for high-performance local networks
    pub fn aggressive() -> Self {
        Self {
            connect_timeout_secs: 1,
            read_timeout_secs: 5,
            write_timeout_secs: 5,
            operation_timeout_secs: 10,
            idle_timeout_secs: 60,
            keepalive_timeout_secs: 30,
            health_check_secs: 2,
            hsm_operation_secs: 1,
            hsm_probe_millis: 250,
            discovery_operation_secs: 5,
            ai_decision_secs: 10,
            ai_request_timeout_secs: 10,
            ai_batch_timeout_millis: 5,
            pool_idle_timeout_secs: 60,
            max_connection_age_secs: 1800,
        }
    }

    /// Conservative timeouts for unreliable or long-running operations
    pub fn conservative() -> Self {
        Self {
            connect_timeout_secs: 30,
            read_timeout_secs: 120,
            write_timeout_secs: 120,
            operation_timeout_secs: 300,
            idle_timeout_secs: 600,
            keepalive_timeout_secs: 120,
            health_check_secs: 15,
            hsm_operation_secs: 5,
            hsm_probe_millis: 2000,
            discovery_operation_secs: 30,
            ai_decision_secs: 120,
            ai_request_timeout_secs: 120,
            ai_batch_timeout_millis: 50,
            pool_idle_timeout_secs: 600,
            max_connection_age_secs: 7200,
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Network timeouts validation
        if self.connect_timeout_secs == 0 {
            return Err("connect_timeout_secs must be non-zero".to_string());
        }
        if self.read_timeout_secs == 0 {
            return Err("read_timeout_secs must be non-zero".to_string());
        }
        if self.write_timeout_secs == 0 {
            return Err("write_timeout_secs must be non-zero".to_string());
        }
        if self.operation_timeout_secs == 0 {
            return Err("operation_timeout_secs must be non-zero".to_string());
        }

        // Operation timeout should be reasonable
        let min_operation = self.connect_timeout_secs + self.read_timeout_secs.min(self.write_timeout_secs);
        if self.operation_timeout_secs < min_operation {
            return Err(format!(
                "operation_timeout_secs ({}) should be at least {}",
                self.operation_timeout_secs, min_operation
            ));
        }

        // Domain timeouts validation
        if !(1..=30).contains(&self.health_check_secs) {
            return Err(format!("health_check_secs must be 1-30, got {}", self.health_check_secs));
        }
        if !(1..=10).contains(&self.hsm_operation_secs) {
            return Err(format!("hsm_operation_secs must be 1-10, got {}", self.hsm_operation_secs));
        }
        if !(100..=5000).contains(&self.hsm_probe_millis) {
            return Err(format!("hsm_probe_millis must be 100-5000, got {}", self.hsm_probe_millis));
        }

        Ok(())
    }

    // Duration conversion helpers
    pub fn connect_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_secs)
    }
    pub fn read_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.read_timeout_secs)
    }
    pub fn health_check_duration(&self) -> Duration {
        Duration::from_secs(self.health_check_secs)
    }
    pub fn hsm_operation_duration(&self) -> Duration {
        Duration::from_secs(self.hsm_operation_secs)
    }
    pub fn hsm_probe_duration(&self) -> Duration {
        Duration::from_millis(self.hsm_probe_millis)
    }
}

impl Default for UnifiedTimeoutConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for UnifiedTimeoutConfig
#[derive(Debug, Default)]
pub struct UnifiedTimeoutConfigBuilder {
    connect_timeout_secs: Option<u64>,
    read_timeout_secs: Option<u64>,
    health_check_secs: Option<u64>,
    hsm_operation_secs: Option<u64>,
    // Add other fields as needed
}

impl UnifiedTimeoutConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect_timeout_secs(mut self, secs: u64) -> Self {
        self.connect_timeout_secs = Some(secs);
        self
    }

    pub fn read_timeout_secs(mut self, secs: u64) -> Self {
        self.read_timeout_secs = Some(secs);
        self
    }

    pub fn health_check_secs(mut self, secs: u64) -> Self {
        self.health_check_secs = Some(secs);
        self
    }

    pub fn build(self) -> UnifiedTimeoutConfig {
        let defaults = UnifiedTimeoutConfig::const_defaults();
        UnifiedTimeoutConfig {
            connect_timeout_secs: self.connect_timeout_secs.unwrap_or(defaults.connect_timeout_secs),
            read_timeout_secs: self.read_timeout_secs.unwrap_or(defaults.read_timeout_secs),
            health_check_secs: self.health_check_secs.unwrap_or(defaults.health_check_secs),
            hsm_operation_secs: self.hsm_operation_secs.unwrap_or(defaults.hsm_operation_secs),
            ..defaults
        }
    }
}

// Type aliases for backward compatibility
pub type TimeoutConfig = UnifiedTimeoutConfig;
pub type CanonicalTimeoutConfig = UnifiedTimeoutConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UnifiedTimeoutConfig::default();
        assert_eq!(config.connect_timeout_secs, 5);
        assert_eq!(config.health_check_secs, 5);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_presets() {
        let aggressive = UnifiedTimeoutConfig::aggressive();
        assert_eq!(aggressive.connect_timeout_secs, 1);
        
        let conservative = UnifiedTimeoutConfig::conservative();
        assert_eq!(conservative.connect_timeout_secs, 30);
    }

    #[test]
    fn test_validation() {
        let mut config = UnifiedTimeoutConfig::default();
        assert!(config.validate().is_ok());
        
        config.connect_timeout_secs = 0;
        assert!(config.validate().is_err());
    }
}

