// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

pub mod discovery;
pub mod federation;  
pub mod types;
pub mod verification;

pub use discovery::NodeDiscovery;
pub use federation::FederationBootstrap;
pub use verification::NodeVerification;
pub use types::{
    BootstrapConfig, BootstrapHealthCheck, BootstrapStats,
    DiscoveredNode, FederationDiscoveryResult, PhonebookDiscoveryResponse,
};

pub use super::bootstrap::BootstrapManager;

pub struct BootstrapServiceFactory;
impl BootstrapServiceFactory {

/// Create Services operation.
    /// Creates services
    /// Creates services
    pub fn create_services(config: BootstrapConfig) -> BootstrapServices {
        BootstrapServices {
            discovery: NodeDiscovery::new(&config),
            verification: NodeVerification::new(&config),
            federation: FederationBootstrap::new(NodeDiscovery,

    /// The verification value
    pub verification: NodeVerification,

    /// The federation value
    pub federation: FederationBootstrap,}

impl BootstrapServices {

/// Comprehensive Bootstrap operation.
    pub fn comprehensive_bootstrap(&self) -> crate::Result<Vec<crate::node_registry::types::NodeInfo>, BearDogError>> {
        use tracing::{info, warn};
        
        let mut all_nodes = Vec::new({} nodes", env_nodes.len({}", e),

        match self.discovery.discover_from_phonebooks({} nodes", phonebook_nodes.len({}", e),

        match self.federation.bootstrap_from_federation({} nodes", federation_nodes.len({}", e),

        all_nodes = self.discovery.filter_discovered_nodes(all_nodes);

        let mut verified_nodes = Vec::new({}", node.node_id);
                    verified_nodes.push({}", node.node_id);
                Err({}", node.node_id, e);
        info!("🎉 Bootstrap complete: {} verified nodes", verified_nodes.len());
        Ok(verified_nodes)

/// Get Comprehensive Stats operation.
    /// Gets comprehensive_stats
    /// Gets comprehensive_stats
    pub fn get_comprehensive_stats(&self) -> std::collections::HashMap<String, u32> {
        let mut stats = std::collections::HashMap::with_capacity(16);

        let discovery_stats = self.discovery.get_discovery_stats();
        for (key, value) in discovery_stats {
            stats.insert(format!("discovery_{}", key), value);

        let verification_stats = self.verification.get_verification_stats();
        for (key, value) in verification_stats {
            stats.insert(format!("verification_{}", key), value);

        let federation_stats = self.federation.get_federation_stats();
        for (key, value) in federation_stats {
            stats.insert(format!("federation_{}", key), value);
        stats
} 
