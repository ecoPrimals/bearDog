// Service configuration types for BearDog
// Provides service discovery, registration, and management types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

/// Service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    /// Service name
    /// Name of the item
    pub name: String,
    /// Service version
    /// The version value
    pub version: String,
    /// Service description
    /// The description value
    pub description: String,
    /// Service endpoints
    /// Collection of endpoints
    pub endpoints: Vec<ServiceEndpoint>,
    /// Service metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Service tags
    /// Collection of tags
    pub tags: Vec<String>,
}

impl Default for ServiceDefinition {
    fn default() -> Self {
        Self {
            name: "default-service".to_string(),
            version: "1.0.0".to_string(),
            description: "Default service".to_string(),
            endpoints: Vec::new(),
            metadata: HashMap::new(),
            tags: Vec::new(),
        }
    }
}

/// Service endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint name
    /// Name of the item
    pub name: String,
    /// Endpoint address
    /// The address value
    pub address: SocketAddr,
    /// Endpoint protocol
    /// The protocol value
    pub protocol: String,
    /// Endpoint path
    /// The path value
    pub path: String,
    /// Health check path
    /// Optional health check path
    pub health_check_path: Option<String>,
    /// Endpoint metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        // Use network configuration for default endpoint
        let network_config = crate::canonical::config::network::NetworkConfig::default();
        let default_address = format!(
            "{}:{}",
            network_config.default_host, network_config.service_ports.api_port
        );

        Self {
            name: "default".to_string(),
            address: default_address.parse().unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse default address {}, using fallback",
                    default_address
                );
                "127.0.0.1:8080".parse().unwrap_or_else(|_| {
                    // This should never fail, but if it does, we create a minimal valid address
                    std::net::SocketAddr::new(
                        std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                        8080,
                    )
                })
            }),
            protocol: "http".to_string(),
            path: "/".to_string(),
            health_check_path: Some("/health".to_string()),
            metadata: HashMap::new(),
        }
    }
}

/// Service registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistry {
    /// Registry name
    /// Name of the item
    pub name: String,
    /// Registry type
    /// The registry type value
    pub registry_type: RegistryType,
    /// Registry endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Authentication configuration
    /// Optional auth
    pub auth: Option<RegistryAuth>,
    /// Service TTL in seconds
    /// Number of service_ttl_seconds
    pub service_ttl_seconds: u64,
    /// Health check interval in seconds
    /// Number of health_check_interval_seconds
    pub health_check_interval_seconds: u64,
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self {
            name: "default-registry".to_string(),
            registry_type: RegistryType::Memory,
            endpoint: "localhost:8500".to_string(),
            auth: None,
            service_ttl_seconds: 300,
            health_check_interval_seconds: 30,
        }
    }
}

/// Registry types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of registry
pub enum RegistryType {
    /// In-memory registry
    Memory,
    /// Key-value service registry (replaces Consul/etcd hardcoding)
    KeyValueRegistry,
    /// Container orchestration service registry (replaces Kubernetes hardcoding)
    ContainerOrchestrationRegistry,
    /// Custom registry
    Custom { name: String },
    Custom { name: String },
    Custom { name: String },
}

/// Registry authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryAuth {
    /// Authentication type
    /// The auth type value
    pub auth_type: String,
    /// Username
    /// Name of the useritem
    pub username: Option<String>,
    /// Password
    /// Optional password
    pub password: Option<String>,
    /// Token
    /// Optional token
    pub token: Option<String>,
    /// Additional auth parameters
    /// Mapping of params
    pub params: HashMap<String, String>,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscovery {
    /// Discovery enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Service registries
    /// Collection of registries
    pub registries: Vec<ServiceRegistry>,
    /// Discovery interval in seconds
    /// Number of discovery_interval_seconds
    pub discovery_interval_seconds: u64,
    /// Service cache TTL in seconds
    /// Number of cache_ttl_seconds
    pub cache_ttl_seconds: u64,
    /// Load balancing strategy
    /// The load balancing value
    pub load_balancing: LoadBalancingStrategy,
}

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self {
            enabled: true,
            registries: vec![ServiceRegistry::default()],
            discovery_interval_seconds: 30,
            cache_ttl_seconds: 300,
            load_balancing: LoadBalancingStrategy::RoundRobin,
        }
    }
}

/// Load balancing strategies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin
    RoundRobin,
    /// Random selection
    Random,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
}

/// Service health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceHealth {
    /// Service is healthy
    Healthy,
    /// Service is unhealthy
    Unhealthy,
    /// Service health is unknown
    Unknown,
    /// Service is in maintenance mode
    Maintenance,
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Service instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    /// Instance ID
    pub id: String,
    /// Service definition
    /// The service value
    pub service: ServiceDefinition,
    /// Instance health
    /// The health value
    pub health: ServiceHealth,
    /// Last health check timestamp
    /// Optional last health check
    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    /// Instance metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Instance tags
    /// Collection of tags
    pub tags: Vec<String>,
}

impl Default for ServiceInstance {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            service: ServiceDefinition::default(),
            health: ServiceHealth::Unknown,
            last_health_check: None,
            metadata: HashMap::new(),
            tags: Vec::new(),
        }
    }
}

impl ServiceDefinition {
    /// Create a new service definition
    /// Creates a new instance
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            ..Default::default()
        }
    }

    /// Add an endpoint to the service
    pub fn add_endpoint(&mut self, endpoint: ServiceEndpoint) {
        self.endpoints.push(endpoint);
    }

    /// Add a tag to the service
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Sets metadata
    /// Sets metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get the primary endpoint
    pub fn primary_endpoint(&self) -> Option<&ServiceEndpoint> {
        self.endpoints.first()
    }

    /// Check if service has a specific tag
    /// Checks if tag
    /// Checks if tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

impl ServiceInstance {
    /// Create a new service instance
    /// Creates a new instance
    pub fn new(service: ServiceDefinition) -> Self {
        Self {
            service,
            ..Default::default()
        }
    }

    /// Update health status
    /// Updates health
    /// Updates health
    pub fn update_health(&mut self, health: ServiceHealth) {
        self.health = health;
        self.last_health_check = Some(chrono::Utc::now());
    }

    /// Check if instance is healthy
    /// Checks if healthy
    /// Checks if healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.health, ServiceHealth::Healthy)
    }

    /// Get service name
    pub fn service_name(&self) -> &str {
        &self.service.name
    }

    /// Get service version
    pub fn service_version(&self) -> &str {
        &self.service.version
    }
}
