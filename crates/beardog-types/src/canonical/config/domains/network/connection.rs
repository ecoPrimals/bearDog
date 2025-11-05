//! # Connection Configuration Module
//!
//! This module contains connection management configurations including connection pooling,
//! timeouts, and load balancing.
//!
//! This is the canonical location for connection pool configuration.
//! All other ConnectionPoolConfig variants should use this via type aliases.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Connection pool configuration
///
/// This is the single source of truth for connection pool settings across BearDog.
/// Consolidates all ConnectionPoolConfig variants from:
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
#[deprecated(since = "3.1.0", note = "Use ConnectionPoolConfig instead")]
pub type ConnectionPoolConfiguration = ConnectionPoolConfig;

/// Timeout configuration - consolidates TimeoutConfig variants
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

/// Load balancer configuration - consolidates LoadBalancerConfig variants
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

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_size: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            max_size: std::env::var("BEARDOG_CONNECTION_POOL_MAX_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            idle_timeout: Duration::from_secs(
                std::env::var("BEARDOG_CONNECTION_IDLE_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(600),
            ),
            max_lifetime: Some(Duration::from_secs(
                std::env::var("BEARDOG_CONNECTION_MAX_LIFETIME_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1800),
            )),
            acquire_timeout: Duration::from_secs(
                std::env::var("BEARDOG_CONNECTION_ACQUIRE_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            enable_validation: true,
            test_on_borrow: true,
            test_on_return: true,
            test_while_idle: true,
            validation_query: Some("SELECT 1".to_string()),
            maintenance_interval: Duration::from_secs(
                std::env::var("BEARDOG_CONNECTION_MAINTENANCE_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
        }
    }
}

impl Default for TimeoutConfiguration {
    fn default() -> Self {
        Self {
            connection_timeout_seconds:
                crate::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_secs(),
            request_timeout_seconds: crate::constants::domains::network::timeouts::REQUEST_TIMEOUT
                .as_secs(),
            keepalive_timeout_seconds: std::env::var("BEARDOG_KEEPALIVE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
            dns_timeout_seconds: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u64,
            tls_handshake_timeout_seconds: std::env::var("BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            read_timeout_seconds: std::env::var("BEARDOG_READ_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            write_timeout_seconds: std::env::var("BEARDOG_WRITE_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            shutdown_timeout_seconds: std::env::var("BEARDOG_SHUTDOWN_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
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

impl Default for LoadBalancerHealthCheckConfiguration {
    fn default() -> Self {
        Self {
            interval_seconds: std::env::var("BEARDOG_LB_HEALTH_CHECK_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            timeout_seconds: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
            unhealthy_threshold: std::env::var("BEARDOG_LB_UNHEALTHY_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            healthy_threshold: std::env::var("BEARDOG_LB_HEALTHY_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),
            path: "/health".to_string(),
            expected_codes: vec![200, 201, 202],
        }
    }
}

impl Default for CircuitBreakerConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
            recovery_timeout_seconds: std::env::var(
                "BEARDOG_CIRCUIT_BREAKER_RECOVERY_TIMEOUT_SECS",
            )
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60),
            half_open_max_calls: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                as u32,
            minimum_throughput: std::env::var("BEARDOG_CIRCUIT_BREAKER_MIN_THROUGHPUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
        }
    }
}

impl Default for FailoverConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_timeout_seconds: std::env::var("BEARDOG_FAILOVER_DETECTION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            max_attempts: std::env::var("BEARDOG_FAILOVER_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            backoff_seconds: crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
        }
    }
}

impl ConnectionPoolConfig {
    /// Validate connection pool configuration
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

        let mut invalid_config = config.clone();
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

        let mut invalid_config = config.clone();
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
