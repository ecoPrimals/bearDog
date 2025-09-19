// Discovery Configuration
//
// Service discovery and registry configuration for providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Discovery enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Discovery type
    /// The discovery type value
    pub discovery_type: DiscoveryType,

    /// Service registry endpoints
    /// Collection of registry endpoints
    pub registry_endpoints: Vec<String>,

    /// Service registration enabled
    /// Whether registration is enabled
    pub registration_enabled: bool,

    /// Health check interval
    /// The health check interval value
    pub health_check_interval: Duration,

    /// Service metadata
    /// Mapping of service metadata
    pub service_metadata: HashMap<String, String>,

    /// Discovery refresh interval
    /// The refresh interval value
    pub refresh_interval: Duration,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_type: DiscoveryType::KeyValueRegistry,
            registry_endpoints: vec!["localhost:8500".to_string()],
            registration_enabled: true,
            health_check_interval: Duration::from_secs(30),
            service_metadata: HashMap::new(),
            refresh_interval: Duration::from_secs(60),
        }
    }
}

/// Discovery types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of discovery
pub enum DiscoveryType {
    /// Key-value service discovery (replaces Consul/etcd hardcoding)
    KeyValueRegistry,
    /// Container orchestration discovery (replaces Kubernetes hardcoding)  
    ContainerOrchestrationRegistry,
    /// Eureka variant
    Eureka,
    /// Zookeeper variant
    Zookeeper,
    /// Custom discovery mechanism
    Custom(String),
}
