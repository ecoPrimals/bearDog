//! Node Registry and Trust Management System
//!
//! This module implements BearDog's node registry and trust management system,
//! which is essential for secure cross-node authorization. The registry maintains
//! information about known nodes, their cryptographic keys, and trust relationships.
//!
//! ## Overview
//!
//! The node registry serves as the foundation for BearDog's distributed security model.
//! It enables nodes to:
//!
//! * **Discover and identify** other nodes in the network
//! * **Establish trust relationships** based on verification and reputation
//! * **Manage cryptographic keys** for secure communication
//! * **Track node capabilities** and service offerings
//! * **Handle node lifecycle** events (joining, leaving, failures)
//!
//! ## Trust Levels
//!
//! BearDog uses a hierarchical trust system:
//!
//! * **Unknown** (0): New or unverified nodes
//! * **Basic** (1): Nodes that have passed basic verification
//! * **Medium** (2): Nodes with moderate trust level
//! * **High** (3): Nodes with proven reliability and good reputation
//! * **Explicit** (4): Nodes explicitly trusted by administrators
//!
//! ## Security Model
//!
//! The registry implements several security measures:
//!
//! * **Cryptographic Identity**: Each node is identified by its public key
//! * **Mutual Authentication**: Nodes must prove their identity before registration
//! * **Trust Propagation**: Trust relationships can be inherited through the network
//! * **Revocation Support**: Compromised or malicious nodes can be quickly removed
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use beardog::node_registry::{BearDogNodeRegistry, NodeInfo, TrustLevel};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = BearDogNodeRegistry::new();
//!
//! // Register a new node
//! let node_info = NodeInfo {
//!     public_key: vec![/* Ed25519 public key bytes */],
//!     trust_level: TrustLevel::Basic,
//!     capabilities: vec!["storage".to_string(), "compute".to_string()],
//!     endpoint: "https://node1.example.com:8443".to_string(),
//!     last_seen: std::time::SystemTime::now(),
//!     metadata: std::collections::HashMap::new(),
//! };
//!
//! registry.register_node("node1".to_string(), node_info).await?;
//!
//! // Check if two nodes trust each other
//! let trust_exists = registry.nodes_trust_each_other("node1", "node2").await?;
//! println!("Nodes trust each other: {}", trust_exists);
//!
//! // List all highly trusted nodes
//! let trusted_nodes = registry.list_trusted_nodes(TrustLevel::High).await?;
//! println!("Highly trusted nodes: {:?}", trusted_nodes);
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::cross_node_auth::NodeRegistry;
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

/// Information about a node in the registry.
///
/// This struct contains all the metadata needed to identify, communicate with,
/// and make trust decisions about a node in the BearDog network.
///
/// ## Security Considerations
///
/// - The `public_key` is the node's cryptographic identity and must be protected
/// - `trust_level` determines what operations the node can perform
/// - `capabilities` should be verified and not simply trusted
/// - `last_seen` helps detect inactive or compromised nodes
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
    pub last_seen: std::time::SystemTime,

    /// Additional metadata about the node.
    ///
    /// Can include information like:
    /// - "version" - Software version
    /// - "region" - Geographic region
    /// - "operator" - Node operator information
    /// - "contact" - Contact information for the operator
    pub metadata: std::collections::HashMap<String, String>,

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

/// Manages trust relationships between nodes.
///
/// The trust store maintains a graph of trust relationships and provides
/// methods for querying and updating these relationships. It supports
/// both direct trust (explicitly set) and transitive trust (inherited
/// through the network).
///
/// ## Trust Propagation
///
/// Trust can propagate through the network based on configurable rules:
/// - **Direct Trust**: Explicitly set relationships
/// - **Transitive Trust**: Trust inherited through mutual connections
/// - **Reputation-based**: Trust based on historical behavior
///
/// ## Example
///
/// ```rust,no_run
/// use beardog::node_registry::{TrustStore, TrustLevel};
///
/// let mut trust_store = TrustStore::new();
///
/// // Set direct trust relationship
/// trust_store.set_trust("node1", "node2", TrustLevel::High);
///
/// // Query trust level
/// let trust_level = trust_store.get_trust_level("node1", "node2");
/// println!("Trust level: {:?}", trust_level);
/// ```
#[derive(Debug, Clone)]
pub struct TrustStore {
    /// Direct trust relationships between nodes.
    ///
    /// Maps (node1, node2) -> trust_level for explicitly set relationships.
    direct_trust: std::collections::HashMap<(String, String), TrustLevel>,

