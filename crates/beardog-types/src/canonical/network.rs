// SPDX-License-Identifier: AGPL-3.0-or-later

// Network configuration types for BearDog
// Provides network-related configuration structures and utilities

use crate::canonical::traits::TimeoutPolicy;
use crate::constants::domains::network::addresses::WILDCARD_IPV4;
use beardog_config::domains::network_ports::DEFAULT_API_PORT;
use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Bind address, port, TLS, pooling, and timeout settings for network listeners and clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The bind address value
    pub bind_address: String,
    /// Port number
    /// Number of port
    pub port: u16,
    /// Enable TLS/SSL
    /// Whether tls is enabled
    pub tls_enabled: bool,
    /// TLS certificate path
    /// Optional tls cert path
    pub tls_cert_path: Option<String>,
    /// TLS private key path
    /// Optional tls key path
    pub tls_key_path: Option<String>,
    /// Connection pool configuration
    /// The connection pool value
    pub connection_pool: crate::canonical::config::domains::network::ConnectionPoolConfig,
    /// Timeout configurations
    pub timeouts: TimeoutConfig,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: DEFAULT_API_PORT,
            tls_enabled: true,
            tls_cert_path: None,
            tls_key_path: None,
            connection_pool:
                crate::canonical::config::domains::network::ConnectionPoolConfig::default(),
            timeouts: TimeoutConfig::default(),
        }
    }
}

/// Connection pool configuration (DEPRECATED - use canonical config)
///
/// **MIGRATION**: Use `canonical::config::domains::network::ConnectionPoolConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::ConnectionPoolConfig instead"
)]
pub type ConnectionPoolConfig = crate::canonical::config::domains::network::ConnectionPoolConfig;

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            keep_alive_timeout: Duration::from_secs(300),
        }
    }
}

// Implement TimeoutPolicy trait for network timeout configuration
impl TimeoutPolicy for TimeoutConfig {
    fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }

    fn operation_timeout(&self, operation: &str) -> Duration {
        match operation {
            "request" | "read" | "write" => self.request_timeout,
            "connect" | "connection" => self.connection_timeout,
            "keepalive" | "keep_alive" => self.keep_alive_timeout,
            _ => self.request_timeout, // Default to request timeout
        }
    }

    fn should_timeout(&self, elapsed: Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }

    fn global_timeout(&self) -> Option<Duration> {
        Some(self.request_timeout)
    }

    fn read_timeout(&self) -> Duration {
        self.request_timeout
    }

    fn write_timeout(&self) -> Duration {
        self.request_timeout
    }

    fn idle_timeout(&self) -> Option<Duration> {
        Some(self.keep_alive_timeout)
    }

    fn remaining_time(&self, elapsed: Duration, operation: &str) -> Duration {
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    fn validate(&self) -> Result<(), String> {
        if self.connection_timeout.is_zero() {
            return Err("Connection timeout cannot be zero".to_string());
        }
        if self.request_timeout.is_zero() {
            return Err("Request timeout cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        self.connection_timeout >= Duration::from_secs(1)
            && self.connection_timeout <= Duration::from_secs(60)
            && self.request_timeout >= Duration::from_secs(5)
            && self.validate().is_ok()
    }
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancing strategy
    /// The strategy value
    pub strategy: LoadBalancingStrategy,
    /// Backend servers
    /// Collection of backends
    pub backends: Vec<BackendServer>,
    /// Health check configuration
    /// The health check value
    pub health_check: HealthCheckConfig,
}

/// Load balancing strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// IP hash
    IpHash,
}

/// Backend server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServer {
    /// Server address
    /// The address value
    pub address: String,
    /// Server port
    /// Number of port
    pub port: u16,
    /// Number of weight
    pub weight: u32,
    /// Server enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Server metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Check interval
    /// The interval value
    pub interval: Duration,
    /// Check timeout
    pub timeout: Duration,
    /// Health check path
    /// The path value
    pub path: String,
    /// Expected status code
    /// Current status of the expected
    pub expected_status: u16,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            path: "/health".to_string(),
            expected_status: 200,
        }
    }
}

