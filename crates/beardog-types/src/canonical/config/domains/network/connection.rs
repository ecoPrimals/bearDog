// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Connection Configuration Module
//!
//! This module contains connection management configurations including connection pooling,
//! timeouts, and load balancing.
//!
//! This is the canonical location for connection pool configuration.
//! All other `ConnectionPoolConfig` variants should use this via type aliases.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Connection pool configuration
///
/// This is the single source of truth for connection pool settings across `BearDog`.
/// Consolidates all `ConnectionPoolConfig` variants from:
/// - `beardog-types/src/network.rs`
/// - `beardog-types/src/canonical/network.rs`
/// - `beardog-types/src/canonical/providers_unified/connection.rs`
/// - `beardog-types/src/canonical/providers/base.rs`
/// - `beardog-production/src/config_management.rs`
///
/// # Field Types
/// - Uses `usize` for pool sizes (matches Rust collection types)
/// - Uses `Duration` for timeouts (type-safe)
/// - Comprehensive validation included
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectionPoolConfig {
    /// Minimum connections maintained in pool
    pub min_size: usize,

    /// Maximum connections allowed in pool
    pub max_size: usize,

    /// Connection idle timeout (connections idle longer than this are closed)
    #[serde(
        serialize_with = "serialize_duration_as_secs",
        deserialize_with = "deserialize_duration_from_secs"
    )]
    pub idle_timeout: Duration,

    /// Maximum connection lifetime (connections older than this are closed)
    #[serde(
        serialize_with = "serialize_optional_duration_as_secs",
        deserialize_with = "deserialize_optional_duration_from_secs"
    )]
    pub max_lifetime: Option<Duration>,

    /// Timeout for acquiring a connection from the pool
    #[serde(
        serialize_with = "serialize_duration_as_secs",
        deserialize_with = "deserialize_duration_from_secs"
    )]
    pub acquire_timeout: Duration,

    /// Enable connection validation before use
    pub enable_validation: bool,

    /// Test connections when borrowed from pool
    pub test_on_borrow: bool,

    /// Test connections when returned to pool
    pub test_on_return: bool,

    /// Test idle connections periodically
    pub test_while_idle: bool,

    /// Validation query/check (e.g., "SELECT 1")
    pub validation_query: Option<String>,

    /// Pool maintenance interval (time between maintenance runs)
    #[serde(
        serialize_with = "serialize_duration_as_secs",
        deserialize_with = "deserialize_duration_from_secs"
    )]
    pub maintenance_interval: Duration,
}

// Serde helper functions for Duration serialization
fn serialize_duration_as_secs<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(duration.as_secs())
}

fn deserialize_duration_from_secs<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
}

fn serialize_optional_duration_as_secs<S>(
    duration: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match duration {
        Some(d) => serializer.serialize_some(&d.as_secs()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_optional_duration_from_secs<'de, D>(
    deserializer: D,
) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt_secs: Option<u64> = Option::deserialize(deserializer)?;
    Ok(opt_secs.map(Duration::from_secs))
}

/// Backward compatibility alias (deprecated)
/// Timeout configuration - consolidates `TimeoutConfig` variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfiguration {
    /// Connection establishment timeout
    pub connection_timeout_seconds: u64,
    /// Request/response timeout
    pub request_timeout_seconds: u64,
    /// Keep-alive timeout
    pub keepalive_timeout_seconds: u64,
    /// DNS resolution timeout
    pub dns_timeout_seconds: u64,
    /// TLS handshake timeout
    pub tls_handshake_timeout_seconds: u64,
    /// Read timeout
    pub read_timeout_seconds: u64,
    /// Write timeout
    pub write_timeout_seconds: u64,
    /// Graceful shutdown timeout
    pub shutdown_timeout_seconds: u64,
}

/// Load balancer configuration - consolidates `LoadBalancerConfig` variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfiguration {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check configuration for load balancer
    pub health_checks: LoadBalancerHealthCheckConfiguration,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfiguration,
    /// Sticky sessions configuration
    pub sticky_sessions: Option<StickySessionConfiguration>,
    /// Failover configuration
    pub failover: FailoverConfiguration,
}

/// Load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round-robin algorithm
    RoundRobin,
    /// Least connections algorithm
    LeastConnections,
    /// Weighted round-robin algorithm
    WeightedRoundRobin,
    /// Random algorithm
    Random,
    /// IP hash algorithm
    IpHash,
    /// Least response time algorithm
    LeastResponseTime,
}

