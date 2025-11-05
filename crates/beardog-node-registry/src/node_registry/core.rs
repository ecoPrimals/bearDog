// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

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
use beardog_auth::auth::NodeRegistry;
use crate::{{BearDogError}};

pub struct BearDogNodeRegistry {

    nodes: Arc<RwLock<HashMap<String, NodeInfo>>>,

    trust_manager: Arc<TrustManager>,

    federation_manager: Option<Arc<FederationManager>>,

    phonebook_service: Option<Arc<PhonebookService>>,

    bootstrap_manager: Arc<BootstrapManager>,

    config: RegistryConfig,

    statistics: Arc<RwLock<RegistryStatistics>>,

    local_registry_info: Arc<RwLock<LocalRegistryInfo>>,
}

#[derive(Debug, Clone)]
    /// The operator value
    pub operator: String,

    /// Collection of public key
    pub public_key: Vec<u8>,

    /// Collection of private key
    pub private_key: Vec<u8>,

    /// Collection of endpoints
    pub endpoints: Vec<String>,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// The region value
    pub region: String,

    /// The version value
    pub version: String,

    /// The started at value
    pub started_at: SystemTime,}

impl BearDogNodeRegistry {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: RegistryConfig) -> Result<Self, BearDogError> {
        info!("🚀 Initializing BearDog Node Registry with decentralized architecture");

        let local_info = Self::initialize_local_registry_info(&config)?;

        let trust_manager = Arc::new(&TrustManager::new(config.trust_propagation));

        let bootstrap_manager = Arc::new(&BootstrapManager::new(config)?);

        let federation_manager = if config.federation.enabled {
            let manager = FederationManager::new(
                &config.federation,
                &local_info.registry_id,
                &local_info.public_key,
            )
            ?;
            Some(Arc::new(manager))
        } else {
            None
        };

