

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedDiscoveryConfig {
    pub service_discovery: ServiceDiscoveryConfig,
    pub node_discovery: NodeDiscoveryConfig,
    pub health_checks: DiscoveryHealthConfig,
    pub load_balancing: DiscoveryLoadBalancingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub discovery_method: String,
    pub refresh_interval: Duration,
    pub timeout: Duration,
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_method: "dns".to_string(),
            refresh_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDiscoveryConfig {
    pub enabled: bool,
    pub discovery_port: u16,
    pub announcement_interval: Duration,
    pub node_timeout: Duration,
}

impl Default for NodeDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_port: 8080,
            announcement_interval: Duration::from_secs(60),
            node_timeout: Duration::from_secs(300),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryHealthConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
}

impl Default for DiscoveryHealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(10),
            healthy_threshold: 2,
            unhealthy_threshold: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryLoadBalancingConfig {
    pub enabled: bool,
    pub strategy: String,
    pub weight_adjustment: bool,
    pub sticky_sessions: bool,
}

impl Default for DiscoveryLoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: "round_robin".to_string(),
            weight_adjustment: true,
            sticky_sessions: false,
        }
    }
}