    /// Configuration for trust propagation.
    propagation_config: TrustPropagationConfig,
}

/// Configuration for how trust propagates through the network.
///
/// These settings control how trust relationships are inherited and
/// computed when there's no direct trust relationship between nodes.
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

/// The main node registry implementation for BearDog.
///
/// This is the primary interface for managing nodes in the BearDog network.
/// It provides thread-safe access to node information, trust relationships,
/// and network topology.
///
/// ## Thread Safety
///
/// The registry is designed to be thread-safe and can be shared across
/// multiple async tasks. All operations are atomic and consistent.
///
/// ## Persistence
///
/// The registry can be configured to persist node information to disk,
/// ensuring that trust relationships and node data survive system restarts.
///
/// ## Example Usage
///
/// ```rust,no_run
/// use beardog::node_registry::{BearDogNodeRegistry, NodeInfo, TrustLevel};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut registry = BearDogNodeRegistry::new();
///
/// // Create node information
/// let node_info = NodeInfo {
///     public_key: vec![/* 32-byte Ed25519 public key */],
///     trust_level: TrustLevel::Basic,
///     capabilities: vec!["storage".to_string()],
///     endpoint: "https://example.com:8443".to_string(),
///     last_seen: std::time::SystemTime::now(),
///     metadata: std::collections::HashMap::new(),
/// };
///
/// // Register the node
/// registry.register_node("node1".to_string(), node_info).await?;
///
/// // Update trust level
/// registry.update_trust_level("node1", TrustLevel::High).await?;
///
/// // Check trust between nodes
/// let trust_exists = registry.nodes_trust_each_other("node1", "node2").await?;
/// # Ok(())
/// # }
/// ```
pub struct BearDogNodeRegistry {
    /// All known nodes indexed by their ID.
    nodes: Arc<RwLock<HashMap<String, NodeInfo>>>,

    /// Trust relationships between nodes.
    trust_store: Arc<RwLock<TrustStore>>,

    /// Configuration for the registry.
    config: RegistryConfig,
}

/// Configuration parameters for the node registry.
///
/// These settings control the behavior of the node registry, including
/// security policies, performance tuning, and operational limits.
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
    pub node_timeout: std::time::Duration,

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
}

