//! Node information and registration types
//!
//! This module contains all types related to node information, registration,
//! and node type management within the registry system.

use std::collections::HashMap;
use std::time::SystemTime;

use crate::BearDogResult;
use super::trust::TrustLevel;

/// Node information structure
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Unique node identifier
    pub id: String,
    /// Node display name
    pub name: String,
    /// Node type (e.g., "security", "phonebook", "compute")
    pub node_type: String,
    /// Node version
    pub version: String,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Node endpoints
    pub endpoints: Vec<String>,
    /// Node public key
    pub public_key: Vec<u8>,
    /// Geographic region
    pub region: String,
    /// Registration timestamp
    pub registered_at: SystemTime,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Trust level
    pub trust_level: TrustLevel,
    /// Node metadata
    pub metadata: HashMap<String, String>,
}

impl NodeInfo {
    /// Create a new node info
    pub fn new(id: String, name: String, node_type: String) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            name,
            node_type,
            version: "1.0.0".to_string(),
            capabilities: Vec::new(),
            endpoints: Vec::new(),
            public_key: Vec::new(),
            region: "default".to_string(),
            registered_at: now,
            last_seen: now,
            trust_level: TrustLevel::Unknown,
            metadata: HashMap::new(),
        }
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    /// Check if node is online (last seen within threshold)
    pub fn is_online(&self, threshold_seconds: u64) -> bool {
        if let Ok(elapsed) = self.last_seen.elapsed() {
            elapsed.as_secs() <= threshold_seconds
        } else {
            false
        }
    }

    /// Add capability
    pub fn add_capability(&mut self, capability: String) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    /// Remove capability
    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    /// Check if node has capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())
    }

    /// Add endpoint
    pub fn add_endpoint(&mut self, endpoint: String) {
        if !self.endpoints.contains(&endpoint) {
            self.endpoints.push(endpoint);
        }
    }

    /// Remove endpoint
    pub fn remove_endpoint(&mut self, endpoint: &str) {
        self.endpoints.retain(|e| e != endpoint);
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Update trust level
    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
    }

    /// Validate node information
    pub fn validate(&self) -> BearDogResult<()> {
        if self.id.is_empty() {
            return Err(crate::error::BearDogError::validation("id", "Node ID cannot be empty"));
        }

        if self.name.is_empty() {
            return Err(crate::error::BearDogError::validation("name", "Node name cannot be empty"));
        }

        if self.node_type.is_empty() {
            return Err(crate::error::BearDogError::validation("node_type", "Node type cannot be empty"));
        }

        if self.endpoints.is_empty() {
            return Err(crate::error::BearDogError::validation("endpoints", "Node must have at least one endpoint"));
        }

        if self.public_key.is_empty() {
            return Err(crate::error::BearDogError::validation("public_key", "Node must have a public key"));
        }

        Ok(())
    }
}

/// Node registration request
#[derive(Debug, Clone)]
pub struct NodeRegistration {
    /// Node information
    pub node_info: NodeInfo,
    /// Registration proof (signature, etc.)
    pub proof: Vec<u8>,
}

impl NodeRegistration {
    /// Create a new node registration
    pub fn new(node_info: NodeInfo, proof: Vec<u8>) -> Self {
        Self { node_info, proof }
    }

    /// Validate registration proof
    pub fn validate_proof(&self) -> BearDogResult<bool> {
        // Placeholder for proof validation logic
        // In a real implementation, this would verify cryptographic signatures
        Ok(!self.proof.is_empty())
    }
}

/// Node entry in the registry
#[derive(Debug, Clone)]
pub struct NodeEntry {
    /// Node information
    pub info: NodeInfo,
    /// Registration timestamp
    pub registered_at: SystemTime,
    /// Node status
    pub status: NodeStatus,
    /// Health check results
    pub health_check_results: Vec<HealthCheckResult>,
}

impl NodeEntry {
    /// Create a new node entry
    pub fn new(info: NodeInfo) -> Self {
        Self {
            info,
            registered_at: SystemTime::now(),
            status: NodeStatus::Active,
            health_check_results: Vec::new(),
        }
    }

    /// Update node status
    pub fn update_status(&mut self, status: NodeStatus) {
        self.status = status;
    }

    /// Add health check result
    pub fn add_health_check_result(&mut self, result: HealthCheckResult) {
        self.health_check_results.push(result);
        // Keep only recent results
        if self.health_check_results.len() > 10 {
            self.health_check_results.remove(0);
        }
    }

    /// Get latest health check result
    pub fn latest_health_check(&self) -> Option<&HealthCheckResult> {
        self.health_check_results.last()
    }

    /// Check if node is healthy
    pub fn is_healthy(&self) -> bool {
        match self.status {
            NodeStatus::Active => {
                if let Some(health_check) = self.latest_health_check() {
                    health_check.is_healthy
                } else {
                    true // No health check results yet, assume healthy
                }
            }
            _ => false,
        }
    }
}

