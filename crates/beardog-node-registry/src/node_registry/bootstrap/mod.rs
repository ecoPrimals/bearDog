

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

    pub fn create_services(config: BootstrapConfig) -> BootstrapServices {
        BootstrapServices {
            discovery: NodeDiscovery::new(config.clone()),
            verification: NodeVerification::new(config.clone()),
            federation: FederationBootstrap::new(config),
        }
    }
}

pub struct BootstrapServices {

    pub discovery: NodeDiscovery,

    pub verification: NodeVerification,

    pub federation: FederationBootstrap,}

impl BootstrapServices {

    pub async fn comprehensive_bootstrap(&self) -> crate::BearDogResult<Vec<crate::node_registry::types::NodeInfo>> {
        use tracing::{info, warn};
        
        let mut all_nodes = Vec::new();

        match self.discovery.discover_from_environment().await {
            Ok(mut env_nodes) => {
                info!("🌍 Environment discovery: {} nodes", env_nodes.len());
                all_nodes.append(&mut env_nodes);
            }
            Err(e) => warn!("Environment discovery failed: {}", e),

        match self.discovery.discover_from_phonebooks().await {
            Ok(mut phonebook_nodes) => {
                info!("📞 Phonebook discovery: {} nodes", phonebook_nodes.len());
                all_nodes.append(&mut phonebook_nodes);
            Err(e) => warn!("Phonebook discovery failed: {}", e),

        match self.federation.bootstrap_from_federation().await {
            Ok(mut federation_nodes) => {
                info!("🌐 Federation discovery: {} nodes", federation_nodes.len());
                all_nodes.append(&mut federation_nodes);
            Err(e) => warn!("Federation discovery failed: {}", e),

        all_nodes = self.discovery.filter_discovered_nodes(all_nodes);

        let mut verified_nodes = Vec::new();
        for node in all_nodes {
            match self.verification.verify_node(&node).await {
                Ok(true) => {
                    info!("✅ Verified node: {}", node.node_id);
                    verified_nodes.push(node);
                }
                Ok(false) => {
                    warn!("❌ Failed to verify node: {}", node.node_id);
                Err(e) => {
                    warn!("⚠️ Verification error for {}: {}", node.node_id, e);
        info!("🎉 Bootstrap complete: {} verified nodes", verified_nodes.len());
        Ok(verified_nodes)

    pub async fn get_comprehensive_stats(&self) -> std::collections::HashMap<String, u32> {
        let mut stats = std::collections::HashMap::with_capacity(16);

        let discovery_stats = self.discovery.get_discovery_stats().await;
        for (key, value) in discovery_stats {
            stats.insert(format_args!("discovery_{}", key).to_string(), value);

        let verification_stats = self.verification.get_verification_stats();
        for (key, value) in verification_stats {
            stats.insert(format_args!("verification_{}", key).to_string(), value);

        let federation_stats = self.federation.get_federation_stats();
        for (key, value) in federation_stats {
            stats.insert(format_args!("federation_{}", key).to_string(), value);
        stats
} 
