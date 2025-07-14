//! Node Registry Types and Data Structures
//!
//! This module contains all the data structures, enums, and type definitions
//! used throughout the node registry and trust management system.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::{BearDogError, BearDogResult};

/// Trust level assigned to nodes in the registry.
///
/// Trust levels determine what operations nodes are allowed to perform
/// and how much confidence the system has in their behavior. Higher
/// trust levels unlock more sensitive operations and greater resource access.
///
/// ## Trust Level Progression
///
/// Nodes typically start at **Unknown** and progress through higher trust
/// levels as they demonstrate reliability and security. Trust can also be
/// explicitly assigned by administrators.
///
/// ## Security Implications
///
/// - **Unknown** nodes are heavily restricted and monitored
/// - **Basic** trust allows standard operations with rate limiting
/// - **Medium** trust enables moderately sensitive operations
/// - **High** trust enables sensitive operations and resource sharing
/// - **Explicit** trust grants near-administrative privileges
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// New or unverified nodes (trust level 0).
    ///
    /// These nodes have not yet proven their identity or reliability.
    /// They are allowed only basic operations and are subject to
    /// strict rate limiting and monitoring.
    Unknown = 0,

    /// Nodes that have passed basic verification (trust level 1).
    ///
    /// These nodes have successfully completed identity verification
    /// and basic security checks. They can perform standard operations
    /// but are still subject to some restrictions.
    Basic = 1,

    /// Nodes with moderate trust level (trust level 2).
    ///
    /// These nodes have demonstrated some reliability and are trusted
    /// with moderately sensitive operations.
    Medium = 2,

    /// Nodes with proven reliability and good reputation (trust level 3).
    ///
    /// These nodes have demonstrated consistent good behavior over time
    /// and have built up a positive reputation. They are trusted with
    /// more sensitive operations and greater resource access.
    High = 3,

    /// Nodes explicitly trusted by administrators (trust level 4).
    ///
    /// These nodes have been manually verified and approved by system
    /// administrators. They have the highest level of trust and can
    /// perform nearly any operation, including administrative tasks.
    Explicit = 4,
}

impl TrustLevel {
    /// Convert trust level to numeric value
    pub fn as_numeric(&self) -> u8 {
        *self as u8
    }

    /// Convert numeric value to trust level
    pub fn from_numeric(value: u8) -> Option<Self> {
        match value {
            0 => Some(TrustLevel::Unknown),
            1 => Some(TrustLevel::Basic),
            2 => Some(TrustLevel::Medium),
            3 => Some(TrustLevel::High),
            4 => Some(TrustLevel::Explicit),
            _ => None,
        }
    }

    /// Check if this trust level allows a specific operation
    pub fn allows_operation(&self, operation: &str) -> bool {
        match operation {
            "basic_read" => *self >= TrustLevel::Unknown,
            "standard_write" => *self >= TrustLevel::Basic,
            "sensitive_read" => *self >= TrustLevel::Medium,
            "sensitive_write" => *self >= TrustLevel::High,
            "admin_operation" => *self >= TrustLevel::Explicit,
            _ => false,
        }
    }
}

