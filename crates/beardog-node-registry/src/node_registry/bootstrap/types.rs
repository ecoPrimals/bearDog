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


/// Bootstrap Types for Node Registry
///
/// Type definitions for node bootstrapping, discovery, and configuration.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::node_registry::types::{TrustLevel, NodeInfo};
/// Phonebook discovery response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonebookDiscoveryResponse {
    /// List of discovered nodes
    pub nodes: Vec<DiscoveredNode>,
    /// Total number of nodes available
    pub total_nodes: usize,
    /// Region of the phonebook service
    pub region: String,
    /// Timestamp of the response
    pub timestamp: String,
}
/// Discovered node from phonebook
pub struct DiscoveredNode {
    /// Node identifier
    pub node_id: String,
    /// Network address (IP:port)
    pub address: String,
    /// Public key for verification
    pub public_key: String,
    /// Trust level assigned by phonebook
    pub trust_level: TrustLevel,
    /// Regional location
    /// Capabilities advertised by the node
    pub capabilities: Vec<String>,
    /// Last seen timestamp
    pub last_seen: String,
    /// Response latency in milliseconds
    pub latency_ms: Option<u32>,
/// Configuration for bootstrap process
pub struct BootstrapConfig {
    /// Maximum number of bootstrap attempts
    pub max_bootstrap_attempts: u32,
    /// Timeout for each bootstrap attempt
    pub bootstrap_timeout_seconds: u64,
    /// List of phonebook service URLs
    pub phonebook_urls: Vec<String>,
    /// Minimum number of trusted nodes required
    pub min_trusted_nodes: u32,
    /// Enable automatic discovery of federation partners
    pub enable_federation_discovery: bool,
    /// Custom headers for phonebook requests
    pub custom_headers: HashMap<String, String>,
    /// Enable regional preference for node selection
    pub prefer_regional_nodes: bool,
    /// Preferred regions in priority order
    pub preferred_regions: Vec<String>,}


impl Default for BootstrapConfig {}


    fn default() -> Self {
        Self {
            max_bootstrap_attempts: 5,
            bootstrap_timeout_seconds: 30,
            phonebook_urls: vec![
                "https://phonebook.beardog.network".to_string(),
                "https://backup-phonebook.beardog.network".to_string(),
            ],
            min_trusted_nodes: 3,
            enable_federation_discovery: true,
            custom_headers: HashMap::new(),
            prefer_regional_nodes: true,
            preferred_regions: vec!["us-east".to_string(), "us-west".to_string()],
        }
    }
/// Bootstrap statistics and metrics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BootstrapStats {
    /// Total bootstrap attempts made
    pub total_attempts: u32,
    /// Number of successful bootstraps
    pub successful_bootstraps: u32,
    /// Number of failed bootstraps
    pub failed_bootstraps: u32,
    /// Average bootstrap time in seconds
    pub average_bootstrap_time_seconds: f64,
    /// Number of nodes discovered
    pub nodes_discovered: u32,
    /// Number of trusted connections established
    pub trusted_connections: u32,
    /// Federation partners discovered
    pub federation_partners_discovered: u32,}


impl BootstrapStats {
    /// Calculate success rate as percentage}


    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.successful_bootstraps as f64 / self.total_attempts as f64) * 100.0
    /// Update statistics after a bootstrap attempt
    pub fn record_attempt(&mut self, success: bool, duration_seconds: f64) {
        self.total_attempts += 1;
        if success {
            self.successful_bootstraps += 1;
            self.failed_bootstraps += 1;
        
        // Update average bootstrap time
        let total_time = self.average_bootstrap_time_seconds * (self.total_attempts - 1) as f64;
        self.average_bootstrap_time_seconds = (total_time + duration_seconds) / self.total_attempts as f64;
/// Federation discovery result
pub struct FederationDiscoveryResult {
    /// Federation partner information
    pub partner_info: NodeInfo,
    /// Trust level established with partner
    /// Shared capabilities or services
    pub shared_capabilities: Vec<String>,
    /// Discovery timestamp
    pub discovered_at: String,
/// Bootstrap health check result
pub struct BootstrapHealthCheck {
    /// Whether bootstrap process is healthy
    pub is_healthy: bool,
    /// Number of active connections
    pub active_connections: u32,
    /// Number of trusted nodes
    pub trusted_node_count: u32,
    /// Last successful bootstrap timestamp
    pub last_successful_bootstrap: Option<String>,
    /// Current bootstrap status
    pub status: String,
    /// Any health warnings or errors
    pub warnings: Vec<String>,
} 