/// Node status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeStatus {
    /// Node is active and healthy
    Active,
    /// Node is inactive or offline
    Inactive,
    /// Node is temporarily suspended
    Suspended,
    /// Node is permanently banned
    Banned,
    /// Node is in maintenance mode
    Maintenance,
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::Active => write!(f, "Active"),
            NodeStatus::Inactive => write!(f, "Inactive"),
            NodeStatus::Suspended => write!(f, "Suspended"),
            NodeStatus::Banned => write!(f, "Banned"),
            NodeStatus::Maintenance => write!(f, "Maintenance"),
        }
    }
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Timestamp of the health check
    pub timestamp: SystemTime,
    /// Whether the node is healthy
    pub is_healthy: bool,
    /// Health check message
    pub message: String,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Additional health metrics
    pub metrics: HashMap<String, String>,
}

impl HealthCheckResult {
    /// Create a healthy result
    pub fn healthy(response_time_ms: u64) -> Self {
        Self {
            timestamp: SystemTime::now(),
            is_healthy: true,
            message: "Health check passed".to_string(),
            response_time_ms,
            metrics: HashMap::new(),
        }
    }

    /// Create an unhealthy result
    pub fn unhealthy(message: String, response_time_ms: u64) -> Self {
        Self {
            timestamp: SystemTime::now(),
            is_healthy: false,
            message,
            response_time_ms,
            metrics: HashMap::new(),
        }
    }

    /// Add metric
    pub fn with_metric(mut self, key: String, value: String) -> Self {
        self.metrics.insert(key, value);
        self
    }
}

/// Universal node type system that supports any node type through string-based identification
/// and capability-based registration. This system is fully extensible for future node types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeType {
    /// String identifier for the node type (e.g., "security", "phonebook", "compute")
    pub type_name: String,
    /// Human-readable display name
    pub display_name: String,
    /// Node type description
    pub description: String,
    /// Default capabilities for this node type
    pub default_capabilities: Vec<String>,
    /// Required capabilities for this node type
    pub required_capabilities: Vec<String>,
    /// Optional capabilities for this node type
    pub optional_capabilities: Vec<String>,
    /// Metadata for this node type
    pub metadata: HashMap<String, String>,
}