/// Information about a node in the registry.
///
/// Contains all metadata necessary to identify, communicate with,
/// and manage trust relationships with a node in the network.
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// The node's Ed25519 public key for cryptographic operations.
    ///
    /// This serves as the node's cryptographic identity and is used for:
    /// - Verifying digital signatures
    /// - Encrypting communications
    /// - Establishing secure channels
    pub public_key: Vec<u8>,

    /// Current trust level assigned to this node.
    ///
    /// This determines what operations the node is authorized to perform
    /// and how much confidence the system has in its behavior.
    pub trust_level: TrustLevel,

    /// List of capabilities this node offers.
    ///
    /// Examples include:
    /// - "storage" - Can store data for other nodes
    /// - "compute" - Can perform computational tasks
    /// - "relay" - Can relay network traffic
    /// - "backup" - Can provide backup services
    pub capabilities: Vec<String>,

    /// Network endpoint for communicating with this node.
    ///
    /// This should be a secure HTTPS URL, for example:
    /// "https://node.example.com:8443"
    pub endpoint: String,

    /// Timestamp of the last time this node was seen active.
    ///
    /// Used for:
    /// - Detecting inactive nodes
    /// - Implementing timeout policies
    /// - Health monitoring
    pub last_seen: SystemTime,

    /// Additional metadata about the node.
    ///
    /// Can include information like:
    /// - "version" - Software version
    /// - "region" - Geographic region
    /// - "operator" - Node operator information
    /// - "contact" - Contact information for the operator
    pub metadata: HashMap<String, String>,

    /// Unique identifier for this node
    pub node_id: String,

    /// Display name for this node
    pub display_name: String,

    /// Type of node (e.g., "beardog_security_node")
    pub node_type: String,

    /// Network address for this node
    pub network_address: String,

    /// Timestamp when this node was registered
    pub registration_timestamp: chrono::DateTime<chrono::Utc>,
}

impl NodeInfo {
    /// Create a new NodeInfo with basic defaults
    pub fn new(node_id: String, public_key: Vec<u8>, endpoint: String) -> Self {
        Self {
            public_key,
            trust_level: TrustLevel::Unknown,
            capabilities: vec!["basic".to_string()],
            endpoint,
            last_seen: SystemTime::now(),
            metadata: HashMap::new(),
            node_id: node_id.clone(),
            display_name: format!("Node: {}", node_id),
            node_type: "beardog_security_node".to_string(),
            network_address: String::new(),
            registration_timestamp: chrono::Utc::now(),
        }
    }

    /// Check if the node is currently active
    pub fn is_active(&self, timeout: Duration) -> bool {
        match self.last_seen.elapsed() {
            Ok(elapsed) => elapsed < timeout,
            Err(_) => false, // SystemTime error, consider inactive
        }
    }

    /// Update the last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    /// Check if node has a specific capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.contains(&capability.to_string())
    }

    /// Add a capability to the node
    pub fn add_capability(&mut self, capability: String) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    /// Remove a capability from the node
    pub fn remove_capability(&mut self, capability: &str) {
        self.capabilities.retain(|c| c != capability);
    }

    /// Validate node info
    pub fn validate(&self) -> BearDogResult<()> {
        if self.node_id.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "Node ID cannot be empty".to_string(),
            });
        }

        if self.public_key.len() != 32 {
            return Err(BearDogError::InvalidInput {
                message: format!(
                    "Public key must be 32 bytes, got {}",
                    self.public_key.len()
                ),
            });
        }

        if self.endpoint.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "Endpoint cannot be empty".to_string(),
            });
        }

        Ok(())
    }
}

/// Configuration for trust propagation behavior.
///
/// Controls how trust relationships can propagate through the network
/// and what conditions must be met for transitive trust.
#[derive(Debug, Clone)]
pub struct TrustPropagationConfig {
    /// Whether to enable transitive trust calculations.
    ///
    /// When enabled, nodes can inherit trust through mutual connections.
    /// For example, if A trusts B and B trusts C, A might trust C.
    pub enable_transitive_trust: bool,

    /// Maximum number of hops for trust propagation.
    ///
    /// Limits how far trust can propagate through the network.
    /// Higher values allow more flexible trust but may introduce security risks.
    pub max_trust_hops: usize,

    /// Minimum trust level required for propagation.
    ///
    /// Only nodes with at least this trust level can propagate trust to others.
    pub min_propagation_trust: TrustLevel,

    /// How much trust degrades with each hop.
    ///
    /// For example, if a High trust node trusts another node with High trust,
    /// but with degradation enabled, the transitive trust might only be Basic.
    pub trust_degradation_enabled: bool,
}

impl Default for TrustPropagationConfig {
    fn default() -> Self {
        Self {
            enable_transitive_trust: true,
            max_trust_hops: 3,
            min_propagation_trust: TrustLevel::Medium,
            trust_degradation_enabled: true,
        }
    }
}