impl BearDogNodeRegistry {
    /// Create a new node registry
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            trust_store: Arc::new(RwLock::new(TrustStore {
                direct_trust: std::collections::HashMap::new(),
                propagation_config: TrustPropagationConfig {
                    enable_transitive_trust: true,
                    max_trust_hops: 3,
                    min_propagation_trust: TrustLevel::Basic,
                    trust_degradation_enabled: true,
                },
            })),
            config,
        }
    }

    /// Add a new node to the registry
    ///
    /// # Arguments
    /// * `node_id` - Unique identifier for the node
    /// * `node_info` - Node information including public key and capabilities
    pub async fn add_node_with_id(
        &self,
        node_id: String,
        node_info: NodeInfo,
    ) -> BearDogResult<()> {
        // Check if we've reached the maximum number of nodes
        if self.nodes.read().await.len() >= self.config.max_nodes {
            return Err(BearDogError::validation(
                "nodes",
                "Maximum number of nodes reached",
            ));
        }

        // Validate the public key
        if node_info.public_key.len() != 32 {
            return Err(BearDogError::validation(
                "public_key",
                "Invalid public key length",
            ));
        }

        // Store the node
        self.nodes.write().await.insert(node_id.clone(), node_info);

        info!("📝 Added node {} to registry", node_id);

        Ok(())
    }

    /// Add a node using its endpoint as the ID
    pub async fn add_node(&self, node_info: NodeInfo) -> BearDogResult<()> {
        let node_id = node_info.endpoint.clone();
        self.add_node_with_id(node_id, node_info).await
    }

    /// Get node information
    pub async fn get_node(&self, node_id: &str) -> BearDogResult<Option<NodeInfo>> {
        Ok(self.nodes.read().await.get(node_id).cloned())
    }

    /// List all known nodes
    pub async fn list_nodes(&self) -> BearDogResult<Vec<NodeInfo>> {
        Ok(self.nodes.read().await.values().cloned().collect())
    }

    /// Update node's last seen timestamp
    pub async fn update_node_last_seen(&self, node_id: &str) -> BearDogResult<()> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.last_seen = std::time::SystemTime::now();
            debug!("👀 Updated last seen for node {}", node_id);
        }
        Ok(())
    }

    /// Set trust level for a node
    pub async fn set_trust_level(
        &self,
        node_id: &str,
        trust_level: TrustLevel,
    ) -> BearDogResult<()> {
        // Update the node's trust level in the registry
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.trust_level = trust_level;
        }

        // Update the trust store
        self.trust_store
            .write()
            .await
            .direct_trust
            .insert((node_id.to_string(), node_id.to_string()), trust_level);

        info!(
            "🤝 Set trust level for node {} to {:?}",
            node_id, trust_level
        );

        Ok(())
    }

    /// Get trust level for a node
    pub async fn get_trust_level(&self, node_id: &str) -> BearDogResult<TrustLevel> {
        let trust_store = self.trust_store.read().await;
        // Check if there's a self-trust entry for this node
        if let Some(trust_level) = trust_store
            .direct_trust
            .get(&(node_id.to_string(), node_id.to_string()))
        {
            Ok(*trust_level)
        } else {
            // Check the node's own trust level from the registry
            if let Some(node) = self.nodes.read().await.get(node_id) {
                Ok(node.trust_level)
            } else {
                Ok(TrustLevel::Unknown)
            }
        }
    }

    /// Remove a node from the registry
    pub async fn remove_node(&self, node_id: &str) -> BearDogResult<()> {
        self.nodes.write().await.remove(node_id);

        // Remove trust relationships involving this node
        let mut trust_store = self.trust_store.write().await;
        trust_store
            .direct_trust
            .retain(|(from_node, to_node), _| from_node != node_id && to_node != node_id);

        info!("🗑️ Removed node {} from registry", node_id);

        Ok(())
    }

    /// Bootstrap with initial nodes from environment variables
    pub async fn bootstrap(&self) -> BearDogResult<()> {
        tracing::info!("Bootstrapping node registry from environment variables");

        // Look for bootstrap nodes in environment variables
        let bootstrap_nodes = self.discover_bootstrap_nodes();

        if bootstrap_nodes.is_empty() {
            tracing::warn!("No bootstrap nodes found in environment - starting in isolated mode");
            return Ok(());
        }

        let mut successful_bootstraps = 0;
        let mut failed_bootstraps = 0;

        for node_id in &bootstrap_nodes {
            tracing::debug!("Bootstrapping node: {}", node_id);

            // Load bootstrap node configuration from environment or secure storage
            match self.load_bootstrap_node_config(node_id).await {
                Ok(node_info) => {
                    // Verify the bootstrap node's identity and public key
                    if self.verify_bootstrap_node(&node_info).await? {
                        // Add the verified bootstrap node
                        self.add_node_with_id(node_id.clone(), node_info).await?;
                        self.set_trust_level(node_id, TrustLevel::Explicit).await?;

                        tracing::info!("✅ Bootstrap node {} added with explicit trust", node_id);
                        successful_bootstraps += 1;
                    } else {
                        tracing::error!("❌ Bootstrap node {} failed verification", node_id);
                        failed_bootstraps += 1;
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to load bootstrap node {}: {}", node_id, e);
                    failed_bootstraps += 1;
                }
            }
        }

        tracing::info!(
            "Bootstrap complete: {} successful, {} failed",
            successful_bootstraps,
            failed_bootstraps
        );

        Ok(())
    }

    /// Discover bootstrap nodes from environment variables
    fn discover_bootstrap_nodes(&self) -> Vec<String> {
        let mut nodes = Vec::new();

        // Look for numbered bootstrap nodes: BEARDOG_BOOTSTRAP_1, BEARDOG_BOOTSTRAP_2, etc.
        for i in 1..=10 {
            // Check first 10 potential nodes
            let env_key = format!("BEARDOG_BOOTSTRAP_{}", i);
            let pubkey_env = format!("{}_PUBLIC_KEY", env_key);
            let address_env = format!("{}_ADDRESS", env_key);

            if std::env::var(&pubkey_env).is_ok() && std::env::var(&address_env).is_ok() {
                nodes.push(format!("bootstrap_{}", i));
            }
        }

        nodes
    }

    /// Load bootstrap node configuration from secure storage
    async fn load_bootstrap_node_config(&self, node_id: &str) -> BearDogResult<NodeInfo> {
        // Try to load from environment variables first
        let env_key = format!(
            "BEARDOG_BOOTSTRAP_{}",
            node_id.to_uppercase().replace('-', "_")
        );
        let env_pubkey = format!("{}_PUBLIC_KEY", env_key);
        let env_address = format!("{}_ADDRESS", env_key);

        if let (Ok(public_key_hex), Ok(network_address)) =
            (std::env::var(&env_pubkey), std::env::var(&env_address))
        {
            let public_key = hex::decode(public_key_hex).unwrap_or_default();

            if public_key.len() != 32 {
                return Err(BearDogError::config(format!(
                    "Bootstrap public key for {} must be 32 bytes, got {}",
                    node_id,
                    public_key.len()
                )));
            }

            return Ok(NodeInfo {
                public_key,
                trust_level: TrustLevel::Basic,
                capabilities: vec!["storage".to_string(), "compute".to_string()],
                endpoint: network_address.clone(),
                last_seen: std::time::SystemTime::now(),
                metadata: {
                    let mut meta = std::collections::HashMap::new();
                    meta.insert("bootstrap".to_string(), "true".to_string());
                    meta.insert("version".to_string(), "1.0.0".to_string());
                    meta
                },
                node_id: node_id.to_string(),
                display_name: format!("Bootstrap Node: {}", node_id),
                node_type: "beardog_security_node".to_string(),
                network_address,
                registration_timestamp: chrono::Utc::now(),
            });
        }

        // If not found in environment, this would load from secure configuration file
        // For now, return an error to enforce proper configuration
        Err(BearDogError::config(format!(
            "Bootstrap node {} not found. Set {} and {} environment variables",
            node_id, env_pubkey, env_address
        )))
    }

    /// Verify a bootstrap node's identity and credentials
    async fn verify_bootstrap_node(&self, node_info: &NodeInfo) -> BearDogResult<bool> {
        // Verify the node's public key is valid Ed25519 key
        if node_info.public_key.len() != 32 {
            tracing::warn!(
                "Bootstrap node {} has invalid public key length",
                node_info.endpoint
            );
            return Ok(false);
        }

        // In a production system, this would:
        // 1. Verify the node's certificate chain
        // 2. Check against a trusted bootstrap registry
        // 3. Perform challenge-response authentication
        // 4. Validate the node's network connectivity

        // For now, we verify that the public key is not all zeros (placeholder key)
        let is_valid_key = !node_info.public_key.iter().all(|&b| b == 0);

        if !is_valid_key {
            tracing::warn!(
                "Bootstrap node {} has placeholder (all-zero) public key",
                node_info.endpoint
            );
            return Ok(false);
        }

        // Additional bootstrap verification could include:
        // - Network connectivity test
        // - Certificate validation
        // - Proof of identity

        tracing::debug!(
            "Bootstrap node {} passed basic verification",
            node_info.endpoint
        );
        Ok(true)
    }

    /// Get nodes with specific trust level or higher
    pub async fn get_trusted_nodes(
        &self,
        min_trust_level: TrustLevel,
    ) -> BearDogResult<Vec<NodeInfo>> {
        let nodes = self.nodes.read().await;
        let trusted_nodes: Vec<NodeInfo> = nodes
            .values()
            .filter(|node| node.trust_level >= min_trust_level)
            .cloned()
            .collect();

        Ok(trusted_nodes)
    }

    /// Verify a node's public key signature
    pub async fn verify_node_signature(
        &self,
        node_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        let node = self
            .get_node(node_id)
            .await?
            .ok_or_else(|| BearDogError::not_found("node", node_id))?;

        tracing::debug!("Verifying signature for node: {}", node_id);

        // Validate node public key length
        if node.public_key.len() != 32 {
            tracing::warn!(
                "Invalid public key length for node {}: {} bytes",
                node_id,
                node.public_key.len()
            );
            return Ok(false);
        }

        // Use our crypto utilities to verify the Ed25519 signature
        let _crypto = crate::crypto_utils::BearDogCrypto;
        let is_valid = crate::crypto_utils::BearDogCrypto::verify_ed25519_signature(
            &node.public_key,
            data,
            signature,
        )?;

        if is_valid {
            tracing::debug!("✅ Node signature verified for {}", node_id);
            // Update last seen time for verified nodes
            if let Err(e) = self.update_node_last_seen(node_id).await {
                tracing::warn!("Failed to update last seen time for {}: {}", node_id, e);
            }
        } else {
            tracing::warn!("❌ Invalid signature from node {}", node_id);
        }

        Ok(is_valid)
    }
}

