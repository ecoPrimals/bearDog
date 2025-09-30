//! # Connection Configuration Module
//!
//! This module contains connection management configurations including connection pooling,
//! timeouts, and load balancing.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Connection pool configuration - consolidates ConnectionPoolConfig variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfiguration {
    /// Maximum connections in pool
    pub max_size: usize,
    /// Minimum connections in pool
    pub min_size: usize,
    /// Connection idle timeout seconds
    pub idle_timeout_seconds: u64,
    /// Maximum connection lifetime seconds
    pub max_lifetime_seconds: u64,
    /// Connection acquire timeout seconds
    pub acquire_timeout_seconds: u64,
    /// Enable connection validation
    pub enable_validation: bool,
    /// Validation query/check
    pub validation_query: Option<String>,
    /// Pool maintenance interval seconds
    pub maintenance_interval_seconds: u64,
}

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

impl Default for ConnectionPoolConfiguration {
    fn default() -> Self {
        Self {
            max_size: 100,
            min_size: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            idle_timeout_seconds: 600, // 10 minutes
            max_lifetime_seconds: 1800, // 30 minutes
            acquire_timeout_seconds: 30,
            enable_validation: true,
            validation_query: Some("SELECT 1".to_string()),
            maintenance_interval_seconds: 60,
        }
    }
}

impl Default for TimeoutConfiguration {
    fn default() -> Self {
        Self {
            connection_timeout_seconds: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_secs(),
            request_timeout_seconds: beardog_types::constants::domains::network::timeouts::DEFAULT_REQUEST_TIMEOUT.as_secs(),
            keepalive_timeout_seconds: 60,
            dns_timeout_seconds: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
            tls_handshake_timeout_seconds: 30,
            read_timeout_seconds: 30,
            write_timeout_seconds: 30,
            shutdown_timeout_seconds: 30,
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
            interval_seconds: 30,
            timeout_seconds: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
            unhealthy_threshold: 3,
            healthy_threshold: 2,
            path: "/health".to_string(),
            expected_codes: vec![200, 201, 202],
        }
    }
}

impl Default for CircuitBreakerConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
            recovery_timeout_seconds: 60,
            half_open_max_calls: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
            minimum_throughput: 20,
        }
    }
}

impl Default for FailoverConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_timeout_seconds: 30,
            max_attempts: 3,
            backoff_seconds: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64,
        }
    }
}

impl ConnectionPoolConfiguration {
    /// Validate connection pool configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.max_size < self.min_size {
            return Err(BearDogError::configuration("Pool max size cannot be less than min size"));
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
            return Err(BearDogError::configuration("Connection timeout cannot be zero"));
        }
        
        if self.request_timeout_seconds == 0 {
            return Err(BearDogError::configuration("Request timeout cannot be zero"));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_validation() {
        let config = ConnectionPoolConfiguration::default();
        assert!(config.validate().is_ok());
        
        let mut invalid_config = config.clone();
        invalid_config.max_size = 5;
        invalid_config.min_size = 10;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_timeout_validation() {
        let config = TimeoutConfiguration::default();
        assert!(config.validate().is_ok());
        
        let mut invalid_config = config.clone();
        invalid_config.connection_timeout_seconds = 0;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_load_balancer_defaults() {
        let config = LoadBalancerConfiguration::default();
        assert!(matches!(config.algorithm, LoadBalancingAlgorithm::RoundRobin));
        assert!(config.circuit_breaker.enabled);
        assert!(config.failover.enabled);
    }
} 