/// Configuration for the node registry.
///
/// Controls registry behavior including limits, timeouts, and trust policies.
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Maximum number of nodes that can be registered.
    ///
    /// Prevents resource exhaustion from too many node registrations.
    /// Default: 10,000
    pub max_nodes: usize,

    /// How long to keep inactive nodes before removing them.
    ///
    /// Nodes that haven't been seen for this duration are considered
    /// inactive and may be removed from the registry.
    /// Default: 30 days
    pub node_timeout: Duration,

    /// Whether to automatically clean up inactive nodes.
    ///
    /// When enabled, the registry will periodically remove nodes
    /// that haven't been seen for longer than `node_timeout`.
    /// Default: true
    pub auto_cleanup: bool,

    /// Minimum trust level required for node registration.
    ///
    /// New nodes must achieve at least this trust level before
    /// they can be registered in the network.
    /// Default: Unknown
    pub min_registration_trust: TrustLevel,

    /// Configuration for trust propagation.
    pub trust_propagation: TrustPropagationConfig,

    /// Maximum number of trust relationships per node
    pub max_trust_relationships: usize,

    /// Enable detailed audit logging
    pub enable_audit_logging: bool,

    /// Background cleanup interval
    pub cleanup_interval: Duration,

    /// Federation configuration
    pub federation: FederationConfig,

    /// Phonebook service configuration
    pub phonebook: PhonebookConfig,

    /// Peer-to-peer networking configuration
    pub p2p: P2PConfig,

    /// This node's role in the network
    pub node_role: NodeRole,

    /// Enable service discovery
    pub enable_service_discovery: bool,

    /// Service advertisement interval
    pub service_advertisement_interval: Duration,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_nodes: 10000,
            node_timeout: Duration::from_secs(30 * 24 * 60 * 60), // 30 days
            auto_cleanup: true,
            min_registration_trust: TrustLevel::Unknown,
            trust_propagation: TrustPropagationConfig::default(),
            max_trust_relationships: 1000,
            enable_audit_logging: true,
            cleanup_interval: Duration::from_secs(60 * 60), // 1 hour
            federation: FederationConfig::default(),
            phonebook: PhonebookConfig::default(),
            p2p: P2PConfig::default(),
            node_role: NodeRole::SecurityNode,
            enable_service_discovery: true,
            service_advertisement_interval: Duration::from_secs(30),
        }
    }
}

/// Trust store managing relationships between nodes.
///
/// Maintains direct trust relationships and handles trust propagation
/// calculations based on the configured policies.
#[derive(Debug, Clone)]
pub struct TrustStore {
    /// Direct trust relationships between nodes.
    ///
    /// Maps (node1, node2) -> trust_level for explicitly set relationships.
    pub(crate) direct_trust: HashMap<(String, String), TrustLevel>,

    /// Configuration for trust propagation.
    pub(crate) propagation_config: TrustPropagationConfig,
}

impl Default for TrustStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustStore {
    /// Create a new trust store
    pub fn new() -> Self {
        Self {
            direct_trust: HashMap::new(),
            propagation_config: TrustPropagationConfig::default(),
        }
    }

    /// Create a new trust store with custom propagation config
    pub fn with_config(config: TrustPropagationConfig) -> Self {
        Self {
            direct_trust: HashMap::new(),
            propagation_config: config,
        }
    }

    /// Get the number of trust relationships
    pub fn relationship_count(&self) -> usize {
        self.direct_trust.len()
    }

    /// Get all nodes that trust a specific node
    pub fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        self.direct_trust
            .keys()
            .filter(|(_, to_node)| to_node == node_id)
            .map(|(from_node, _)| from_node.clone())
            .collect()
    }

    /// Get all nodes trusted by a specific node
    pub fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
        self.direct_trust
            .keys()
            .filter(|(from_node, _)| from_node == node_id)
            .map(|(_, to_node)| to_node.clone())
            .collect()
    }

    /// Clear all trust relationships
    pub fn clear_all(&mut self) {
        self.direct_trust.clear();
    }
}

