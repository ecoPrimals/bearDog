

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::node_registry::types::{TrustLevel, NodeInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonebookDiscoveryResponse {

    pub nodes: Vec<DiscoveredNode>,

    pub total_nodes: usize,

    pub region: String,

    pub timestamp: String,
}

pub struct DiscoveredNode {

    pub node_id: String,

    pub address: String,

    pub public_key: String,

    pub trust_level: TrustLevel,

    pub capabilities: Vec<String>,

    pub last_seen: String,

    pub latency_ms: Option<u32>,

pub struct BootstrapConfig {

    pub max_bootstrap_attempts: u32,

    pub bootstrap_timeout_seconds: u64,

    pub phonebook_urls: Vec<String>,

    pub min_trusted_nodes: u32,

    pub enable_federation_discovery: bool,

    pub custom_headers: HashMap<String, String>,

    pub prefer_regional_nodes: bool,

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
            custom_headers: HashMap::with_capacity(16),
            prefer_regional_nodes: true,
            preferred_regions: vec!["us-east".to_string(), "us-west".to_string()],
        }
    }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BootstrapStats {

    pub total_attempts: u32,

    pub successful_bootstraps: u32,

    pub failed_bootstraps: u32,

    pub average_bootstrap_time_seconds: f64,

    pub nodes_discovered: u32,

    pub trusted_connections: u32,

    pub federation_partners_discovered: u32,}

impl BootstrapStats {

    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            (self.successful_bootstraps as f64 / self.total_attempts as f64) * 100.0

    pub fn record_attempt(&mut self, success: bool, duration_seconds: f64) {
        self.total_attempts += 1;
        if success {
            self.successful_bootstraps += 1;
            self.failed_bootstraps += 1;

        let total_time = self.average_bootstrap_time_seconds * (self.total_attempts - 1) as f64;
        self.average_bootstrap_time_seconds = (total_time + duration_seconds) / self.total_attempts as f64;

pub struct FederationDiscoveryResult {

    pub partner_info: NodeInfo,

    pub shared_capabilities: Vec<String>,

    pub discovered_at: String,

pub struct BootstrapHealthCheck {

    pub is_healthy: bool,

    pub active_connections: u32,

    pub trusted_node_count: u32,

    pub last_successful_bootstrap: Option<String>,

    pub status: String,

    pub warnings: Vec<String>,
} 
