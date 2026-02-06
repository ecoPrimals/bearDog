//! # Bootstrap Types
//!
//! This module provides types for node registry bootstrapping,
//! including discovery, federation, and health checking.

use crate::node_registry::types::{NodeInfo, TrustLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================
// Discovery Types
// ============================================================

/// Result of network discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscoveryResult {
    /// Total nodes discovered
    pub total_nodes: usize,

    /// Region where discovery was performed
    pub region: String,

    /// Timestamp of discovery
    pub timestamp: String,

    /// Discovered nodes
    pub nodes: Vec<DiscoveredNode>,
}

impl Default for NetworkDiscoveryResult {
    fn default() -> Self {
        Self {
            total_nodes: 0,
            region: String::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            nodes: Vec::new(),
        }
    }
}

/// A discovered node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredNode {
    /// Node ID
    pub node_id: String,

    /// Node address
    pub address: String,

    /// Public key
    pub public_key: String,

    /// Trust level
    pub trust_level: TrustLevel,

    /// Node capabilities
    pub capabilities: Vec<String>,

    /// Last seen timestamp
    pub last_seen: String,

    /// Latency in milliseconds
    pub latency_ms: Option<u32>,
}

impl Default for DiscoveredNode {
    fn default() -> Self {
        Self {
            node_id: String::new(),
            address: String::new(),
            public_key: String::new(),
            trust_level: TrustLevel::Unknown,
            capabilities: Vec::new(),
            last_seen: chrono::Utc::now().to_rfc3339(),
            latency_ms: None,
        }
    }
}

// ============================================================
// Bootstrap Configuration
// ============================================================

/// Bootstrap configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfig {
    /// Maximum bootstrap attempts
    pub max_bootstrap_attempts: u32,

    /// Bootstrap timeout in seconds
    pub bootstrap_timeout_seconds: u64,

    /// Phonebook URLs for node discovery
    pub phonebook_urls: Vec<String>,

    /// Minimum number of trusted nodes required
    pub min_trusted_nodes: u32,

    /// Enable federation discovery
    pub enable_federation_discovery: bool,

    /// Custom HTTP headers
    pub custom_headers: HashMap<String, String>,

    /// Prefer regional nodes
    pub prefer_regional_nodes: bool,

    /// Preferred regions
    pub preferred_regions: Vec<String>,
}

impl Default for BootstrapConfig {
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
            preferred_regions: vec!["us-east".to_string(), "eu-west".to_string()],
        }
    }
}

// ============================================================
// Bootstrap Statistics
// ============================================================

/// Bootstrap statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BootstrapStatistics {
    /// Total bootstrap attempts
    pub total_attempts: u32,

    /// Successful bootstraps
    pub successful_bootstraps: u32,

    /// Failed bootstraps
    pub failed_bootstraps: u32,

    /// Average bootstrap time in seconds
    pub average_bootstrap_time_seconds: f64,
}

impl BootstrapStatistics {
    /// Create new statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a bootstrap attempt
    pub fn record_attempt(&mut self, success: bool, duration_seconds: f64) {
        self.total_attempts += 1;
        if success {
            self.successful_bootstraps += 1;
        } else {
            self.failed_bootstraps += 1;
        }

        let total_time =
            self.average_bootstrap_time_seconds * (self.total_attempts - 1) as f64;
        self.average_bootstrap_time_seconds =
            (total_time + duration_seconds) / self.total_attempts as f64;
    }

    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            return 0.0;
        }
        self.successful_bootstraps as f64 / self.total_attempts as f64
    }
}

// ============================================================
// Federation Discovery
// ============================================================

/// Federation discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationDiscoveryResult {
    /// Partner node info
    pub partner_info: NodeInfo,

    /// Shared capabilities
    pub shared_capabilities: Vec<String>,

    /// Discovery timestamp
    pub discovered_at: String,
}

impl Default for FederationDiscoveryResult {
    fn default() -> Self {
        Self {
            partner_info: NodeInfo::default(),
            shared_capabilities: Vec::new(),
            discovered_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// ============================================================
// Health Check
// ============================================================

/// Bootstrap health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapHealthCheck {
    /// Whether bootstrap is healthy
    pub is_healthy: bool,

    /// Active connections
    pub active_connections: u32,

    /// Trusted node count
    pub trusted_node_count: u32,

    /// Last successful bootstrap timestamp
    pub last_successful_bootstrap: Option<String>,

    /// Status message
    pub status: String,

    /// Warnings
    pub warnings: Vec<String>,
}

impl Default for BootstrapHealthCheck {
    fn default() -> Self {
        Self {
            is_healthy: true,
            active_connections: 0,
            trusted_node_count: 0,
            last_successful_bootstrap: None,
            status: "healthy".to_string(),
            warnings: Vec::new(),
        }
    }
}

impl BootstrapHealthCheck {
    /// Create new health check
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    /// Check if there are warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_config_default() {
        let config = BootstrapConfig::default();
        assert_eq!(config.max_bootstrap_attempts, 5);
        assert_eq!(config.bootstrap_timeout_seconds, 30);
        assert!(config.enable_federation_discovery);
    }

    #[test]
    fn test_bootstrap_statistics() {
        let mut stats = BootstrapStatistics::new();
        stats.record_attempt(true, 5.0);
        stats.record_attempt(false, 10.0);
        assert_eq!(stats.total_attempts, 2);
        assert_eq!(stats.successful_bootstraps, 1);
        assert_eq!(stats.success_rate(), 0.5);
    }

    #[test]
    fn test_health_check() {
        let mut health = BootstrapHealthCheck::new();
        assert!(health.is_healthy);
        health.add_warning("Test warning");
        assert!(health.has_warnings());
    }
}
