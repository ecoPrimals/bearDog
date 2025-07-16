//! Core Node Registry Implementation
//!
//! This module contains the main BearDogNodeRegistry implementation with
//! support for decentralized architecture, federation, and phonebook services.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::bootstrap::BootstrapManager;
use super::federation::{FederationManager, NodeSearchCriteria};
use super::phonebook::PhonebookService;
use super::trust::TrustManager;
use super::types::{
    NodeInfo, TrustLevel, RegistryConfig, RegistryStatistics, ServiceHealthStatus,
    ServiceAdvertisement,
};
use crate::auth::NodeRegistry;
use crate::{BearDogError, BearDogResult};

/// The main node registry implementation for BearDog with decentralized features.
///
/// This registry supports:
/// - Local node management and trust relationships
/// - Federation with other BearDog registries
/// - Phonebook services for discovery
/// - Peer-to-peer networking
/// - Bootstrap node management
pub struct BearDogNodeRegistry {
    /// All known nodes indexed by their ID
    nodes: Arc<RwLock<HashMap<String, NodeInfo>>>,

    /// Trust management
    trust_manager: Arc<TrustManager>,

    /// Federation manager (optional)
    federation_manager: Option<Arc<FederationManager>>,

    /// Phonebook service (optional)
    phonebook_service: Option<Arc<PhonebookService>>,

    /// Bootstrap manager
    bootstrap_manager: Arc<BootstrapManager>,

    /// Configuration for the registry
    config: RegistryConfig,

    /// Registry statistics
    statistics: Arc<RwLock<RegistryStatistics>>,

    /// Local registry information
    local_registry_info: Arc<RwLock<LocalRegistryInfo>>,
}

/// Local registry information
#[derive(Debug, Clone)]
pub struct LocalRegistryInfo {
    /// Registry unique identifier
    pub registry_id: String,

    /// Registry operator information
    pub operator: String,

    /// Registry public key
    pub public_key: Vec<u8>,

    /// Registry private key (for signing)
    pub private_key: Vec<u8>,

    /// Registry endpoints
    pub endpoints: Vec<String>,

    /// Registry capabilities
    pub capabilities: Vec<String>,

    /// Registry region
    pub region: String,

    /// Registry version
    pub version: String,

    /// When registry was started
    pub started_at: SystemTime,
}

impl BearDogNodeRegistry {
    /// Create a new node registry with full decentralized capabilities
    pub async fn new(config: RegistryConfig) -> BearDogResult<Self> {
        info!("🚀 Initializing BearDog Node Registry with decentralized architecture");

        // Generate or load registry identity
        let local_info = Self::initialize_local_registry_info(&config).await?;

        // Create trust manager
        let trust_manager = Arc::new(TrustManager::new(config.trust_propagation.clone()));

        // Create bootstrap manager
        let bootstrap_manager = Arc::new(BootstrapManager::new(config.clone()).await?);

        // Create federation manager if enabled
        let federation_manager = if config.federation.enabled {
            let manager = FederationManager::new(
                config.federation.clone(),
                local_info.registry_id.clone(),
                local_info.public_key.clone(),
            )
            .await?;
            Some(Arc::new(manager))
        } else {
            None
        };

        // Create phonebook service if enabled
        let phonebook_service = if config.phonebook.enabled {
            let service = PhonebookService::new(config.phonebook.clone()).await?;
            Some(Arc::new(service))
        } else {
            None
        };

        // Initialize statistics
        let statistics = Arc::new(RwLock::new(RegistryStatistics::new()));

        let registry = Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            trust_manager,
            federation_manager,
            phonebook_service,
            bootstrap_manager,
            config,
            statistics,
            local_registry_info: Arc::new(RwLock::new(local_info)),
        };

        // Start background tasks
        registry.start_background_tasks().await?;

