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


/// Federation Bootstrap for Node Registry
///
/// Handles discovery and bootstrapping from federated registries and networks,
/// enabling cross-registry node discovery and trust establishment.

use crate::{BearDogError, BearDogResult};
use crate::node_registry::types::{NodeInfo, TrustLevel};
use super::types::{BootstrapConfig, FederationDiscoveryResult};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use tokio::time::{timeout, Duration};
use serde_json::Value;
/// Federation discovery and bootstrap service
#[derive(Debug)]
pub struct FederationBootstrap {
    /// Bootstrap configuration
    config: BootstrapConfig,
    /// HTTP client for federation requests
    client: reqwest::Client,
    /// Cache of discovered federations
    federation_cache: std::sync::RwLock<HashMap<String, (Vec<FederationDiscoveryResult>, std::time::Instant)>>,
}
impl FederationBootstrap {
    /// Create new federation bootstrap service}


    pub fn new(config: BootstrapConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(45)) // Longer timeout for federation
            .build()
            .unwrap_or_default();
        Self {
            config,
            client,
            federation_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }
    /// Bootstrap from federation partners
    pub async fn bootstrap_from_federation(&self) -> BearDogResult<Vec<NodeInfo>> {
        if !self.config.enable_federation_discovery {
            debug!("🚫 Federation discovery disabled");
            return Ok(Vec::new());
        debug!("🌐 Starting federation bootstrap process");
        
        let mut all_nodes = Vec::new();
        // Try each configured federation bootstrap node
        for federation_endpoint in &self.config.phonebook_urls {
            if federation_endpoint.contains("federation") {
                match self.bootstrap_from_federation_endpoint(federation_endpoint).await {
                    Ok(mut nodes) => {
                        info!("🌐 Discovered {} nodes from federation {}", nodes.len(), federation_endpoint);
                        all_nodes.append(&mut nodes);
                    }
                    Err(e) => {
                        warn!("Failed to bootstrap from federation {}: {}", federation_endpoint, e);
                }
            }
        // Remove duplicates
        all_nodes.dedup_by(|a, b| a.node_id == b.node_id);
        info!("🌐 Total discovered {} nodes from federation", all_nodes.len());
        Ok(all_nodes)
    /// Bootstrap from a single federation endpoint
    async fn bootstrap_from_federation_endpoint(&self, endpoint: &str) -> BearDogResult<Vec<NodeInfo>> {
        // First, discover the federation network
        let federation_network = self.discover_federation_network(endpoint).await?;
        let mut nodes = Vec::new();
        // Bootstrap from each registry in the federation
        for registry_info in federation_network.get("registries").and_then(|r| r.as_array()).unwrap_or(&Vec::new()) {
            if let Some(registry_endpoints) = registry_info.get("endpoints").and_then(|e| e.as_array()) {
                for endpoint_value in registry_endpoints {
                    if let Some(endpoint_url) = endpoint_value.as_str() {
                        match self.bootstrap_from_registry_endpoint(endpoint_url).await {
                            Ok(mut registry_nodes) => {
                                nodes.append(&mut registry_nodes);
                            }
                            Err(e) => {
                                warn!("Failed to bootstrap from registry {}: {}", endpoint_url, e);
                        }
        Ok(nodes)
    /// Discover federation network information
    async fn discover_federation_network(&self, endpoint: &str) -> BearDogResult<Value> {
        let discovery_url = format!("{}/api/v1/federation/network", endpoint);
        // Check cache first
        if let Some((cached_results, timestamp)) = self.get_cached_federation(endpoint) {
            if timestamp.elapsed() < Duration::from_secs(600) { // 10-minute cache
                return Ok(serde_json::to_value(&cached_results).unwrap_or_default());
        debug!("🔍 Discovering federation network from {}", endpoint);
        let response = timeout(
            Duration::from_secs(self.config.bootstrap_timeout_seconds),
            self.client.get(&discovery_url).send()
        ).await
        .map_err(|_| BearDogError::timeout("Federation discovery timed out"))?
        .map_err(|e| BearDogError::network(format!("Federation request failed: {}", e)))?;
        if !response.status().is_success() {
            return Err(BearDogError::network(format!(
                "Federation endpoint returned error: {}",
                response.status()
            )));
        let federation_info: Value = response
            .json()
            .await
            .map_err(|e| BearDogError::parsing(format!("Failed to parse federation response: {}", e)))?;
        // Cache the result
        self.cache_federation_results(endpoint, &federation_info).await;
        Ok(federation_info)
    /// Bootstrap from a specific registry endpoint
    async fn bootstrap_from_registry_endpoint(&self, endpoint: &str) -> BearDogResult<Vec<NodeInfo>> {
        let registry_url = format!("{}/api/v1/registry/nodes", endpoint);
        debug!("🔗 Bootstrapping from registry endpoint: {}", endpoint);
            self.client.get(&registry_url).send()
        .map_err(|_| BearDogError::timeout("Registry bootstrap timed out"))?
        .map_err(|e| BearDogError::network(format!("Registry request failed: {}", e)))?;
                "Registry returned error: {}",
        let registry_response: Value = response
            .map_err(|e| BearDogError::parsing(format!("Failed to parse registry response: {}", e)))?;
        // Parse nodes from registry response
        if let Some(node_array) = registry_response.get("nodes").and_then(|n| n.as_array()) {
            for node_value in node_array {
                if let Ok(node_info) = self.parse_registry_node(node_value).await {
                    nodes.push(node_info);
        info!("📋 Discovered {} nodes from registry {}", nodes.len(), endpoint);
    /// Parse node information from registry response
    async fn parse_registry_node(&self, node_value: &Value) -> BearDogResult<NodeInfo> {
        let node_id = node_value.get("node_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::parsing("Missing node_id in registry response"))?
            .to_string();
        let address = node_value.get("address")
            .ok_or_else(|| BearDogError::parsing("Missing address in registry response"))?
        let public_key = node_value.get("public_key")
            .unwrap_or("")
        // Parse capabilities
        let capabilities = if let Some(caps_array) = node_value.get("capabilities").and_then(|c| c.as_array()) {
            caps_array
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };
        // Parse trust level
        let trust_level = node_value.get("trust_level")
            .and_then(|s| match s {
                "untrusted" => Some(TrustLevel::Untrusted),
                "low" => Some(TrustLevel::Low),
                "medium" => Some(TrustLevel::Medium),
                "high" => Some(TrustLevel::High),
                _ => None,
            })
            .unwrap_or(TrustLevel::Low);
        let last_seen = node_value.get("last_seen")
            .map(|s| s.to_string());
        // Parse metadata
        let metadata = if let Some(meta_obj) = node_value.get("metadata").and_then(|m| m.as_object()) {
            meta_obj
                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            HashMap::new()
        Ok(NodeInfo {
            node_id,
            address,
            public_key,
            capabilities,
            trust_level,
            last_seen,
            metadata,
        })
    /// Discover federation partners for a given registry
    pub async fn discover_federation_partners(&self, registry_endpoint: &str) -> BearDogResult<Vec<FederationDiscoveryResult>> {
        let partners_url = format!("{}/api/v1/federation/partners", registry_endpoint);
        debug!("🤝 Discovering federation partners from {}", registry_endpoint);
            self.client.get(&partners_url).send()
        .map_err(|_| BearDogError::timeout("Federation partners discovery timed out"))?
        .map_err(|e| BearDogError::network(format!("Partners request failed: {}", e)))?;
                "Partners endpoint returned error: {}",
        let partners_response: Value = response
            .map_err(|e| BearDogError::parsing(format!("Failed to parse partners response: {}", e)))?;
        let mut partners = Vec::new();
        if let Some(partners_array) = partners_response.get("partners").and_then(|p| p.as_array()) {
            for partner_value in partners_array {
                if let Ok(partner_result) = self.parse_federation_partner(partner_value).await {
                    partners.push(partner_result);
        info!("🤝 Discovered {} federation partners", partners.len());
        Ok(partners)
    /// Parse federation partner information
    async fn parse_federation_partner(&self, partner_value: &Value) -> BearDogResult<FederationDiscoveryResult> {
        let partner_info = NodeInfo {
            node_id: partner_value.get("node_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            address: partner_value.get("address")
                .unwrap_or("")
            public_key: partner_value.get("public_key")
            capabilities: Vec::new(),
            trust_level: TrustLevel::Low,
            last_seen: None,
            metadata: HashMap::new(),
        let trust_level = partner_value.get("trust_level")
        let shared_capabilities = if let Some(caps_array) = partner_value.get("shared_capabilities").and_then(|c| c.as_array()) {
        let discovered_at = partner_value.get("discovered_at")
            .unwrap_or("unknown")
        Ok(FederationDiscoveryResult {
            partner_info,
            shared_capabilities,
            discovered_at,
    /// Get cached federation results}


    fn get_cached_federation(&self, endpoint: &str) -> Option<(Vec<FederationDiscoveryResult>, std::time::Instant)> {
        let cache = self.federation_cache.read().ok()?;
        cache.get(endpoint).cloned()
    /// Cache federation results
    async fn cache_federation_results(&self, endpoint: &str, federation_info: &Value) {
        // Convert federation info to discovery results for caching
        let mut results = Vec::new();
        // This is a simplified caching approach
        if let Some(registries) = federation_info.get("registries").and_then(|r| r.as_array()) {
            for registry in registries {
                if let Ok(result) = self.parse_federation_partner(registry).await {
                    results.push(result);
        if let Ok(mut cache) = self.federation_cache.write() {
            cache.insert(endpoint.to_string(), (results, std::time::Instant::now()));
            
            // Limit cache size
            if cache.len() > 100 {
                let cutoff = std::time::Instant::now() - Duration::from_secs(1200);
                cache.retain(|_, (_, timestamp)| *timestamp > cutoff);
    /// Validate federation node for trust establishment}


    pub fn validate_federation_node(&self, node: &NodeInfo) -> BearDogResult<bool> {
        // Basic federation node validation
        if node.node_id.is_empty() || node.address.is_empty() {
            return Ok(false);
        // Federation nodes should have some form of public key
        if node.public_key.is_empty() {
            warn!("⚠️ Federation node {} has no public key", node.node_id);
        // Check trust level is appropriate for federation
        match node.trust_level {
            TrustLevel::Untrusted => Ok(false),
            TrustLevel::Low | TrustLevel::Medium | TrustLevel::High => Ok(true),
    /// Get federation statistics}


    pub fn get_federation_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("federation_enabled".to_string(), if self.config.enable_federation_discovery { 1 } else { 0 });
        if let Ok(cache) = self.federation_cache.read() {
            stats.insert("cached_federations".to_string(), cache.len() as u32);
            let total_partners: u32 = cache.values()
                .map(|(partners, _)| partners.len() as u32)
                .sum();
            stats.insert("total_federation_partners".to_string(), total_partners);
        stats
    /// Clear federation cache
    pub fn clear_cache(&self) {
            cache.clear();
} 