/// Load balancer health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerHealthCheckConfiguration {
    /// Health check interval seconds
    pub interval_seconds: u64,
    /// Health check timeout seconds
    pub timeout_seconds: u64,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
    /// Healthy threshold
    pub healthy_threshold: u32,
    /// Health check path
    pub path: String,
    /// Expected response codes
    pub expected_codes: Vec<u16>,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfiguration {
    /// Enable circuit breaker
    pub enabled: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery timeout seconds
    pub recovery_timeout_seconds: u64,
    /// Half-open max calls
    pub half_open_max_calls: u32,
    /// Minimum throughput
    pub minimum_throughput: u32,
}

/// Sticky session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickySessionConfiguration {
    /// Cookie name for sticky sessions
    pub cookie_name: String,
    /// Cookie duration seconds
    pub cookie_duration_seconds: u64,
    /// Enable secure cookie
    pub secure_cookie: bool,
    /// Enable HTTP-only cookie
    pub http_only_cookie: bool,
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfiguration {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover detection timeout seconds
    pub detection_timeout_seconds: u64,
    /// Maximum failover attempts
    pub max_attempts: u32,
    /// Failover backoff seconds
    pub backoff_seconds: u64,
}

impl FailoverConfiguration {
    /// Default detection timeout in seconds
    pub const DEFAULT_DETECTION_TIMEOUT_SECS: u64 = 30;

    /// Default maximum attempts
    pub const DEFAULT_MAX_ATTEMPTS: u32 = 3;

    /// Default backoff in seconds
    pub const DEFAULT_BACKOFF_SECS: u64 =
        crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64;

    /// Create `FailoverConfiguration` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub const fn with_defaults() -> Self {
        Self {
            enabled: true,
            detection_timeout_seconds: Self::DEFAULT_DETECTION_TIMEOUT_SECS,
            max_attempts: Self::DEFAULT_MAX_ATTEMPTS,
            backoff_seconds: Self::DEFAULT_BACKOFF_SECS,
        }
    }

    /// Create `FailoverConfiguration` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_FAILOVER_DETECTION_TIMEOUT_SECS`: Detection timeout (default: 30)
    /// - `BEARDOG_FAILOVER_MAX_ATTEMPTS`: Maximum attempts (default: 3)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: true,
            detection_timeout_seconds: get("BEARDOG_FAILOVER_DETECTION_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_DETECTION_TIMEOUT_SECS),
            max_attempts: get("BEARDOG_FAILOVER_MAX_ATTEMPTS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_ATTEMPTS),
            backoff_seconds: Self::DEFAULT_BACKOFF_SECS,
        }
    }
}

impl ConnectionPoolConfig {
    /// Default maximum pool size
    pub const DEFAULT_MAX_SIZE: usize = 100;

    /// Default idle timeout in seconds
    pub const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 600;

    /// Default maximum lifetime in seconds
    pub const DEFAULT_MAX_LIFETIME_SECS: u64 = 1800;

    /// Default acquire timeout in seconds
    pub const DEFAULT_ACQUIRE_TIMEOUT_SECS: u64 = 30;

    /// Default maintenance interval in seconds
    pub const DEFAULT_MAINTENANCE_INTERVAL_SECS: u64 = 60;