/// Node registration information
#[derive(Debug, Clone)]
pub struct NodeRegistration {
    /// Node identifier
    pub node_id: String,
    /// Node information
    pub node_info: NodeInfo,
}

impl NodeRegistration {
    /// Create a new node registration
    pub fn new(node_id: String, node_info: NodeInfo) -> Self {
        Self { node_id, node_info }
    }
}

/// Node entry for internal registry use
#[derive(Debug, Clone)]
pub struct NodeEntry {
    /// Node information from auth module
    pub node_info: crate::auth::types::NodeInfo,
    /// Trust level as f64 for compatibility
    pub trust_level: f64,
    /// Last seen timestamp
    pub last_seen: SystemTime,
    /// Whether node is currently active
    pub is_active: bool,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl NodeEntry {
    /// Create a new node entry
    pub fn new(
        node_info: crate::auth::types::NodeInfo,
        trust_level: f64,
    ) -> Self {
        Self {
            node_info,
            trust_level,
            last_seen: SystemTime::now(),
            is_active: true,
            capabilities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    /// Check if node is active based on timeout
    pub fn is_active_within(&self, timeout: Duration) -> bool {
        match self.last_seen.elapsed() {
            Ok(elapsed) => elapsed < timeout,
            Err(_) => false,
        }
    }
}

/// Bootstrap node configuration
#[derive(Debug, Clone)]
pub struct BootstrapNodeConfig {
    /// Node identifier
    pub node_id: String,
    /// Public key hex string
    pub public_key_hex: String,
    /// Network address
    pub network_address: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl BootstrapNodeConfig {
    /// Create a new bootstrap node config
    pub fn new(node_id: String, public_key_hex: String, network_address: String) -> Self {
        Self {
            node_id,
            public_key_hex,
            network_address,
            metadata: HashMap::new(),
        }
    }

    /// Validate the bootstrap configuration
    pub fn validate(&self) -> BearDogResult<()> {
        if self.node_id.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "Bootstrap node ID cannot be empty".to_string(),
            });
        }

        if self.public_key_hex.is_empty() {
            return Err(BearDogError::InvalidInput {
                message: "Bootstrap public key cannot be empty".to_string(),
            });
        }

        // Validate hex format
        if hex::decode(&self.public_key_hex).is_err() {
            return Err(BearDogError::InvalidInput {
                message: "Bootstrap public key must be valid hex".to_string(),
            });
        }

        // Validate key length (should be 32 bytes = 64 hex chars)
        if self.public_key_hex.len() != 64 {
            return Err(BearDogError::InvalidInput {
                message: format!(
                    "Bootstrap public key must be 64 hex characters, got {}",
                    self.public_key_hex.len()
                ),
            });
        }

        Ok(())
    }

    /// Convert to NodeInfo
    pub fn to_node_info(&self) -> BearDogResult<NodeInfo> {
        self.validate()?;

        let public_key = hex::decode(&self.public_key_hex)
            .map_err(|e| BearDogError::InvalidInput {
                message: format!("Failed to decode public key: {}", e),
            })?;

        Ok(NodeInfo {
            public_key,
            trust_level: TrustLevel::Explicit, // Bootstrap nodes get explicit trust
            capabilities: vec!["bootstrap".to_string(), "relay".to_string()],
            endpoint: self.network_address.clone(),
            last_seen: SystemTime::now(),
            metadata: self.metadata.clone(),
            node_id: self.node_id.clone(),
            display_name: format!("Bootstrap Node: {}", self.node_id),
            node_type: "bootstrap_node".to_string(),
            network_address: self.network_address.clone(),
            registration_timestamp: chrono::Utc::now(),
        })
    }
}

/// Node verification result
#[derive(Debug, Clone)]
pub struct NodeVerificationResult {
    /// Whether verification passed
    pub verified: bool,
    /// Verification message or error
    pub message: String,
    /// Trust level after verification
    pub trust_level: TrustLevel,
    /// Additional verification metadata
    pub metadata: HashMap<String, String>,
}

impl NodeVerificationResult {
    /// Create a successful verification result
    pub fn success(trust_level: TrustLevel, message: String) -> Self {
        Self {
            verified: true,
            message,
            trust_level,
            metadata: HashMap::new(),
        }
    }