impl NodeType {
    /// Create a new node type
    pub fn new(
        type_name: String,
        display_name: String,
        description: String,
    ) -> Self {
        Self {
            type_name,
            display_name,
            description,
            default_capabilities: Vec::new(),
            required_capabilities: Vec::new(),
            optional_capabilities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add default capability
    pub fn with_default_capability(mut self, capability: String) -> Self {
        self.default_capabilities.push(capability);
        self
    }

    /// Add required capability
    pub fn with_required_capability(mut self, capability: String) -> Self {
        self.required_capabilities.push(capability);
        self
    }

    /// Add optional capability
    pub fn with_optional_capability(mut self, capability: String) -> Self {
        self.optional_capabilities.push(capability);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Validate node capabilities against this node type
    pub fn validate_capabilities(&self, capabilities: &[String]) -> BearDogResult<bool> {
        // Check that all required capabilities are present
        for required in &self.required_capabilities {
            if !capabilities.contains(required) {
                return Ok(false);
            }
        }

        // Check that all provided capabilities are either required or optional
        for capability in capabilities {
            if !self.required_capabilities.contains(capability) 
                && !self.optional_capabilities.contains(capability) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get all possible capabilities for this node type
    pub fn all_capabilities(&self) -> Vec<String> {
        let mut all_caps = self.required_capabilities.clone();
        all_caps.extend(self.optional_capabilities.clone());
        all_caps.sort();
        all_caps.dedup();
        all_caps
    }
}

/// Node type registry for managing supported node types
#[derive(Debug, Clone)]
pub struct NodeTypeRegistry {
    /// Registered node types
    pub types: HashMap<String, NodeType>,
}

impl NodeTypeRegistry {
    /// Create a new node type registry
    pub fn new() -> Self {
        let mut registry = Self {
            types: HashMap::new(),
        };

        // Register default node types
        registry.register_default_types();
        registry
    }

    /// Register a new node type
    pub fn register_type(&mut self, node_type: NodeType) {
        self.types.insert(node_type.type_name.clone(), node_type);
    }

    /// Get a node type by name
    pub fn get_type(&self, type_name: &str) -> Option<&NodeType> {
        self.types.get(type_name)
    }

    /// Check if a node type is registered
    pub fn is_registered(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    /// Get all registered node types
    pub fn get_all_types(&self) -> Vec<&NodeType> {
        self.types.values().collect()
    }

    /// Validate node against its type
    pub fn validate_node(&self, node: &NodeInfo) -> BearDogResult<bool> {
        if let Some(node_type) = self.get_type(&node.node_type) {
            node_type.validate_capabilities(&node.capabilities)
        } else {
            Ok(false) // Unknown node type
        }
    }

    /// Register default node types
    fn register_default_types(&mut self) {
        // Security node type
        let security_type = NodeType::new(
            "security".to_string(),
            "Security Node".to_string(),
            "Node providing security services".to_string(),
        )
        .with_required_capability("authentication".to_string())
        .with_required_capability("authorization".to_string())
        .with_optional_capability("encryption".to_string())
        .with_optional_capability("threat_detection".to_string());

        // Phonebook node type
        let phonebook_type = NodeType::new(
            "phonebook".to_string(),
            "Phonebook Node".to_string(),
            "Node providing discovery services".to_string(),
        )
        .with_required_capability("discovery".to_string())
        .with_optional_capability("caching".to_string());

        // Compute node type
        let compute_type = NodeType::new(
            "compute".to_string(),
            "Compute Node".to_string(),
            "Node providing computational services".to_string(),
        )
        .with_required_capability("computation".to_string())
        .with_optional_capability("gpu_acceleration".to_string())
        .with_optional_capability("parallel_processing".to_string());

        // Storage node type
        let storage_type = NodeType::new(
            "storage".to_string(),
            "Storage Node".to_string(),
            "Node providing storage services".to_string(),
        )
        .with_required_capability("storage".to_string())
        .with_optional_capability("replication".to_string())
        .with_optional_capability("backup".to_string());

        self.register_type(security_type);
        self.register_type(phonebook_type);
        self.register_type(compute_type);
        self.register_type(storage_type);
    }
}

impl Default for NodeTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard node type constants
pub mod node_types {
    /// Security service node type
    pub const SECURITY: &str = "security";
    /// Phonebook service node type
    pub const PHONEBOOK: &str = "phonebook";
    /// Federation service node type
    pub const FEDERATION: &str = "federation";
    /// Compute service node type
    pub const COMPUTE: &str = "compute";
    /// Storage service node type
    pub const STORAGE: &str = "storage";
    /// Relay service node type
    pub const RELAY: &str = "relay";
    /// Backup service node type
    pub const BACKUP: &str = "backup";
    /// Monitoring service node type
    pub const MONITORING: &str = "monitoring";
    /// Analytics service node type
    pub const ANALYTICS: &str = "analytics";
    /// Gateway service node type
    pub const GATEWAY: &str = "gateway";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_info_creation() {
        let node = NodeInfo::new(
            "test-node".to_string(),
            "Test Node".to_string(),
            "security".to_string(),
        );
        assert_eq!(node.id, "test-node");
        assert_eq!(node.name, "Test Node");
        assert_eq!(node.node_type, "security");
        assert_eq!(node.trust_level, TrustLevel::Unknown);
    }

    #[test]
    fn test_node_capabilities() {
        let mut node = NodeInfo::new(
            "test-node".to_string(),
            "Test Node".to_string(),
            "security".to_string(),
        );
        
        node.add_capability("authentication".to_string());
        assert!(node.has_capability("authentication"));
        
        node.remove_capability("authentication");
        assert!(!node.has_capability("authentication"));
    }

    #[test]
    fn test_node_type_validation() {
        let node_type = NodeType::new(
            "test".to_string(),
            "Test Type".to_string(),
            "Test node type".to_string(),
        )
        .with_required_capability("required_cap".to_string())
        .with_optional_capability("optional_cap".to_string());

        // Valid: has required capability
        assert!(node_type.validate_capabilities(&["required_cap".to_string()]).unwrap());
        
        // Invalid: missing required capability
        assert!(!node_type.validate_capabilities(&["optional_cap".to_string()]).unwrap());
        
        // Valid: has both required and optional
        assert!(node_type.validate_capabilities(&[
            "required_cap".to_string(),
            "optional_cap".to_string()
        ]).unwrap());
    }

    #[test]
    fn test_node_type_registry() {
        let registry = NodeTypeRegistry::new();
        
        // Default types should be registered
        assert!(registry.is_registered("security"));
        assert!(registry.is_registered("phonebook"));
        assert!(registry.is_registered("compute"));
        assert!(registry.is_registered("storage"));
        
        // Unknown type should not be registered
        assert!(!registry.is_registered("unknown"));
    }

    #[test]
    fn test_health_check_result() {
        let healthy = HealthCheckResult::healthy(100);
        assert!(healthy.is_healthy);
        assert_eq!(healthy.response_time_ms, 100);
        
        let unhealthy = HealthCheckResult::unhealthy("Error".to_string(), 1000);
        assert!(!unhealthy.is_healthy);
        assert_eq!(unhealthy.response_time_ms, 1000);
    }

    #[test]
    fn test_node_entry() {
        let node_info = NodeInfo::new(
            "test-node".to_string(),
            "Test Node".to_string(),
            "security".to_string(),
        );
        let mut entry = NodeEntry::new(node_info);
        
        assert_eq!(entry.status, NodeStatus::Active);
        assert!(entry.is_healthy()); // No health checks yet, assume healthy
        
        entry.add_health_check_result(HealthCheckResult::unhealthy("Error".to_string(), 1000));
        assert!(!entry.is_healthy());
    }
} 