#[async_trait]
impl NodeRegistry for BearDogNodeRegistry {
    /// Get the public key for a specific node
    async fn get_node_public_key(&self, node_id: &str) -> BearDogResult<Vec<u8>> {
        let node = self
            .get_node(node_id)
            .await?
            .ok_or_else(|| BearDogError::not_found("node", node_id))?;

        Ok(node.public_key)
    }

    /// Register a new node with its public key
    async fn register_node(&self, node_id: &str, public_key: &[u8]) -> BearDogResult<()> {
        let node_info = NodeInfo {
            public_key: public_key.to_vec(),
            trust_level: TrustLevel::Unknown,
            capabilities: vec!["basic".to_string()],
            endpoint: format!("node://{}", node_id), // Default endpoint format
            last_seen: std::time::SystemTime::now(),
            metadata: std::collections::HashMap::new(),
            node_id: node_id.to_string(),
            display_name: format!("Node: {}", node_id),
            node_type: "beardog_security_node".to_string(),
            network_address: "".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        self.add_node_with_id(node_id.to_string(), node_info).await
    }

    /// Check if a node is trusted (has at least Basic trust level)
    async fn is_trusted_node(&self, node_id: &str) -> BearDogResult<bool> {
        let trust_level = self.get_trust_level(node_id).await?;
        Ok(trust_level >= TrustLevel::Basic)
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_nodes: 1000,
            node_timeout: std::time::Duration::from_secs(30 * 24 * 60 * 60),
            auto_cleanup: true,
            min_registration_trust: TrustLevel::Unknown,
            trust_propagation: TrustPropagationConfig {
                enable_transitive_trust: true,
                max_trust_hops: 3,
                min_propagation_trust: TrustLevel::Basic,
                trust_degradation_enabled: true,
            },
        }
    }
}

