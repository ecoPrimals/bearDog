//! Network Configuration Module
//!
//! Network and connectivity configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network and connectivity configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub enable_tls: bool,
    pub circuit_breaker: crate::canonical::providers::CircuitBreakerConfig,
    pub load_balancing: LoadBalancingConfig,
    pub bandwidth_mbps: u32,
    pub port_ranges: Vec<PortRange>,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    pub enabled: bool,
    pub strategy: LoadBalancingStrategy,
    pub health_check_interval: Duration,
    pub max_failures: u32,
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoadBalancingStrategy {
    #[default]
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
}

/// Port range configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
    pub protocol: NetworkProtocol,
}

/// Network protocol types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum NetworkProtocol {
    #[default]
    Tcp,
    Udp,
    Http,
    Https,
    Grpc,
}

impl NetworkConfig {
    /// Create a new network configuration
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            ..Default::default()
        }
    }

    /// Check if TLS is enabled
    pub fn is_tls_enabled(&self) -> bool {
        self.enable_tls
    }

    /// Get the full address
    pub fn full_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
