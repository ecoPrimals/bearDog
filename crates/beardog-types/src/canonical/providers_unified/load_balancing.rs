// SPDX-License-Identifier: AGPL-3.0-or-later

// Load Balancing Configuration
//
// Load balancing strategies and configuration for providers.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Load balancing strategy
    /// The strategy value
    pub strategy: LoadBalancingStrategy,

    /// Health check configuration
    /// The health check value
    pub health_check: LoadBalancerHealthConfig,

    /// Session affinity enabled
    /// Whether `session_affinity` is enabled
    pub session_affinity: bool,

    /// Weighted routing enabled
    /// Whether `weighted_routing` is enabled
    pub weighted_routing: bool,

    /// Failover enabled
    /// Whether failover is enabled
    pub failover_enabled: bool,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check: LoadBalancerHealthConfig::default(),
            session_affinity: false,
            weighted_routing: false,
            failover_enabled: true,
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// `RoundRobin` variant
    RoundRobin,
    /// `LeastConnections` variant
    LeastConnections,
    /// `WeightedRoundRobin` variant
    WeightedRoundRobin,
    /// Random variant
    Random,
    /// `IpHash` variant
    IpHash,
    /// Custom load balancing algorithm
    Custom(String),
}

/// Load balancer health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerHealthConfig {
    /// Health check enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Health check interval
    /// The interval value
    pub interval: Duration,

    /// Health check timeout
    pub timeout: Duration,

    /// Unhealthy threshold
    /// Number of `unhealthy_threshold`
    pub unhealthy_threshold: u32,

    /// Healthy threshold
    /// Number of `healthy_threshold`
    pub healthy_threshold: u32,
}

impl Default for LoadBalancerHealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            unhealthy_threshold: 3,
            healthy_threshold: 2,
        }
    }
}
