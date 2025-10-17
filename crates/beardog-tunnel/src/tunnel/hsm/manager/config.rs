//! HSM Manager Configuration
//!
//! Configuration types for HSM manager functionality.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Simple HSM tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimpleHsmTier {
    /// Smartphone HSM (iOS/Android)
    Smartphone,
    /// Software HSM
    Software,
    /// Hardware HSM
    Hardware,
    /// Hybrid HSM
    Hybrid,
}

impl std::fmt::Display for SimpleHsmTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleHsmTier::Smartphone => write!(f, "Smartphone"),
            SimpleHsmTier::Software => write!(f, "Software"),
            SimpleHsmTier::Hardware => write!(f, "Hardware"),
            SimpleHsmTier::Hybrid => write!(f, "Hybrid"),
        }
    }
}

/// HSM manager configuration
#[derive(Debug, Clone)]
pub struct HsmManagerConfig {
    /// Health check configuration
    pub health_config: HealthConfig,
    /// Failover configuration
    pub failover_config: FailoverConfig,
    /// Performance configuration
    pub performance_config: PerformanceConfig,
}

impl Default for HsmManagerConfig {
    fn default() -> Self {
        Self {
            health_config: HealthConfig::default(),
            failover_config: FailoverConfig::default(),
            performance_config: PerformanceConfig::default(),
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthConfig {
    /// Health check interval
    pub check_interval: Duration,
    /// Failure threshold before marking unhealthy
    pub failure_threshold: u32,
    /// Recovery threshold before marking healthy
    pub recovery_threshold: u32,
    /// Health check timeout
    pub timeout: Duration,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 5,
            timeout: Duration::from_secs(5),
        }
    }
}

/// Failover configuration
#[derive(Debug, Clone)]
pub struct FailoverConfig {
    /// Enable failover
    pub enabled: bool,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Circuit breaker threshold
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker timeout
    pub circuit_breaker_timeout: Duration,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(60),
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable operation caching
    pub enable_caching: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Operation timeout
    pub operation_timeout: Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(30),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hsm_tier_display() {
        assert_eq!(SimpleHsmTier::Smartphone.to_string(), "Smartphone");
        assert_eq!(SimpleHsmTier::Software.to_string(), "Software");
        assert_eq!(SimpleHsmTier::Hardware.to_string(), "Hardware");
        assert_eq!(SimpleHsmTier::Hybrid.to_string(), "Hybrid");
    }

    #[test]
    fn test_hsm_manager_config_default() {
        let config = HsmManagerConfig::default();
        assert_eq!(config.health_config.check_interval, Duration::from_secs(30));
        assert!(config.failover_config.enabled);
        assert!(config.performance_config.enable_caching);
    }

    #[test]
    fn test_health_config_default() {
        let config = HealthConfig::default();
        assert_eq!(config.check_interval, Duration::from_secs(30));
        assert_eq!(config.failure_threshold, 3);
        assert_eq!(config.recovery_threshold, 5);
        assert_eq!(config.timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_failover_config_default() {
        let config = FailoverConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, Duration::from_millis(100));
        assert_eq!(config.circuit_breaker_threshold, 5);
    }

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.enable_caching);
        assert_eq!(config.max_concurrent_operations, 100);
        assert_eq!(config.operation_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_simple_hsm_tier_equality() {
        assert_eq!(SimpleHsmTier::Software, SimpleHsmTier::Software);
        assert_ne!(SimpleHsmTier::Software, SimpleHsmTier::Hardware);
    }
}