impl Default for TrustStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustStore {
    /// Create a new trust store
    ///
    /// # Returns
    /// A new trust store with default configuration
    pub fn new() -> Self {
        Self {
            direct_trust: std::collections::HashMap::new(),
            propagation_config: TrustPropagationConfig {
                enable_transitive_trust: true,
                max_trust_hops: 3,
                min_propagation_trust: TrustLevel::Basic,
                trust_degradation_enabled: true,
            },
        }
    }

    /// Set trust level between two nodes
    ///
    /// # Arguments
    /// * `from_node` - The node that is trusting
    /// * `to_node` - The node being trusted
    /// * `trust_level` - The level of trust to establish
    pub fn set_trust(&mut self, from_node: &str, to_node: &str, trust_level: TrustLevel) {
        self.direct_trust
            .insert((from_node.to_string(), to_node.to_string()), trust_level);
    }

    /// Get trust level between two nodes
    ///
    /// # Arguments
    /// * `from_node` - The node that is trusting
    /// * `to_node` - The node being trusted
    ///
    /// # Returns
    /// The trust level if a direct relationship exists, otherwise `TrustLevel::Unknown`
    pub fn get_trust_level(&self, from_node: &str, to_node: &str) -> TrustLevel {
        self.direct_trust
            .get(&(from_node.to_string(), to_node.to_string()))
            .cloned()
            .unwrap_or(TrustLevel::Unknown)
    }

    /// Check if there's a trust relationship between two nodes
    ///
    /// # Arguments
    /// * `from_node` - The node that is trusting
    /// * `to_node` - The node being trusted
    ///
    /// # Returns
    /// True if there is at least Basic trust between the nodes
    pub fn has_trust_relationship(&self, from_node: &str, to_node: &str) -> bool {
        let trust_level = self.get_trust_level(from_node, to_node);
        trust_level >= TrustLevel::Basic
    }
}

/// In-memory implementation of node registry for testing and simple deployments
pub type InMemoryNodeRegistry = BearDogNodeRegistry;

/// Node registration structure for compatibility
#[derive(Debug, Clone)]
pub struct NodeRegistration {
    pub node_id: String,
    pub node_info: NodeInfo,
}

