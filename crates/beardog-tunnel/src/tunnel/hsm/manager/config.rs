// SPDX-License-Identifier: AGPL-3.0-or-later

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
            Self::Smartphone => write!(f, "Smartphone"),
            Self::Software => write!(f, "Software"),
            Self::Hardware => write!(f, "Hardware"),
            Self::Hybrid => write!(f, "Hybrid"),
        }
    }
}

/// HSM manager configuration
#[derive(Debug, Clone, Default)]
pub struct HsmManagerConfig {
    /// Health check configuration
    pub health_config: HealthConfig,
    /// Failover configuration
    pub failover_config: FailoverConfig,
    /// Performance configuration
    pub performance_config: PerformanceConfig,
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

    // ========================================================================
    // COMPREHENSIVE CONFIGURATION TESTS (Final Coverage Push)
    // ========================================================================

    #[test]
    fn test_all_hsm_tiers() {
        // Test all tier types
        let tiers = [
            SimpleHsmTier::Smartphone,
            SimpleHsmTier::Software,
            SimpleHsmTier::Hardware,
            SimpleHsmTier::Hybrid,
        ];

        assert_eq!(tiers.len(), 4);

        // Verify each tier is distinct
        for i in 0..tiers.len() {
            for j in i + 1..tiers.len() {
                assert_ne!(tiers[i], tiers[j]);
            }
        }
    }

    #[test]
    fn test_hsm_tier_clone() {
        let tier1 = SimpleHsmTier::Hardware;
        let tier2 = tier1;
        assert_eq!(tier1, tier2);
    }

    #[test]
    fn test_health_config_clone() {
        let config1 = HealthConfig {
            check_interval: Duration::from_secs(60),
            failure_threshold: 5,
            recovery_threshold: 10,
            timeout: Duration::from_secs(10),
        };

        let config2 = config1.clone();

        assert_eq!(config1.check_interval, config2.check_interval);
        assert_eq!(config1.failure_threshold, config2.failure_threshold);
        assert_eq!(config1.recovery_threshold, config2.recovery_threshold);
        assert_eq!(config1.timeout, config2.timeout);
    }

