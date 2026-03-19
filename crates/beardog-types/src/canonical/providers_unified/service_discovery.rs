// SPDX-License-Identifier: AGPL-3.0-only

//! Service Discovery Configuration
//!
//! Service-registry-specific discovery configuration that extends the canonical base.

use super::super::config::domains::discovery::DiscoveryConfig;
use serde::{Deserialize, Serialize};

/// Service discovery configuration (wraps canonical base with registry-specific fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// Base discovery configuration
    pub base: DiscoveryConfig,

    /// Discovery type (service registry specific)
    pub discovery_type: DiscoveryType,
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            base: DiscoveryConfig::default(),
            discovery_type: DiscoveryType::KeyValueRegistry,
        }
    }
}

/// Discovery types for service registries
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryType {
    /// Key-value service discovery (Consul, etcd)
    KeyValueRegistry,
    /// Container orchestration discovery (Kubernetes)
    ContainerOrchestrationRegistry,
    /// Eureka service registry
    Eureka,
    /// Zookeeper service registry
    Zookeeper,
    /// Custom discovery mechanism
    Custom(String),
}

impl Default for DiscoveryType {
    fn default() -> Self {
        DiscoveryType::KeyValueRegistry
    }
}