    /// Create a failed verification result
    pub fn failure(message: String) -> Self {
        Self {
            verified: false,
            message,
            trust_level: TrustLevel::Unknown,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the result
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Type alias for backward compatibility
pub type InMemoryNodeRegistry = crate::node_registry::BearDogNodeRegistry;

/// Trust relationship between two nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRelationship {
    /// Source node
    pub from_node: String,
    /// Target node
    pub to_node: String,
    /// Trust level
    pub trust_level: TrustLevel,
    /// When the relationship was established
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the relationship was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TrustRelationship {
    /// Create a new trust relationship
    pub fn new(from_node: String, to_node: String, trust_level: TrustLevel) -> Self {
        let now = chrono::Utc::now();
        Self {
            from_node,
            to_node,
            trust_level,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the trust level
    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
        self.updated_at = chrono::Utc::now();
    }
}

/// Registry statistics
#[derive(Debug, Clone, Default)]
pub struct RegistryStatistics {
    /// Total number of registered nodes
    pub total_nodes: usize,
    /// Number of active nodes
    pub active_nodes: usize,
    /// Number of trust relationships
    pub trust_relationships: usize,
    /// Nodes by trust level
    pub nodes_by_trust_level: HashMap<TrustLevel, usize>,
    /// Average node age in seconds
    pub average_node_age_seconds: u64,
    /// Registry uptime in seconds
    pub registry_uptime_seconds: u64,
}

impl RegistryStatistics {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Get total trust relationships
    pub fn total_trust_relationships(&self) -> usize {
        self.trust_relationships
    }

    /// Get percentage of active nodes
    pub fn active_node_percentage(&self) -> f64 {
        if self.total_nodes == 0 {
            0.0
        } else {
            (self.active_nodes as f64 / self.total_nodes as f64) * 100.0
        }
    }
}

/// Node role within a cluster
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeRole {
    /// Standard BearDog security node
    SecurityNode,
    
    /// Phonebook discovery node - helps others find each other
    PhonebookNode,
    
    /// Federation bridge node - connects different BearDog registries
    FederationBridge,
    
    /// Compute provider node
    ComputeProvider,
    
    /// Storage provider node
    StorageProvider,
    
    /// Relay node for network traffic
    RelayNode,
    
    /// Backup/archive node
    BackupNode,
}

/// Federation configuration for connecting multiple BearDog registries
#[derive(Debug, Clone)]
pub struct FederationConfig {
    /// Enable federation with other BearDog instances
    pub enable_federation: bool,
    
    /// Maximum number of federated registries to connect with
    pub max_federated_registries: usize,
    
    /// Interval for federation heartbeat/sync
    pub federation_heartbeat_interval: Duration,
    
    /// Timeout for federation operations
    pub federation_timeout: Duration,
    
    /// Trust level required for federation
    pub min_federation_trust: TrustLevel,
    
    /// Enable automatic federation discovery
    pub enable_auto_federation: bool,
    
    /// Federation discovery tags
    pub federation_tags: Vec<String>,
}

impl Default for FederationConfig {
    fn default() -> Self {
        Self {
            enable_federation: true,
            max_federated_registries: 10,
            federation_heartbeat_interval: Duration::from_secs(30),
            federation_timeout: Duration::from_secs(10),
            min_federation_trust: TrustLevel::Basic,
            enable_auto_federation: true,
            federation_tags: vec![
                "beardog-security".to_string(),
                "distributed-registry".to_string(),
            ],
        }
    }
}

/// Phonebook service configuration
#[derive(Debug, Clone)]
pub struct PhonebookConfig {
    /// Enable phonebook service functionality
    pub enable_phonebook_service: bool,
    
    /// Address to bind phonebook service
    pub bind_address: String,
    
    /// Port for phonebook service
    pub port: u16,
    
    /// Maximum number of nodes to track
    pub max_tracked_nodes: usize,
    
    /// How long to keep node entries
    pub node_entry_ttl: Duration,
    
    /// Enable public discovery (vs private networks only)
    pub enable_public_discovery: bool,
    
    /// Geographic region for phonebook service
    pub region: String,
    
    /// Supported protocols
    pub supported_protocols: Vec<String>,
}

impl Default for PhonebookConfig {
    fn default() -> Self {
        Self {
            enable_phonebook_service: false,
            bind_address: "0.0.0.0".to_string(),
            port: 8844,
            max_tracked_nodes: 10000,
            node_entry_ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
            enable_public_discovery: false,
            region: "global".to_string(),
            supported_protocols: vec![
                "https".to_string(),
                "beardog-secure".to_string(),
                "federation".to_string(),
            ],
        }
    }
}

/// Peer-to-peer network configuration
#[derive(Debug, Clone)]
pub struct P2PConfig {
    /// Enable peer-to-peer networking
    pub enable_p2p: bool,
    
    /// Local node's external address
    pub external_address: String,
    
    /// P2P listening port
    pub p2p_port: u16,
    
    /// Maximum number of simultaneous peer connections
    pub max_peer_connections: usize,
    
    /// Connection timeout for peer connections
    pub connection_timeout: Duration,
    
    /// Enable NAT traversal
    pub enable_nat_traversal: bool,
    
    /// STUN servers for NAT traversal
    pub stun_servers: Vec<String>,
    
    /// Enable DHT for distributed discovery
    pub enable_dht: bool,
    
    /// DHT bootstrap nodes
    pub dht_bootstrap_nodes: Vec<String>,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            enable_p2p: true,
            external_address: "auto-detect".to_string(),
            p2p_port: 8845,
            max_peer_connections: 50,
            connection_timeout: Duration::from_secs(15),
            enable_nat_traversal: true,
            stun_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            enable_dht: true,
            dht_bootstrap_nodes: Vec::new(),
        }
    }
}

/// Distributed registry information
#[derive(Debug, Clone)]
pub struct DistributedRegistryInfo {
    /// Registry unique identifier
    pub registry_id: String,
    
    /// Registry operator information
    pub operator: String,
    
    /// Registry public key for verification
    pub registry_public_key: Vec<u8>,
    
    /// Registry endpoints
    pub endpoints: Vec<String>,
    
    /// Registry capabilities
    pub capabilities: Vec<String>,
    
    /// Registry geographic region
    pub region: String,
    
    /// Registry version
    pub version: String,
    
    /// Federation status
    pub federation_status: FederationStatus,
    
    /// Last seen timestamp
    pub last_seen: SystemTime,
    
    /// Trust level for this registry
    pub trust_level: TrustLevel,
}

/// Federation status for a registry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FederationStatus {
    /// Not federated
    NotFederated,
    
    /// Attempting to federate
    Connecting,
    
    /// Successfully federated
    Federated,
    
    /// Federation failed
    Failed,
    
    /// Temporarily disconnected
    Disconnected,
}

/// Service advertisement for discovery
#[derive(Debug, Clone)]
pub struct ServiceAdvertisement {
    /// Service unique identifier
    pub service_id: String,
    
    /// Service name
    pub service_name: String,
    
    /// Service type
    pub service_type: String,
    
    /// Service version
    pub version: String,
    
    /// Service endpoints
    pub endpoints: Vec<String>,
    
    /// Service capabilities
    pub capabilities: Vec<String>,
    
    /// Service metadata
    pub metadata: HashMap<String, String>,
    
    /// Geographic region
    pub region: String,
    
    /// Advertisement TTL
    pub ttl: Duration,
    
    /// Service health status
    pub health_status: ServiceHealthStatus,
}

/// Service health status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceHealthStatus {
    /// Service is healthy
    Healthy,
    
    /// Service is degraded
    Degraded,
    
    /// Service is unavailable
    Unavailable,
    
    /// Service health is unknown
    Unknown,
} 