// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Node Discovery for Bootstrap Process
///
/// Handles discovery of bootstrap nodes from various sources including
/// phonebook services, environment variables, and configured endpoints.

use crate::{BearDogError, BearDogResult};
use crate::node_registry::types::{NodeInfo, TrustLevel};
use super::types::{DiscoveredNode, PhonebookDiscoveryResponse, BootstrapConfig};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tokio::time::{timeout, Duration};
/// Node discovery service for bootstrap operations
#[derive(Debug)]
pub struct NodeDiscovery {
    /// Bootstrap configuration
    config: BootstrapConfig,
    /// HTTP client for API calls
    client: reqwest::Client,
}
impl NodeDiscovery {
    /// Create new node discovery service}


    pub fn new(config: BootstrapConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
        Self { config, client }
    }
    /// Discover nodes from environment variables
    pub async fn discover_from_environment(&self) -> BearDogResult<Vec<NodeInfo>> {
        debug!("🔍 Discovering bootstrap nodes from environment variables");
        
        let mut nodes = Vec::new();
        // Check common environment variables
        if let Ok(node_list) = std::env::var("BEARDOG_BOOTSTRAP_NODES") {
            for node_addr in node_list.split(',') {
                let addr = node_addr.trim();
                if !addr.is_empty() {
                    if let Ok(node_info) = self.parse_node_address(addr).await {
                        nodes.push(node_info);
                    }
                }
            }
        }
        // Check individual node environment variables
        for i in 1..=10 {
            if let Ok(node_addr) = std::env::var(format!("BEARDOG_BOOTSTRAP_NODE_{}", i)) {
                if let Ok(node_info) = self.parse_node_address(&node_addr).await {
                    nodes.push(node_info);
        info!("📋 Discovered {} nodes from environment", nodes.len());
        Ok(nodes)
    /// Discover nodes from phonebook services
    pub async fn discover_from_phonebooks(&self) -> BearDogResult<Vec<NodeInfo>> {
        debug!("📞 Discovering nodes from phonebook services");
        let mut all_nodes = Vec::new();
        for phonebook_url in &self.config.phonebook_urls {
            match self.discover_from_phonebook(phonebook_url).await {
                Ok(mut nodes) => {
                    info!("📞 Discovered {} nodes from {}", nodes.len(), phonebook_url);
                    all_nodes.append(&mut nodes);
                Err(e) => {
                    warn!("Failed to discover from phonebook {}: {}", phonebook_url, e);
        // Remove duplicates based on node_id
        all_nodes.dedup_by(|a, b| a.node_id == b.node_id);
        info!("📋 Total discovered {} unique nodes from phonebooks", all_nodes.len());
        Ok(all_nodes)
    /// Discover nodes from a single phonebook service
    async fn discover_from_phonebook(&self, phonebook_url: &str) -> BearDogResult<Vec<NodeInfo>> {
        let discovery_url = format!("{}/api/v1/nodes/discover", phonebook_url);
        // Build discovery request
        let mut request = self.client.get(&discovery_url);
        // Add custom headers if configured
        for (header, value) in &self.config.custom_headers {
            request = request.header(header, value);
        // Add query parameters for regional preferences
        if self.config.prefer_regional_nodes && !self.config.preferred_regions.is_empty() {
            let regions = self.config.preferred_regions.join(",");
            request = request.query(&[("preferred_regions", regions)]);
        // Execute request with timeout
        let response = timeout(
            Duration::from_secs(self.config.bootstrap_timeout_seconds),
            request.send()
        ).await
        .map_err(|_| BearDogError::timeout("Phonebook discovery request timed out"))?
        .map_err(|e| BearDogError::network(format!("Phonebook request failed: {}", e)))?;
        if !response.status().is_success() {
            return Err(BearDogError::network(format!(
                "Phonebook returned error: {}", 
                response.status()
            )));
        // Parse phonebook response
        let discovery_response: PhonebookDiscoveryResponse = response
            .json()
            .await
            .map_err(|e| BearDogError::parsing(format!("Failed to parse phonebook response: {}", e)))?;
        // Convert discovered nodes to NodeInfo
        for discovered in discovery_response.nodes {
            let node_info = NodeInfo {
                node_id: discovered.node_id,
                address: discovered.address,
                public_key: discovered.public_key,
                capabilities: discovered.capabilities,
                trust_level: discovered.trust_level,
                last_seen: Some(discovered.last_seen),
                metadata: HashMap::new(),
            };
            nodes.push(node_info);
    /// Parse node address from string format
    async fn parse_node_address(&self, addr: &str) -> BearDogResult<NodeInfo> {
        // Support formats: "host:port", "http://host:port", "node_id@host:port"
        let (node_id, address) = if addr.contains('@') {
            let parts: Vec<&str> = addr.splitn(2, '@').collect();
            if parts.len() == 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                return Err(BearDogError::validation("Invalid node address format"));
        } else {
            // Generate node_id from address
            let node_id = format!("bootstrap_{}", addr.replace(':', "_").replace('.', "_"));
            (node_id, addr.to_string())
        };
        // Normalize address format
        let normalized_address = if address.starts_with("http://") || address.starts_with("https://") {
            address
            format!("http://{}", address)
        Ok(NodeInfo {
            node_id,
            address: normalized_address,
            public_key: String::new(), // Will be discovered during verification
            capabilities: Vec::new(),   // Will be discovered during handshake
            trust_level: TrustLevel::Untrusted, // Default until verified
            last_seen: None,
            metadata: HashMap::new(),
        })
    /// Filter discovered nodes based on configuration preferences
    pub fn filter_discovered_nodes(&self, nodes: Vec<NodeInfo>) -> Vec<NodeInfo> {
        let mut filtered = nodes;
        // Apply regional filtering if enabled
            filtered.retain(|node| {
                // Simple regional filtering based on address patterns
                self.config.preferred_regions.iter().any(|region| {
                    node.address.contains(region) || 
                    node.metadata.get("region").map_or(false, |r| r.as_str() == Some(region))
                })
            });
        // Limit to reasonable number of nodes
        if filtered.len() > 50 {
            filtered.truncate(50);
        filtered
    /// Validate discovered node information
    pub fn validate_node(&self, node: &NodeInfo) -> BearDogResult<()> {
        // Basic validation
        if node.node_id.is_empty() {
            return Err(BearDogError::validation("Node ID cannot be empty"));
        if node.address.is_empty() {
            return Err(BearDogError::validation("Node address cannot be empty"));
        // Validate address format
        if !node.address.starts_with("http://") && !node.address.starts_with("https://") {
            return Err(BearDogError::validation("Node address must include protocol"));
        Ok(())
    /// Get discovery statistics}


    pub async fn get_discovery_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        // Count phonebook endpoints
        stats.insert("phonebook_endpoints".to_string(), self.config.phonebook_urls.len() as u32);
        // Count preferred regions
        stats.insert("preferred_regions".to_string(), self.config.preferred_regions.len() as u32);
        // Regional preference enabled
        stats.insert("regional_preference".to_string(), if self.config.prefer_regional_nodes { 1 } else { 0 });
        stats
} 
