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


/// # Node Registry Configuration - Canonical
///
/// **UNIFIED NODE REGISTRY CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Unified Node Registry Configuration
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


/// **CANONICAL** Node Discovery Configuration
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

/// **CANONICAL** P2P Configuration
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

/// **CANONICAL** Federation Configuration
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

/// **CANONICAL** Trust Configuration
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

/// **CANONICAL** Phonebook Configuration
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

/// **CANONICAL** Bootstrap Configuration
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
