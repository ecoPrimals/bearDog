

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedNodeRegistryConfig {
    pub discovery: NodeDiscoveryConfig,
    pub p2p: P2PConfig,
    pub federation: FederationConfig,
    pub trust: TrustConfig,
    pub phonebook: PhonebookConfig,
    pub bootstrap: BootstrapConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDiscoveryConfig {
    pub enabled: bool,
    pub discovery_interval: Duration,
    pub discovery_timeout: Duration,
    pub max_nodes: u32,
}

impl Default for NodeDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_interval: Duration::from_secs(60),
            discovery_timeout: Duration::from_secs(10),
            max_nodes: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConfig {
    pub enabled: bool,
    pub listen_port: u16,
    pub max_peers: u32,
    pub peer_timeout: Duration,
    pub heartbeat_interval: Duration,
}

impl Default for P2PConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            listen_port: 8081,
            max_peers: 50,
            peer_timeout: Duration::from_secs(300),
            heartbeat_interval: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    pub enabled: bool,
    pub federation_id: String,
    pub sync_interval: Duration,
    pub trust_threshold: f64,
}

impl Default for FederationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            federation_id: "default".to_string(),
            sync_interval: Duration::from_secs(300),
            trust_threshold: 0.7,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustConfig {
    pub enabled: bool,
    pub initial_trust_level: f64,
    pub trust_decay_rate: f64,
    pub trust_update_interval: Duration,
}

impl Default for TrustConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            initial_trust_level: 0.5,
            trust_decay_rate: 0.01,
            trust_update_interval: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonebookConfig {
    pub enabled: bool,
    pub max_entries: u32,
    pub entry_ttl: Duration,
    pub cleanup_interval: Duration,
}

impl Default for PhonebookConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: 1000,
            entry_ttl: Duration::from_secs(3600),
            cleanup_interval: Duration::from_secs(300),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfig {
    pub enabled: bool,
    pub bootstrap_nodes: Vec<String>,
    pub bootstrap_timeout: Duration,
    pub retry_attempts: u32,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bootstrap_nodes: Vec::new(),
            bootstrap_timeout: Duration::from_secs(30),
            retry_attempts: 3,
        }
    }
}
