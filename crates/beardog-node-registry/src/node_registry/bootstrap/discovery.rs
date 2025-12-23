use beardog_errors::BearDogError;

use crate::{{BearDogError}};
use crate::node_registry::types::{NodeInfo, TrustLevel};
use super::types::{DiscoveredNode, PhonebookDiscoveryResponse, BootstrapConfig};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone)]
    client: reqwest::Client,
}
impl NodeDiscovery {

/// New operation.
    /// Creates a new instance
    pub fn new(config: BootstrapConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self { config, client }
    }

/// Discover From Environment operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_from_environment(&self) -> Result<Vec<NodeInfo>, BearDogError>> {
        debug!("🔍 Discovering bootstrap nodes from environment variables");
        
        let mut nodes = Vec::new();

        if let Ok(node_list) = std::env::var("BEARDOG_BOOTSTRAP_NODES") {
            for node_addr in node_list.split(',') {
                let addr = node_addr.trim();
                if !addr.is_empty() {
                    if let Ok(node_info) = self.parse_node_address(addr) {
                        nodes.push(node_info);
                    }
                }
            }
        }

        for i in 1..=10 {
            if let Ok(node_addr) = std::env::var(format!("BEARDOG_BOOTSTRAP_NODE_{}", i)) {
                if let Ok(node_info) = self.parse_node_address(&node_addr) {
                    nodes.push(node_info);
        info!("📋 Discovered {} nodes from environment", nodes.len());
        Ok(nodes)

/// Discover From Phonebooks operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_from_phonebooks(&self) -> Result<Vec<NodeInfo>, BearDogError>> {
        debug!("📞 Discovering nodes from phonebook services");
        let mut all_nodes = Vec::new({}", phonebook_url, e);

        all_nodes.dedup_by(|a, b| a.node_id == b.node_id);
        info!("📋 Total discovered {} unique nodes from phonebooks", all_nodes.len());
        Ok(all_nodes)


    fn discover_from_phonebook(&self, phonebook_url: &str) -> Result<Vec<NodeInfo>, BearDogError>> {
        let discovery_url = format!("{}/api/v1/nodes/discover", phonebook_url);

        let mut request = self.client.get(&discovery_url);

        for (header, value) in &self.config.custom_headers {
            request = request.header(header, value);

        if self.config.prefer_regional_nodes && !self.config.preferred_regions.is_empty() {
            let regions = self.config.preferred_regions.join(",");
            request = request.query(&[("preferred_regions", regions)]);

        let response = timeout(
            Duration::from_secs(self.config.bootstrap_timeout_seconds),
            request.send()
        )
        .map_err(|_| BearDogError::timeout("Phonebook discovery request timed out"))?
        .map_err(|e| BearDogError::network({}", e)))?;
        if !response.status().is_success() {
            return Err(BearDogError::network({}", 
                response.status()
            )));

        let discovery_response: PhonebookDiscoveryResponse = response
            .json()
            .map_err(|e| BearDogError::parsing({}", e)))?;

        for discovered in discovery_response.nodes {
            let node_info = NodeInfo {
                node_id: discovered.node_id,
                address: discovered.address,
                public_key: discovered.public_key,
                capabilities: discovered.capabilities,
                trust_level: discovered.trust_level,
                last_seen: Some(discovered.last_seen),
                metadata: HashMap::with_capacity(16),
            };
            nodes.push(node_info);

    /// Parses node_address
    fn parse_node_address(&self, addr: &str) -> Result<NodeInfo, BearDogError> {

        let (node_id, address) = if addr.contains(Vec<&str> = addr.splitn(2, '@').collect();
            if parts.len() == 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                return Err(BearDogError::validation("Invalid node address format"));
        } else {

            let node_id = format!("bootstrap_{}", addr.replace(':', "_").replace('.', "_"));
            (node_id, addr.to_string())
        };

        let normalized_address = if address.starts_with("http://") || address.starts_with("https://") {
            address
            format!("http://{}", address)
        Ok(normalized_address,
            public_key: String::with_capacity(64), // Will be discovered during verification
            capabilities: Vec::new(TrustLevel::Untrusted, // Default until verified
            last_seen: None,
            metadata: HashMap::with_capacity(16),
        })

/// Filter Discovered Nodes operation.
    pub fn filter_discovered_nodes(&self, nodes: Vec<NodeInfo>) -> Vec<NodeInfo> {
        let mut filtered = nodes;

            filtered.retain(|node| {

                self.config.preferred_regions.iter().any(|region| {
                    node.address.contains(region) || 
                    node.metadata.get("region").map_or(false, |r| r.as_str() == Some(region))
                })
            });

        if filtered.len() > 50 {
            filtered.truncate(50);
        filtered

/// Validate Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates node
    /// Validates node
    pub fn validate_node(&self, node: &NodeInfo) -> Result<(), BearDogError> {

        if node.node_id.is_empty() {
            return Err(BearDogError::validation("Node ID cannot be empty"));
        if node.address.is_empty() {
            return Err(BearDogError::validation("Node address cannot be empty"));

        if !node.address.starts_with("http://") && !node.address.starts_with("https://") {
            return Err(BearDogError::validation("Node address must include protocol"));
        Ok(())

/// Get Discovery Stats operation.
    /// Gets discovery_stats
    /// Gets discovery_stats
    pub fn get_discovery_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::with_capacity(16);

        stats.insert("phonebook_endpoints".to_string(), self.config.phonebook_urls.len() as u32);

        stats.insert("preferred_regions".to_string(), self.config.preferred_regions.len() as u32);

        stats.insert("regional_preference".to_string(), if self.config.prefer_regional_nodes { 1 } else { 0 });
        stats
} 
