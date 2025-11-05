// Network configuration types for BearDog
// Provides network-related configuration structures and utilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
            bind_address: std::env::var("BEARDOG_NETWORK_BIND_ADDRESS")
                .or_else(|_| std::env::var("BEARDOG_BIND_ADDRESS"))
                .unwrap_or_else(|_| "0.0.0.0".to_string()), // Standard bind-to-all-interfaces
            port: std::env::var("BEARDOG_NETWORK_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
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
    /// Checks if tls configured
    pub fn is_tls_configured(&self) -> bool {
        self.tls_enabled && self.tls_cert_path.is_some() && self.tls_key_path.is_some()
    }

    /// Validate the network configuration
    /// Validates input
    /// Validates input
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