    /// Create `ConnectionPoolConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            min_size: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            max_size: Self::DEFAULT_MAX_SIZE,
            idle_timeout: Duration::from_secs(Self::DEFAULT_IDLE_TIMEOUT_SECS),
            max_lifetime: Some(Duration::from_secs(Self::DEFAULT_MAX_LIFETIME_SECS)),
            acquire_timeout: Duration::from_secs(Self::DEFAULT_ACQUIRE_TIMEOUT_SECS),
            enable_validation: true,
            test_on_borrow: true,
            test_on_return: true,
            test_while_idle: true,
            validation_query: Some("SELECT 1".to_string()),
            maintenance_interval: Duration::from_secs(Self::DEFAULT_MAINTENANCE_INTERVAL_SECS),
        }
    }

    /// Create `ConnectionPoolConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_CONNECTION_POOL_MAX_SIZE`: Maximum pool size (default: 100)
    /// - `BEARDOG_CONNECTION_IDLE_TIMEOUT_SECS`: Idle timeout (default: 600)
    /// - `BEARDOG_CONNECTION_MAX_LIFETIME_SECS`: Max lifetime (default: 1800)
    /// - `BEARDOG_CONNECTION_ACQUIRE_TIMEOUT_SECS`: Acquire timeout (default: 30)
    /// - `BEARDOG_CONNECTION_MAINTENANCE_INTERVAL_SECS`: Maintenance interval (default: 60)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            min_size: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            max_size: get("BEARDOG_CONNECTION_POOL_MAX_SIZE")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_SIZE),
            idle_timeout: Duration::from_secs(
                get("BEARDOG_CONNECTION_IDLE_TIMEOUT_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_IDLE_TIMEOUT_SECS),
            ),
            max_lifetime: Some(Duration::from_secs(
                get("BEARDOG_CONNECTION_MAX_LIFETIME_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_MAX_LIFETIME_SECS),
            )),
            acquire_timeout: Duration::from_secs(
                get("BEARDOG_CONNECTION_ACQUIRE_TIMEOUT_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_ACQUIRE_TIMEOUT_SECS),
            ),
            enable_validation: true,
            test_on_borrow: true,
            test_on_return: true,
            test_while_idle: true,
            validation_query: Some("SELECT 1".to_string()),
            maintenance_interval: Duration::from_secs(
                get("BEARDOG_CONNECTION_MAINTENANCE_INTERVAL_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_MAINTENANCE_INTERVAL_SECS),
            ),
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl TimeoutConfiguration {
    /// Default keepalive timeout in seconds
    pub const DEFAULT_KEEPALIVE_TIMEOUT_SECS: u64 = 60;

    /// Default TLS handshake timeout in seconds
    pub const DEFAULT_TLS_HANDSHAKE_TIMEOUT_SECS: u64 = 30;

    /// Default read timeout in seconds
    pub const DEFAULT_READ_TIMEOUT_SECS: u64 = 30;

    /// Default write timeout in seconds
    pub const DEFAULT_WRITE_TIMEOUT_SECS: u64 = 30;

    /// Default shutdown timeout in seconds
    pub const DEFAULT_SHUTDOWN_TIMEOUT_SECS: u64 = 30;

    /// Default DNS resolution timeout in seconds
    pub const DEFAULT_DNS_TIMEOUT_SECS: u64 = 5;

    /// Create `TimeoutConfiguration` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub const fn with_defaults() -> Self {
        Self {
            connection_timeout_seconds:
                crate::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_secs(),
            request_timeout_seconds: crate::constants::domains::network::timeouts::REQUEST_TIMEOUT
                .as_secs(),
            keepalive_timeout_seconds: Self::DEFAULT_KEEPALIVE_TIMEOUT_SECS,
            dns_timeout_seconds: Self::DEFAULT_DNS_TIMEOUT_SECS,
            tls_handshake_timeout_seconds: Self::DEFAULT_TLS_HANDSHAKE_TIMEOUT_SECS,
            read_timeout_seconds: Self::DEFAULT_READ_TIMEOUT_SECS,
            write_timeout_seconds: Self::DEFAULT_WRITE_TIMEOUT_SECS,
            shutdown_timeout_seconds: Self::DEFAULT_SHUTDOWN_TIMEOUT_SECS,
        }
    }

    /// Create `TimeoutConfiguration` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_KEEPALIVE_TIMEOUT_SECS`: Keepalive timeout (default: 60)
    /// - `BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS`: TLS handshake timeout (default: 30)
    /// - `BEARDOG_READ_TIMEOUT_SECS`: Read timeout (default: 30)
    /// - `BEARDOG_WRITE_TIMEOUT_SECS`: Write timeout (default: 30)
    /// - `BEARDOG_SHUTDOWN_TIMEOUT_SECS`: Shutdown timeout (default: 30)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            connection_timeout_seconds:
                crate::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_secs(),
            request_timeout_seconds: crate::constants::domains::network::timeouts::REQUEST_TIMEOUT
                .as_secs(),
            keepalive_timeout_seconds: get("BEARDOG_KEEPALIVE_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_KEEPALIVE_TIMEOUT_SECS),
            dns_timeout_seconds: get("BEARDOG_DNS_RESOLUTION_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_DNS_TIMEOUT_SECS),
            tls_handshake_timeout_seconds: get("BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_TLS_HANDSHAKE_TIMEOUT_SECS),
            read_timeout_seconds: get("BEARDOG_READ_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_READ_TIMEOUT_SECS),
            write_timeout_seconds: get("BEARDOG_WRITE_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_WRITE_TIMEOUT_SECS),
            shutdown_timeout_seconds: get("BEARDOG_SHUTDOWN_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_SHUTDOWN_TIMEOUT_SECS),
        }
    }
}

impl Default for TimeoutConfiguration {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for LoadBalancerConfiguration {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_checks: LoadBalancerHealthCheckConfiguration::default(),
            circuit_breaker: CircuitBreakerConfiguration::default(),
            sticky_sessions: None,
            failover: FailoverConfiguration::default(),
        }
    }
}

impl LoadBalancerHealthCheckConfiguration {
    /// Default health check interval in seconds
    pub const DEFAULT_INTERVAL_SECS: u64 = 30;

    /// Default unhealthy threshold
    pub const DEFAULT_UNHEALTHY_THRESHOLD: u32 = 3;

    /// Default healthy threshold
    pub const DEFAULT_HEALTHY_THRESHOLD: u32 = 2;

    /// Create `LoadBalancerHealthCheckConfiguration` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            interval_seconds: Self::DEFAULT_INTERVAL_SECS,
            timeout_seconds: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
            unhealthy_threshold: Self::DEFAULT_UNHEALTHY_THRESHOLD,
            healthy_threshold: Self::DEFAULT_HEALTHY_THRESHOLD,
            path: "/health".to_string(),
            expected_codes: vec![200, 201, 202],
        }
    }

    /// Create `LoadBalancerHealthCheckConfiguration` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_LB_HEALTH_CHECK_INTERVAL_SECS`: Health check interval (default: 30)
    /// - `BEARDOG_LB_UNHEALTHY_THRESHOLD`: Unhealthy threshold (default: 3)
    /// - `BEARDOG_LB_HEALTHY_THRESHOLD`: Healthy threshold (default: 2)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            interval_seconds: get("BEARDOG_LB_HEALTH_CHECK_INTERVAL_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_INTERVAL_SECS),
            timeout_seconds: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
            unhealthy_threshold: get("BEARDOG_LB_UNHEALTHY_THRESHOLD")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_UNHEALTHY_THRESHOLD),
            healthy_threshold: get("BEARDOG_LB_HEALTHY_THRESHOLD")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_HEALTHY_THRESHOLD),
            path: "/health".to_string(),
            expected_codes: vec![200, 201, 202],
        }
    }
}

