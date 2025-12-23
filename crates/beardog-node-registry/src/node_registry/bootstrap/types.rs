

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::node_registry::types::{TrustLevel, NodeInfo};

#[derive(Debug, Clone)]
    /// Number of total_nodes
    pub total_nodes: usize,

    /// The region value
    pub region: String,


    pub timestamp: String,
}

pub struct DiscoveredNode {


    pub node_id: String,

    /// The address value
    pub address: String,

    /// The public key value
    pub public_key: String,

    /// The trust level value
    pub trust_level: TrustLevel,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// The last seen value
    pub last_seen: String,

    /// Optional latency ms
    pub latency_ms: Option<u32>,

pub struct BootstrapConfig {

    /// Number of max_bootstrap_attempts
    pub max_bootstrap_attempts: u32,


    pub bootstrap_timeout_seconds: u64,

    /// Collection of phonebook urls
    pub phonebook_urls: Vec<String>,

    /// Number of min_trusted_nodes
    pub min_trusted_nodes: u32,

    /// Whether enable_federation_discovery is enabled
    pub enable_federation_discovery: bool,

    /// Mapping of custom headers
    pub custom_headers: HashMap<String, String>,

    /// Whether prefer_regional_nodes is enabled
    pub prefer_regional_nodes: bool,

    /// Collection of preferred regions
    pub preferred_regions: Vec<String>,}

impl Default for BootstrapConfig {}

    fn default(5,
            bootstrap_timeout_seconds: 30,
            phonebook_urls: vec![
                "https://phonebook.beardog.network".to_string(),
                "https://backup-phonebook.beardog.network".to_string(),
            enable_federation_discovery: true,
            custom_headers: HashMap::with_capacity(true,
            preferred_regions: vec!["us-east".to_string(), duration_seconds: f64) {
        self.total_attempts += 1;
        if success {
            self.successful_bootstraps += 1;
            self.failed_bootstraps += 1;

        let total_time = self.average_bootstrap_time_seconds * (self.total_attempts - 1) as f64;
        self.average_bootstrap_time_seconds = (total_time + duration_seconds) / self.total_attempts as f64;

pub struct FederationDiscoveryResult {

    /// The partner info value
    pub partner_info: NodeInfo,

    /// Collection of shared capabilities
    pub shared_capabilities: Vec<String>,

    /// The discovered at value
    pub discovered_at: String,

pub struct BootstrapHealthCheck {

    /// Whether is_healthy is enabled
    pub is_healthy: bool,

    /// Number of active_connections
    pub active_connections: u32,

    /// Number of trusted_node
    pub trusted_node_count: u32,

    /// Optional last successful bootstrap
    pub last_successful_bootstrap: Option<String>,

    /// Current status of the component
    pub status: String,

    /// Collection of warnings
    pub warnings: Vec<String>,
} 
