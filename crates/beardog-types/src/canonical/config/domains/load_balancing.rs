// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical Load Balancing Configuration
//!
//! This module provides the unified, canonical load balancing configuration
//! for all BearDog components (network, discovery, HSM, etc.).
//!
//! ## Design Principles:
//! - **Single Source of Truth**: One LoadBalancingConfig across all domains
//! - **Algorithm Flexibility**: Support for round-robin, weighted, least-connections, etc.
//! - **Health-Aware**: Integrates with health checks for intelligent routing
//! - **Zero-Cost Abstractions**: Efficient configuration with no runtime overhead
//!
//! ## Usage:
//!
//! ```rust
//! use beardog_types::canonical::config::domains::load_balancing::{
//!     CanonicalLoadBalancingConfig, LoadBalancingAlgorithm
//! };
//!
//! let config = CanonicalLoadBalancingConfig {
//!     enabled: true,
//!     algorithm: LoadBalancingAlgorithm::RoundRobin,
//!     health_check_interval_secs: 30,
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical load balancing configuration for all BearDog components
///
/// This is the single, unified load balancing configuration.
/// All domain-specific LoadBalancingConfigs should migrate to this.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalLoadBalancingConfig {
    /// Whether load balancing is enabled
    pub enabled: bool,

    /// Load balancing algorithm to use
    pub algorithm: LoadBalancingAlgorithm,

    /// Minimum number of healthy endpoints required
    pub min_healthy_endpoints: usize,

    /// Health check interval in seconds
    pub health_check_interval_secs: u64,

    /// Timeout for health checks
    pub health_check_timeout: Duration,

    /// Whether to enable sticky sessions (session affinity)
    pub sticky_sessions_enabled: bool,

    /// Session timeout for sticky sessions
    pub session_timeout: Duration,

    /// Weights for weighted algorithms (endpoint_id -> weight)
    pub weights: std::collections::HashMap<String, u32>,
}

/// Type alias for convenience (follows BearDog naming conventions)
pub type LoadBalancingConfig = CanonicalLoadBalancingConfig;

/// Load balancing algorithm selection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LoadBalancingAlgorithm {
    /// Round-robin: Distribute requests evenly across all endpoints
    RoundRobin,

    /// Weighted round-robin: Distribute based on endpoint weights
    WeightedRoundRobin,

    /// Least connections: Route to endpoint with fewest active connections
    LeastConnections,

    /// Random: Randomly select an endpoint
    Random,

    /// IP hash: Route based on client IP for session affinity
    IpHash,

    /// Consistent hashing: Distribute based on consistent hash ring
    ConsistentHash,

    /// Priority-based: Route to highest priority available endpoint
    Priority,
}

impl Default for CanonicalLoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            min_healthy_endpoints: 1,
            health_check_interval_secs: 30,
            health_check_timeout: Duration::from_secs(5),
            sticky_sessions_enabled: false,
            session_timeout: Duration::from_secs(300), // 5 minutes
            weights: std::collections::HashMap::new(),
        }
    }
}

impl LoadBalancingAlgorithm {
    /// Default load balancing algorithm
    pub const DEFAULT: Self = Self::RoundRobin;

    /// Create LoadBalancingAlgorithm with hardcoded defaults
    pub fn with_defaults() -> Self {
        Self::DEFAULT
    }

    /// Create LoadBalancingAlgorithm from environment variables
    pub fn from_env() -> Self {
        std::env::var("BEARDOG_LOAD_BALANCING_ALGORITHM")
            .ok()
            .and_then(|s| match s.to_lowercase().as_str() {
                "roundrobin" | "round_robin" => Some(Self::RoundRobin),
                "leastconnections" | "least_connections" => Some(Self::LeastConnections),
                "weightedroundrobin" | "weighted_round_robin" => Some(Self::WeightedRoundRobin),
                "random" => Some(Self::Random),
                _ => None,
            })
            .unwrap_or(Self::DEFAULT)
    }
}

impl Default for LoadBalancingAlgorithm {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static LOAD_BAL_ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_config() {
        let config = CanonicalLoadBalancingConfig::default();
        assert!(config.enabled);
        assert_eq!(config.algorithm, LoadBalancingAlgorithm::RoundRobin);
        assert_eq!(config.min_healthy_endpoints, 1);
        assert_eq!(config.health_check_interval_secs, 30);
    }

    #[test]
    fn test_algorithm_serialization() {
        let algorithm = LoadBalancingAlgorithm::WeightedRoundRobin;
        let json = serde_json::to_string(&algorithm).expect("serialize load balancing algorithm");
        assert_eq!(json, "\"weighted_round_robin\"");

        let deserialized: LoadBalancingAlgorithm =
            serde_json::from_str(&json).expect("deserialize load balancing algorithm");
        assert_eq!(deserialized, algorithm);
    }

    #[test]
    fn test_config_serialization() {
        let config = CanonicalLoadBalancingConfig {
            enabled: true,
            algorithm: LoadBalancingAlgorithm::LeastConnections,
            min_healthy_endpoints: 2,
            health_check_interval_secs: 60,
            health_check_timeout: Duration::from_secs(10),
            sticky_sessions_enabled: true,
            session_timeout: Duration::from_secs(600),
            weights: [("endpoint1".to_string(), 10), ("endpoint2".to_string(), 5)]
                .iter()
                .cloned()
                .collect(),
        };

        let json = serde_json::to_string(&config).expect("serialize load balancing config");
        let deserialized: CanonicalLoadBalancingConfig =
            serde_json::from_str(&json).expect("deserialize load balancing config");
        assert_eq!(deserialized, config);
    }

    #[test]
    fn test_type_alias() {
        // Ensure type alias works correctly
        let _config: LoadBalancingConfig = CanonicalLoadBalancingConfig::default();
    }

    #[test]
    fn test_algorithm_from_env_recognizes_aliases() {
        let _guard = LOAD_BAL_ENV_LOCK
            .lock()
            .expect("serialize load balancing env tests");

        std::env::remove_var("BEARDOG_LOAD_BALANCING_ALGORITHM");
        assert_eq!(
            LoadBalancingAlgorithm::from_env(),
            LoadBalancingAlgorithm::RoundRobin
        );

        std::env::set_var("BEARDOG_LOAD_BALANCING_ALGORITHM", "least_connections");
        assert_eq!(
            LoadBalancingAlgorithm::from_env(),
            LoadBalancingAlgorithm::LeastConnections
        );

        std::env::set_var("BEARDOG_LOAD_BALANCING_ALGORITHM", "bogus_unknown_value");
        assert_eq!(
            LoadBalancingAlgorithm::from_env(),
            LoadBalancingAlgorithm::RoundRobin
        );

        std::env::remove_var("BEARDOG_LOAD_BALANCING_ALGORITHM");
    }
}