impl Default for LoadBalancerHealthCheckConfiguration {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl CircuitBreakerConfiguration {
    /// Default recovery timeout in seconds
    pub const DEFAULT_RECOVERY_TIMEOUT_SECS: u64 = 60;

    /// Default minimum throughput
    pub const DEFAULT_MIN_THROUGHPUT: u32 = 20;

    /// Create `CircuitBreakerConfiguration` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for circuit breaker thresholds"
    )]
    #[must_use]
    pub const fn with_defaults() -> Self {
        Self {
            enabled: true,
            failure_threshold: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
            recovery_timeout_seconds: Self::DEFAULT_RECOVERY_TIMEOUT_SECS,
            half_open_max_calls: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
            minimum_throughput: Self::DEFAULT_MIN_THROUGHPUT,
        }
    }

    /// Create `CircuitBreakerConfiguration` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_CIRCUIT_BREAKER_RECOVERY_TIMEOUT_SECS`: Recovery timeout (default: 60)
    /// - `BEARDOG_CIRCUIT_BREAKER_MIN_THROUGHPUT`: Minimum throughput (default: 20)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    #[expect(
        clippy::cast_possible_truncation,
        reason = "default pool size fits u32 for circuit breaker thresholds"
    )]
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: true,
            failure_threshold: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
            recovery_timeout_seconds: get("BEARDOG_CIRCUIT_BREAKER_RECOVERY_TIMEOUT_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_RECOVERY_TIMEOUT_SECS),
            half_open_max_calls: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
            minimum_throughput: get("BEARDOG_CIRCUIT_BREAKER_MIN_THROUGHPUT")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MIN_THROUGHPUT),
        }
    }
}

impl Default for CircuitBreakerConfiguration {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for FailoverConfiguration {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl ConnectionPoolConfig {
    /// Validate connection pool configuration
    ///
    /// # Errors
    ///
    /// Returns an error if pool min/max sizes are inconsistent or zero.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.max_size < self.min_size {
            return Err(BearDogError::configuration(
                "Pool max size cannot be less than min size",
            ));
        }

        if self.max_size == 0 {
            return Err(BearDogError::configuration("Pool max size cannot be zero"));
        }

        Ok(())
    }
}

impl TimeoutConfiguration {
    /// Validate timeout configuration
    ///
    /// # Errors
    ///
    /// Returns an error if connection or request timeouts are zero.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.connection_timeout_seconds == 0 {
            return Err(BearDogError::configuration(
                "Connection timeout cannot be zero",
            ));
        }

        if self.request_timeout_seconds == 0 {
            return Err(BearDogError::configuration(
                "Request timeout cannot be zero",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_validation() {
        let config = ConnectionPoolConfig::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        invalid_config.max_size = 5;
        invalid_config.min_size = 10;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_timeout_validation() {
        let config = TimeoutConfiguration::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.validate().is_ok());

        let mut invalid_config = config;
        invalid_config.connection_timeout_seconds = 0;
        assert!(invalid_config.validate().is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_load_balancer_defaults() {
        let config = LoadBalancerConfiguration::default();
        assert!(matches!(
            config.algorithm,
            LoadBalancingAlgorithm::RoundRobin
        ));
        assert!(config.circuit_breaker.enabled);
        assert!(config.failover.enabled);
    }
}