        info!("✅ BearDog Node Registry initialized successfully");
        Ok(registry)
    }

    /// Create a new registry with default configuration
    pub async fn new_default() -> BearDogResult<Self> {
        Self::new(RegistryConfig::default()).await
    }

    /// Add a new node to the registry
    pub async fn add_node_with_id(
        &self,
        node_id: String,
        node_info: NodeInfo,
    ) -> BearDogResult<()> {
        info!("📝 Adding node '{}' to registry", node_id);

        // Check limits
        {
            let nodes = self.nodes.read().await;
            if nodes.len() >= self.config.max_nodes {
                return Err(BearDogError::validation(
                    "nodes",
                    "Maximum number of nodes reached",
                ));
            }
        }

        // Validate node info
        node_info.validate()?;

        // Check trust level requirements
        if node_info.trust_level < self.config.min_registration_trust {
            return Err(BearDogError::validation(
                "trust_level",
                "Node trust level too low for registration",
            ));
        }

        // Add to local registry
        {
            let mut nodes = self.nodes.write().await;
            nodes.insert(node_id.clone(), node_info.clone());
        }

        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.total_nodes += 1;
            *stats
                .nodes_by_trust_level
                .entry(node_info.trust_level)
                .or_insert(0) += 1;
        }

        // Register with phonebook service if available
        if let Some(ref phonebook) = self.phonebook_service {
            if let Err(e) = phonebook
                .register_node(node_info.clone(), "local".to_string())
                .await
            {
                warn!("Failed to register node with phonebook: {}", e);
            }
        }

        // Advertise through federation if enabled
        if let Some(ref federation) = self.federation_manager {
            let advertisement = ServiceAdvertisement {
                service_id: node_id.clone(),
                service_name: node_info.name.clone(),
                service_type: node_info.node_type.clone(),
                version: "1.0.0".to_string(),
                endpoints: node_info.endpoints.clone(),
                capabilities: node_info.capabilities.clone(),
                metadata: node_info.metadata.clone(),
                region: self.local_registry_info.read().await.region.clone(),
                ttl: Duration::from_secs(24 * 60 * 60), // 24 hours
                health_status: ServiceHealthStatus::Healthy,
            };

            if let Err(e) = federation.advertise_service(advertisement).await {
                warn!("Failed to advertise service through federation: {}", e);
            }
        }

        info!("✅ Node '{}' added to registry successfully", node_id);
        Ok(())
    }

    /// Add a node using its endpoint as the ID
    pub async fn add_node(&self, node_info: NodeInfo) -> BearDogResult<()> {
        let node_id = node_info.id.clone();
        self.add_node_with_id(node_id, node_info).await
    }

    /// Get node information
    pub async fn get_node(&self, node_id: &str) -> BearDogResult<Option<NodeInfo>> {
        // Try local registry first
        {
            let nodes = self.nodes.read().await;
            if let Some(node) = nodes.get(node_id) {
                return Ok(Some(node.clone()));
            }
        }

        // Try federation if enabled
        if let Some(ref federation) = self.federation_manager {
            let criteria = NodeSearchCriteria {
                node_type: None,
                required_capabilities: Vec::new(),
                min_trust_level: TrustLevel::Unknown,
                region: None,
                max_results: 1,
            };

            if let Ok(federated_nodes) = federation.find_federated_nodes(&criteria).await {
                for federated_node in federated_nodes {
                    if federated_node.node_info.id == node_id {
                        return Ok(Some(federated_node.node_info));
                    }
                }
            }
        }

        Ok(None)
    }

    /// List all known nodes (local + federated)
    pub async fn list_nodes(&self) -> BearDogResult<Vec<NodeInfo>> {
        let mut all_nodes = Vec::new();

        // Add local nodes
        {
            let nodes = self.nodes.read().await;
            all_nodes.extend(nodes.values().cloned());
        }

        // Add federated nodes if available
        if let Some(ref federation) = self.federation_manager {
            let criteria = NodeSearchCriteria::default();
            if let Ok(federated_nodes) = federation.find_federated_nodes(&criteria).await {
                for federated_node in federated_nodes {
                    all_nodes.push(federated_node.node_info);
                }
            }
        }

        Ok(all_nodes)
    }

    /// Update node's last seen timestamp
    pub async fn update_node_last_seen(&self, node_id: &str) -> BearDogResult<()> {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.last_seen = SystemTime::now();
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
        // Update local node
        {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(node_id) {
                let old_trust = node.trust_level;
                node.trust_level = trust_level;

                // Update statistics
                {
                    let mut stats = self.statistics.write().await;
                    *stats.nodes_by_trust_level.entry(old_trust).or_insert(0) = stats
                        .nodes_by_trust_level
                        .get(&old_trust)
                        .unwrap_or(&0)
                        .saturating_sub(1);
                    *stats.nodes_by_trust_level.entry(trust_level).or_insert(0) += 1;
                }
            }
        }

        // Update trust manager
        self.trust_manager
            .set_trust_level(node_id, trust_level)
            .await?;

        info!(
            "🤝 Set trust level for node {} to {:?}",
            node_id, trust_level
        );
        Ok(())
    }

    /// Get trust level for a node
    pub async fn get_trust_level(&self, node_id: &str) -> BearDogResult<TrustLevel> {
        // Try local registry first
        {
            let nodes = self.nodes.read().await;
            if let Some(node) = nodes.get(node_id) {
                return Ok(node.trust_level);
            }
        }

        // Try trust manager
        self.trust_manager.get_trust_level(node_id).await
    }

    /// Remove a node from the registry
    pub async fn remove_node(&self, node_id: &str) -> BearDogResult<()> {
        info!("🗑️ Removing node '{}' from registry", node_id);

        let removed_node = {
            let mut nodes = self.nodes.write().await;
            nodes.remove(node_id)
        };

        if let Some(node) = removed_node {
            // Update statistics
            {
                let mut stats = self.statistics.write().await;
                stats.total_nodes = stats.total_nodes.saturating_sub(1);
                *stats
                    .nodes_by_trust_level
                    .entry(node.trust_level)
                    .or_insert(0) = stats
                    .nodes_by_trust_level
                    .get(&node.trust_level)
                    .unwrap_or(&0)
                    .saturating_sub(1);
            }

            // Remove from trust manager
            self.trust_manager.remove_node(node_id).await?;

            // Unregister from phonebook if available
            if let Some(ref phonebook) = self.phonebook_service {
                if let Err(e) = phonebook.unregister_node(node_id).await {
                    warn!("Failed to unregister node from phonebook: {}", e);
                }
            }

            info!("✅ Node '{}' removed from registry", node_id);
        }

        Ok(())
    }

    /// Bootstrap the registry with initial nodes
    pub async fn bootstrap(&self) -> BearDogResult<()> {
        info!("🔄 Bootstrapping node registry");

        // Use bootstrap manager
        self.bootstrap_manager.bootstrap_registry(self).await?;

        // Start federation discovery if enabled
        if let Some(ref federation) = self.federation_manager {
            federation.start_federation_discovery().await?;
        }

        info!("✅ Registry bootstrap completed");
        Ok(())
    }

    /// Get trusted nodes with minimum trust level
    pub async fn get_trusted_nodes(
        &self,
        min_trust_level: TrustLevel,
    ) -> BearDogResult<Vec<NodeInfo>> {
        let nodes = self.nodes.read().await;
        let trusted_nodes = nodes
            .values()
            .filter(|node| node.trust_level >= min_trust_level)
            .cloned()
            .collect();

        Ok(trusted_nodes)
    }

    /// Verify a node's signature
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

        debug!("🔐 Verifying signature for node: {}", node_id);

        // Validate public key
        if node.public_key.len() != 32 {
            return Ok(false);
        }

        // Verify signature
        let is_valid = crate::crypto_utils::BearDogCrypto::verify_ed25519_signature(
            &node.public_key,
            data,
            signature,
        )?;

        if is_valid {
            debug!("✅ Signature verified for node {}", node_id);
            self.update_node_last_seen(node_id).await?;
        } else {
            warn!("❌ Invalid signature from node {}", node_id);
        }

        Ok(is_valid)
    }

    /// Get registry statistics
    pub async fn get_statistics(&self) -> RegistryStatistics {
        self.statistics.read().await.clone()
    }

    /// Get local registry information
    pub async fn get_local_registry_info(&self) -> LocalRegistryInfo {
        self.local_registry_info.read().await.clone()
    }

    /// Get federation status
    pub async fn get_federation_status(
        &self,
    ) -> Option<super::federation::FederationManagerStatus> {
        if let Some(ref federation) = self.federation_manager {
            Some(federation.get_federation_status().await)
        } else {
            None
        }
    }

    /// Get phonebook statistics
    pub async fn get_phonebook_status(&self) -> Option<super::phonebook::PhonebookStatus> {
        if let Some(ref phonebook) = self.phonebook_service {
            Some(phonebook.get_statistics().await)
        } else {
            None
        }
    }

    /// Health check for the registry
    pub async fn health_check(&self) -> BearDogResult<NodeRegistryHealthStatus> {
        let mut health = NodeRegistryHealthStatus {
            overall_status: HealthStatus::Healthy,
            local_nodes: 0,
            federation_status: None,
            phonebook_status: None,
            trust_relationships: 0,
            last_check: chrono::Utc::now(),
        };

        // Check local nodes
        {
            let nodes = self.nodes.read().await;
            health.local_nodes = nodes.len();
        }

        // Check federation
        if let Some(ref federation) = self.federation_manager {
            health.federation_status = Some(federation.health_check().await?);
        }

        // Check phonebook
        if let Some(ref phonebook) = self.phonebook_service {
            health.phonebook_status = Some(phonebook.get_statistics().await.health_status);
        }

        // Check trust relationships
        health.trust_relationships = self.trust_manager.get_relationship_count().await;

        Ok(health)
    }

    // Private methods

    async fn initialize_local_registry_info(
        config: &RegistryConfig,
    ) -> BearDogResult<LocalRegistryInfo> {
        // Generate or load cryptographic keys
        let (public_key, private_key) =
            crate::crypto_utils::BearDogCrypto::generate_ed25519_keypair()?;

        Ok(LocalRegistryInfo {
            registry_id: format!("beardog-registry-{}", uuid::Uuid::new_v4()),
            operator: "BearDog User".to_string(),
            public_key,
            private_key,
            endpoints: vec!["https://localhost:8843".to_string()],
            capabilities: vec![
                "security".to_string(),
                "trust-management".to_string(),
                "node-registry".to_string(),
            ],
            region: "local".to_string(),
            version: "1.0.0".to_string(),
            started_at: SystemTime::now(),
        })
    }

    async fn start_background_tasks(&self) -> BearDogResult<()> {
        // TODO: Start background tasks for:
        // 1. Periodic cleanup of inactive nodes
        // 2. Statistics collection
        // 3. Federation health checks
        // 4. Phonebook heartbeats

        Ok(())
    }
}