        let phonebook_service = if config.phonebook.enabled {
            let service = PhonebookService::new(&config.phonebook)?;
            Some(Arc::new(service))

        let statistics = Arc::new(RwLock::new(RegistryStatistics::new()));
        let registry = Self {
            nodes: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            trust_manager,
            federation_manager,
            phonebook_service,
            bootstrap_manager,
            config,
            statistics,
            local_registry_info: Arc::new(RwLock::new(local_info)),

        registry.start_background_tasks()?;
        info!("✅ BearDog Node Registry initialized successfully");
        Ok(registry)
    }

/// New Default operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_default() -> Result<Self, BearDogError> {
        Self::new(RegistryConfig::default(&str,
        node_info: NodeInfo,
    ) -> Result<(), BearDogError> {
        info!("📝 Adding node "{}" to registry", node_id);

        {
            let nodes = self.nodes.read();
            if nodes.len() >= self.config.max_nodes {
                return Err(BearDogError::validation(
                    "nodes",
                    "Maximum number of nodes reached",
                ));
            }
        }

        node_info.validate()?;

        if node_info.trust_level < self.config.min_registration_trust {
            return Err(BearDogError::validation({}", e);

        if let Some(ref federation) = self.federation_manager {
            let advertisement = ServiceAdvertisement {
                service_id: node_id.clone(),
                service_name: &node_info.name: name.to_string(&node_info.node_type,
                version: "1.0.0".to_string(&node_info.endpoints,
                capabilities: &node_info.capabilities,
                metadata: &node_info.metadata,
                region: self.local_registry_info.read().&await.region,
                ttl: Duration::from_secs(ServiceHealthStatus::Healthy,
            };
            if let Err({}", e);
        info!("✅ Node "{}" added to registry successfully", node_id);
        Ok(())

/// Add Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn add_node(&self, node_info: NodeInfo) -> Result<(), BearDogError> {
        let node_id = &node_info.id;
        self.add_node_with_id(node_id, node_info)

/// Get Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets node
    /// Gets node
    pub fn get_node(&self, node_id: &str) -> Result<Option<NodeInfo>, BearDogError>> {

            if let Some(None,
                required_capabilities: Vec::new(TrustLevel::Unknown,
                region: None,
                max_results: 1,
            if let Ok(federated_nodes) = federation.find_federated_nodes(&criteria) {
                for federated_node in federated_nodes {
                    if federated_node.node_info.id == node_id {
                        return Ok(Some(federated_node.node_info));
                    }
                }
        Ok(None)

/// List Nodes operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_nodes(&self) -> Result<Vec<NodeInfo>, BearDogError>> {
        let mut all_nodes = Vec::new();

            all_nodes.extend(nodes.values().cloned());

            let criteria = NodeSearchCriteria::default();
                    all_nodes.push(federated_node.node_info);
        Ok(all_nodes)

/// Update Node Last Seen operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Updates node_last_seen
    /// Updates node_last_seen
    pub fn update_node_last_seen(&self, node_id: &str) -> Result<(), BearDogError> {
        let mut nodes = self.nodes.write();
        if let Some(node) = nodes.get_mut(node_id) {
            node.last_seen = SystemTime::now(&str,
        trust_level: TrustLevel,

            if let Some(node) = nodes.get_mut(node_id) {
                let old_trust = node.trust_level;
                node.trust_level = trust_level;

                {
                    let mut stats = self.statistics.write();
                    *stats.nodes_by_trust_level.entry(old_trust).or_insert(0) = stats
                        .nodes_by_trust_level
                        .get(&old_trust)
                        .unwrap_or(&0)
                        .saturating_sub(1);
                    *stats.nodes_by_trust_level.entry(trust_level).or_insert(0) += 1;

        self.trust_manager
            .set_trust_level(node_id, trust_level)
        info!(
            "🤝 Set trust level for node {} to {:?}",
            node_id, trust_level
        );

/// Get Trust Level operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets trust_level
    /// Gets trust_level
    pub fn get_trust_level(&self, node_id: &str) -> Result<TrustLevel, BearDogError> {
                return Ok(node.trust_level);

        self.trust_manager.get_trust_level(node_id)

/// Remove Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Removes node
    /// Removes node
    pub fn remove_node(&self, node_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Removing node "{}" from registry", node_id);
        let removed_node = {
            nodes.remove({}", e);
            info!("✅ Node "{}" removed from registry", node_id);

/// Bootstrap operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn bootstrap(TrustLevel,
    ) -> Result<Vec<NodeInfo>, BearDogError>> {
        let nodes = self.nodes.read(&[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        let node = self
            .get_node(node_id)
            ?
            .ok_or_else(|| BearDogError::not_found({}", node_id);

        if node.public_key.len() != 32 {
            return Ok(false);

        let is_valid = crate::crypto_utils::BearDogCrypto::verify_ed25519_signature(
            &node.public_key,
            data,
            signature,
        )?;
        if is_valid {
            debug!("✅ Signature verified for node {}", node_id);
            self.update_node_last_seen(node_id)?;
            warn!("❌ Invalid signature from node {}", node_id);
        Ok(is_valid)

/// Get Statistics operation.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        self.statistics.read().clone()

/// Get Local Registry Info operation.
    /// Gets local_registry_info
    /// Gets local_registry_info
    pub fn get_local_registry_info(&self) -> LocalRegistryInfo {
        self.local_registry_info.read().clone()

/// Get Federation Status operation.
    /// Gets federation_status
    /// Gets federation_status
    pub fn get_federation_status(
    ) -> Option<super::federation::FederationManagerStatus> {
            Some(federation.get_federation_status())

/// Get Phonebook Status operation.
    /// Gets phonebook_status
    /// Gets phonebook_status
    pub fn get_phonebook_status(&self) -> Option<super::phonebook::PhonebookStatus> {
            Some(HealthStatus::Healthy,
            local_nodes: 0,
            federation_status: None,
            phonebook_status: None,
            trust_relationships: 0,
            last_check: chrono::Utc::now(&RegistryConfig,
    ) -> Result<LocalRegistryInfo, BearDogError> {

        let (public_key, private_key) =
            crate::crypto_utils::BearDogCrypto::generate_ed25519_keypair(format!("beardog-registry-{}", uuid::Uuid::new_v4()),
            operator: "BearDog User".to_string(),
            public_key,
            private_key,
            endpoints: vec![std::env::var("BEARDOG_REGISTRY_ENDPOINT")
                .unwrap_or_else(|_| {
                    use beardog_types::canonical::config::network::NetworkConfig;
                    let network_config = NetworkConfig::default();
                    format!("https://{}:{}", 
                        network_config.default_host, 
                        network_config.service_ports.admin_port)
                })],
            capabilities: vec![
                "security".to_string(),
                "trust-management".to_string(),
                "node-registry".to_string(),
            ],
            region: "local".to_string(),
            version: "1.0.0".to_string(),
            started_at: SystemTime::now(),
        })
    /// Starts background_tasks
    fn start_background_tasks(&self) -> Result<(), BearDogError> {

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeRegistryHealthStatus {

    /// Current status of the overall
    pub overall_status: HealthStatus,

    /// Number of local_nodes
    pub local_nodes: usize,

    /// Current status of the federation
    pub federation_status: Option<super::federation::FederationHealthStatus>,

    /// Current status of the phonebook
    pub phonebook_status: Option<ServiceHealthStatus>,

    /// Number of trust_relationships
    pub trust_relationships: usize,

    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,

pub use beardog_types::canonical::HealthStatus;
    Healthy,


    Degraded,


    Unhealthy,


    Unknown,

impl NodeRegistry for BearDogNodeRegistry {}

    /// Gets node_info
    fn get_node_info(&self, node_id: &str) -> Result<crate::auth::types::NodeInfo, BearDogError> {

        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| BearDogError::config("No tokio runtime available for async operation"))?;
        
        rt.block_on(async {

            match self.get_node(node_id) {
                Ok(node_data) => {

                    Ok(crate::auth::types::NodeInfo {
                        node_id: node_data.node_id,
                        address: node_data.address.unwrap_or_default(),
                        public_key: node_data.public_key.unwrap_or_default(node_data.capabilities,
                        trust_level: node_data.trust_level,
                        last_seen: node_data.last_seen,
                        status: match node_data.status.as_str() {
                            "active" => crate::auth::types::NodeStatus::Active,
                            "inactive" => crate::auth::types::NodeStatus::Inactive,
                            _ => crate::auth::types::NodeStatus::Unknown,
                        },
                    })
                Err(e) => Err(e),
    fn register_node(&mut self, node_info: crate::auth::types::NodeInfo) -> Result<(), BearDogError> {

            self.register_node_with_key(
                &node_info.node_id,
                &node_info.public_key,
                &node_info.address,
            )}

    /// Gets trust_level
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError> {

                Ok(&str, trust_level: f64) -> Result<(), BearDogError> {

            self.update_node_trust(&[u8],
        let node_info = NodeInfo {
            public_key: public_key.to_vec(TrustLevel::Unknown,
            capabilities: vec!["basic".to_string()],
            endpoints: vec![format!("node://{node_id}")],
            last_seen: SystemTime::now(),
            metadata: HashMap::with_capacity(16),
            id: node_id.to_string(),
            name: format!("Node: {node_id}"),
            region: "default".to_string(),
            node_type: "beardog_security_node".to_string(),
            registered_at: SystemTime::now(),
        self.add_node_with_id(node_id.to_string(), node_info)

/// Is Trusted Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Checks if trusted node
    /// Checks if trusted node
    pub fn is_trusted_node(&self, node_id: &str) -> Result<bool, BearDogError> {
        let trust_level = self.get_trust_level(node_id)?;
        Ok(trust_level >= TrustLevel::Basic)