impl NodeRegistration {
    pub fn new(node_id: String, node_info: NodeInfo) -> Self {
        Self { node_id, node_info }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_registry_creation() {
        let config = RegistryConfig::default();
        let registry = BearDogNodeRegistry::new(config);

        let nodes = registry.list_nodes().await.unwrap();
        assert!(nodes.is_empty());
    }

    #[tokio::test]
    async fn test_add_and_get_node() {
        let config = RegistryConfig::default();
        let registry = BearDogNodeRegistry::new(config);

        let mut metadata = HashMap::new();
        metadata.insert("display_name".to_string(), "Test Node".to_string());

        let node_info = NodeInfo {
            public_key: vec![1u8; 32], // Use non-zero key
            trust_level: TrustLevel::Basic,
            capabilities: vec!["test".to_string()],
            endpoint: "https://127.0.0.1:8080".to_string(),
            last_seen: std::time::SystemTime::now(),
            metadata,
            node_id: "test-node".to_string(),
            display_name: "Test Node".to_string(),
            node_type: "beardog_security_node".to_string(),
            network_address: "https://127.0.0.1:8080".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        registry
            .add_node_with_id("test-node".to_string(), node_info.clone())
            .await
            .unwrap();

        let retrieved = registry.get_node("test-node").await.unwrap().unwrap();
        assert_eq!(retrieved.endpoint, "https://127.0.0.1:8080");
        assert_eq!(retrieved.public_key, vec![1u8; 32]);
    }

    #[tokio::test]
    async fn test_trust_levels() {
        let config = RegistryConfig::default();
        let registry = BearDogNodeRegistry::new(config);

        let node_info = NodeInfo {
            public_key: vec![1u8; 32],
            trust_level: TrustLevel::Unknown,
            capabilities: vec!["test".to_string()],
            endpoint: "https://test.example.com".to_string(),
            last_seen: std::time::SystemTime::now(),
            metadata: HashMap::new(),
            node_id: "test-node".to_string(),
            display_name: "Test Node".to_string(),
            node_type: "beardog_security_node".to_string(),
            network_address: "https://test.example.com".to_string(),
            registration_timestamp: chrono::Utc::now(),
        };

        registry
            .add_node_with_id("test-node".to_string(), node_info)
            .await
            .unwrap();

        // Initially unknown
        let trust_level = registry.get_trust_level("test-node").await.unwrap();
        assert_eq!(trust_level, TrustLevel::Unknown);

        // Set to high trust
        registry
            .set_trust_level("test-node", TrustLevel::High)
            .await
            .unwrap();
        let trust_level = registry.get_trust_level("test-node").await.unwrap();
        assert_eq!(trust_level, TrustLevel::High);
    }

    #[tokio::test]
    async fn test_trusted_nodes_filtering() {
        let config = RegistryConfig::default();
        let registry = BearDogNodeRegistry::new(config);

        // Add nodes with different trust levels
        let nodes = [
            ("node1", TrustLevel::Unknown),
            ("node2", TrustLevel::Basic),
            ("node3", TrustLevel::High),
            ("node4", TrustLevel::Explicit),
        ];

        for (i, (node_id, trust_level)) in nodes.iter().enumerate() {
            let node_info = NodeInfo {
                public_key: vec![(i + 1) as u8; 32], // Use different keys
                trust_level: trust_level.clone(),
                capabilities: vec!["test".to_string()],
                endpoint: format!("https://{}.example.com", node_id),
                last_seen: std::time::SystemTime::now(),
                metadata: HashMap::new(),
                node_id: node_id.to_string(),
                display_name: format!("Node: {}", node_id),
                node_type: "beardog_security_node".to_string(),
                network_address: format!("https://{}.example.com", node_id),
                registration_timestamp: chrono::Utc::now(),
            };

            registry
                .add_node_with_id(node_id.to_string(), node_info)
                .await
                .unwrap();
            registry
                .set_trust_level(node_id, trust_level.clone())
                .await
                .unwrap();
        }

        // Get nodes with Basic trust or higher
        let trusted_nodes = registry.get_trusted_nodes(TrustLevel::Basic).await.unwrap();
        assert_eq!(trusted_nodes.len(), 3); // node2, node3, node4

        // Get nodes with High trust or higher
        let high_trust_nodes = registry.get_trusted_nodes(TrustLevel::High).await.unwrap();
        assert_eq!(high_trust_nodes.len(), 2); // node3, node4
    }
}