/// Health status information for the node registry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeRegistryHealthStatus {
    /// Overall health status of the node registry
    pub overall_status: HealthStatus,
    /// Number of nodes registered locally
    pub local_nodes: usize,
    /// Federation connection health status
    pub federation_status: Option<super::federation::FederationHealthStatus>,
    /// Phonebook service health status
    pub phonebook_status: Option<ServiceHealthStatus>,
    /// Number of active trust relationships
    pub trust_relationships: usize,
    /// Timestamp of last health check
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// Health status enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(PartialEq)]
pub enum HealthStatus {
    /// System is operating normally
    Healthy,
    /// System is functional but with reduced performance
    Degraded,
    /// System is not functioning properly
    Unhealthy,
    /// Health status cannot be determined
    Unknown,
}

// Implement NodeRegistry trait for backwards compatibility
impl NodeRegistry for BearDogNodeRegistry {
    fn get_node_info(&self, node_id: &str) -> BearDogResult<crate::auth::types::NodeInfo> {
        // This is a sync method but we need async - return a placeholder for now
        // TODO: Implement proper async trait support
        Err(BearDogError::config(
            "Async trait support needed - use async methods instead",
        ))
    }

    fn register_node(&mut self, node_info: crate::auth::types::NodeInfo) -> BearDogResult<()> {
        // This is a sync method but we need async - return a placeholder for now
        // TODO: Implement proper async trait support
        Err(BearDogError::config(
            "Async trait support needed - use async methods instead",
        ))
    }

    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64> {
        // This is a sync method but we need async - return a placeholder for now
        // TODO: Implement proper async trait support
        Err(BearDogError::config(
            "Async trait support needed - use async methods instead",
        ))
    }

    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()> {
        // This is a sync method but we need async - return a placeholder for now
        // TODO: Implement proper async trait support
        Err(BearDogError::config(
            "Async trait support needed - use async methods instead",
        ))
    }
}

impl BearDogNodeRegistry {
    /// Register a node with public key (async version)
    pub async fn register_node_with_key(
        &self,
        node_id: &str,
        public_key: &[u8],
    ) -> BearDogResult<()> {
        let node_info = NodeInfo {
            public_key: public_key.to_vec(),
            trust_level: TrustLevel::Unknown,
            capabilities: vec!["basic".to_string()],
            endpoints: vec![format!("node://{node_id}")],
            last_seen: SystemTime::now(),
            metadata: HashMap::new(),
            id: node_id.to_string(),
            name: format!("Node: {node_id}"),
            version: "1.0.0".to_string(),
            region: "default".to_string(),
            node_type: "beardog_security_node".to_string(),
            registered_at: SystemTime::now(),
        };

        self.add_node_with_id(node_id.to_string(), node_info).await
    }

    /// Check if a node is trusted
    pub async fn is_trusted_node(&self, node_id: &str) -> BearDogResult<bool> {
        let trust_level = self.get_trust_level(node_id).await?;
        Ok(trust_level >= TrustLevel::Basic)
    }
}