impl NetworkConfig {
    /// Load network configuration from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            bind_address: std::env::var(env_keys::ENV_NETWORK_BIND_ADDRESS)
                .or_else(|_| std::env::var(env_keys::ENV_BIND_ADDRESS))
                .unwrap_or_else(|_| WILDCARD_IPV4.to_string()),
            port: std::env::var(env_keys::ENV_NETWORK_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_API_PORT),
            ..Default::default()
        }
    }

    /// Create a new network configuration
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the full bind address including port
    #[must_use]
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.bind_address, self.port)
    }

    /// Check if TLS is properly configured
    #[must_use]
    /// Checks if tls configured
    pub const fn is_tls_configured(&self) -> bool {
        self.tls_enabled && self.tls_cert_path.is_some() && self.tls_key_path.is_some()
    }

    /// Validate the network configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if port, TLS paths, or connection pool bounds are invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("Port must be greater than 0".to_string());
        }

        if self.tls_enabled {
            if self.tls_cert_path.is_none() {
                return Err("TLS certificate path is required when TLS is enabled".to_string());
            }
            if self.tls_key_path.is_none() {
                return Err("TLS key path is required when TLS is enabled".to_string());
            }
        }

        if self.connection_pool.max_size == 0 {
            return Err("Connection pool max size must be greater than 0".to_string());
        }

        if self.connection_pool.max_size < self.connection_pool.min_size {
            return Err("Connection pool max size must be >= min size".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod network_timeout_policy_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn timeout_config_operation_timeout_branches() {
        let tc = TimeoutConfig::default();
        assert_eq!(tc.operation_timeout("request"), tc.request_timeout);
        assert_eq!(tc.operation_timeout("read"), tc.request_timeout);
        assert_eq!(tc.operation_timeout("connect"), tc.connection_timeout);
        assert_eq!(tc.operation_timeout("keepalive"), tc.keep_alive_timeout);
        assert_eq!(tc.operation_timeout("unknown"), tc.request_timeout);
    }

    #[test]
    fn timeout_config_should_timeout_and_remaining() {
        let tc = TimeoutConfig::default();
        assert!(!tc.should_timeout(Duration::from_secs(1), "request"));
        assert!(tc.should_timeout(Duration::from_secs(120), "request"));
        assert_eq!(
            tc.remaining_time(Duration::from_secs(5), "request"),
            tc.request_timeout
                .checked_sub(Duration::from_secs(5))
                .unwrap()
        );
    }

    #[test]
    fn timeout_config_validate_errors() {
        let mut tc = TimeoutConfig::default();
        tc.connection_timeout = Duration::ZERO;
        assert!(tc.validate().is_err());
        tc.connection_timeout = Duration::from_secs(30);
        tc.request_timeout = Duration::ZERO;
        assert!(tc.validate().is_err());
    }

    #[test]
    fn timeout_config_is_production_ready() {
        let mut tc = TimeoutConfig::default();
        assert!(tc.is_production_ready());
        tc.connection_timeout = Duration::from_secs(0);
        assert!(!tc.is_production_ready());
    }

    #[test]
    fn network_config_validate_port_and_tls() {
        let mut nc = NetworkConfig::default();
        nc.tls_enabled = false;
        assert!(nc.validate().is_ok());

        nc.port = 0;
        assert!(nc.validate().unwrap_err().contains("Port"));

        nc.port = DEFAULT_API_PORT;
        nc.tls_enabled = true;
        assert!(nc.validate().unwrap_err().contains("certificate"));

        nc.tls_cert_path = Some("/path/cert.pem".to_string());
        assert!(nc.validate().unwrap_err().contains("key"));

        nc.tls_key_path = Some("/path/key.pem".to_string());
        assert!(nc.validate().is_ok());

        nc.connection_pool.max_size = 0;
        assert!(nc.validate().unwrap_err().contains("pool"));
    }
}