    #[test]
    fn test_failover_config_clone() {
        let config1 = FailoverConfig {
            enabled: false,
            max_retries: 5,
            retry_delay: Duration::from_secs(1),
            circuit_breaker_threshold: 10,
            circuit_breaker_timeout: Duration::from_secs(120),
        };

        let config2 = config1.clone();

        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.max_retries, config2.max_retries);
        assert_eq!(config1.retry_delay, config2.retry_delay);
        assert_eq!(
            config1.circuit_breaker_threshold,
            config2.circuit_breaker_threshold
        );
        assert_eq!(
            config1.circuit_breaker_timeout,
            config2.circuit_breaker_timeout
        );
    }

    #[test]
    fn test_performance_config_clone() {
        let config1 = PerformanceConfig {
            enable_caching: false,
            max_concurrent_operations: 50,
            operation_timeout: Duration::from_secs(60),
        };

        let config2 = config1.clone();

        assert_eq!(config1.enable_caching, config2.enable_caching);
        assert_eq!(
            config1.max_concurrent_operations,
            config2.max_concurrent_operations
        );
        assert_eq!(config1.operation_timeout, config2.operation_timeout);
    }

    #[test]
    fn test_hsm_manager_config_clone() {
        let config1 = HsmManagerConfig::default();
        let config2 = config1.clone();

        assert_eq!(
            config1.health_config.check_interval,
            config2.health_config.check_interval
        );
        assert_eq!(
            config1.failover_config.enabled,
            config2.failover_config.enabled
        );
        assert_eq!(
            config1.performance_config.enable_caching,
            config2.performance_config.enable_caching
        );
    }

    #[test]
    fn test_health_config_custom() {
        let config = HealthConfig {
            check_interval: Duration::from_secs(120),
            failure_threshold: 10,
            recovery_threshold: 15,
            timeout: Duration::from_secs(15),
        };

        assert_eq!(config.check_interval, Duration::from_secs(120));
        assert_eq!(config.failure_threshold, 10);
        assert_eq!(config.recovery_threshold, 15);
        assert_eq!(config.timeout, Duration::from_secs(15));
    }

    #[test]
    fn test_failover_config_custom() {
        let config = FailoverConfig {
            enabled: false,
            max_retries: 10,
            retry_delay: Duration::from_secs(5),
            circuit_breaker_threshold: 20,
            circuit_breaker_timeout: Duration::from_secs(300),
        };

        assert!(!config.enabled);
        assert_eq!(config.max_retries, 10);
        assert_eq!(config.retry_delay, Duration::from_secs(5));
        assert_eq!(config.circuit_breaker_threshold, 20);
        assert_eq!(config.circuit_breaker_timeout, Duration::from_secs(300));
    }

    #[test]
    fn test_performance_config_custom() {
        let config = PerformanceConfig {
            enable_caching: false,
            max_concurrent_operations: 200,
            operation_timeout: Duration::from_secs(120),
        };

        assert!(!config.enable_caching);
        assert_eq!(config.max_concurrent_operations, 200);
        assert_eq!(config.operation_timeout, Duration::from_secs(120));
    }

    #[test]
    fn test_hsm_manager_config_custom() {
        let health_config = HealthConfig {
            check_interval: Duration::from_secs(45),
            failure_threshold: 4,
            recovery_threshold: 6,
            timeout: Duration::from_secs(8),
        };

        let failover_config = FailoverConfig {
            enabled: true,
            max_retries: 5,
            retry_delay: Duration::from_millis(200),
            circuit_breaker_threshold: 8,
            circuit_breaker_timeout: Duration::from_secs(90),
        };

        let performance_config = PerformanceConfig {
            enable_caching: true,
            max_concurrent_operations: 150,
            operation_timeout: Duration::from_secs(45),
        };

        let config = HsmManagerConfig {
            health_config,
            failover_config,
            performance_config,
        };

        assert_eq!(config.health_config.check_interval, Duration::from_secs(45));
        assert!(config.failover_config.enabled);
        assert!(config.performance_config.enable_caching);
    }

    #[test]
    fn test_hsm_tier_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(SimpleHsmTier::Hardware, "hardware");
        map.insert(SimpleHsmTier::Software, "software");
        map.insert(SimpleHsmTier::Smartphone, "smartphone");
        map.insert(SimpleHsmTier::Hybrid, "hybrid");

        assert_eq!(map.get(&SimpleHsmTier::Hardware), Some(&"hardware"));
        assert_eq!(map.get(&SimpleHsmTier::Software), Some(&"software"));
        assert_eq!(map.get(&SimpleHsmTier::Smartphone), Some(&"smartphone"));
        assert_eq!(map.get(&SimpleHsmTier::Hybrid), Some(&"hybrid"));
    }

    #[test]
    fn test_health_config_edge_cases() {
        // Test with minimum values
        let config = HealthConfig {
            check_interval: Duration::from_millis(1),
            failure_threshold: 1,
            recovery_threshold: 1,
            timeout: Duration::from_millis(1),
        };

        assert_eq!(config.check_interval, Duration::from_millis(1));
        assert_eq!(config.failure_threshold, 1);
        assert_eq!(config.recovery_threshold, 1);
        assert_eq!(config.timeout, Duration::from_millis(1));
    }

    #[test]
    fn test_failover_config_edge_cases() {
        // Test with zero retries
        let config = FailoverConfig {
            enabled: false,
            max_retries: 0,
            retry_delay: Duration::ZERO,
            circuit_breaker_threshold: 0,
            circuit_breaker_timeout: Duration::ZERO,
        };

        assert!(!config.enabled);
        assert_eq!(config.max_retries, 0);
        assert_eq!(config.retry_delay, Duration::ZERO);
        assert_eq!(config.circuit_breaker_threshold, 0);
    }

    #[test]
    fn test_performance_config_edge_cases() {
        // Test with maximum values
        let config = PerformanceConfig {
            enable_caching: true,
            max_concurrent_operations: usize::MAX,
            operation_timeout: Duration::from_secs(3600),
        };

        assert!(config.enable_caching);
        assert_eq!(config.max_concurrent_operations, usize::MAX);
        assert_eq!(config.operation_timeout, Duration::from_secs(3600));
    }

    #[test]
    fn test_tier_serde() {
        // Test serialization/deserialization
        let tier = SimpleHsmTier::Hardware;
        let serialized = serde_json::to_string(&tier).expect("SimpleHsmTier should serialize");
        let deserialized: SimpleHsmTier =
            serde_json::from_str(&serialized).expect("SimpleHsmTier round-trip should deserialize");
        assert_eq!(tier, deserialized);
    }

    #[test]
    fn test_all_tiers_serde() {
        // Test all tier variants can be serialized/deserialized
        let tiers = vec![
            SimpleHsmTier::Smartphone,
            SimpleHsmTier::Software,
            SimpleHsmTier::Hardware,
            SimpleHsmTier::Hybrid,
        ];

        for tier in tiers {
            let serialized = serde_json::to_string(&tier).expect("SimpleHsmTier should serialize");
            let deserialized: SimpleHsmTier = serde_json::from_str(&serialized)
                .expect("SimpleHsmTier round-trip should deserialize");
            assert_eq!(tier, deserialized);
        }
    }
}
