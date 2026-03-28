// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::SecurityContext;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsNode {
    pub id: String,
    /// Collection of genetics data
    pub genetics_data: Vec<u8>,
    /// The last seen value
    pub last_seen: DateTime<Utc>,
    /// The security context value
    pub security_context: SecurityContext,
}

impl GeneticsNode {
    /// Creates a new instance
    pub fn new(id: String, genetics_data: Vec<u8>) -> Self {
        Self {
            id,
            genetics_data,
            last_seen: Utc::now(),
            security_context: SecurityContext::default(),
        }
    }

    /// Updates last_seen
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PGeneticsNetwork {
    /// Mapping of nodes
    pub nodes: HashMap<String, GeneticsNode>,
    pub config: P2PNetworkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PNetworkConfig {
    /// Number of max_nodes
    pub max_nodes: usize,
    pub connection_timeout_ms: u64,
    /// Number of sync_interval_ms
    pub sync_interval_ms: u64,
}

impl Default for P2PNetworkConfig {
    fn default() -> Self {
        Self {
            max_nodes: 100,
            connection_timeout_ms: 5000,
            sync_interval_ms: 10000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkStatus {
    /// Active or enabled state
    Active,
    /// Inactive or disabled state
    Inactive,
    /// Currently syncing
    Syncing,
    /// Error or failure state
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnPriority {
    /// Represents low variant
    Low,
    /// Represents normal variant
    Normal,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedSpawnRequest {
    pub genetics_id: String,
    /// The priority value
    pub priority: SpawnPriority,
    /// Number of required_nodes
    pub required_nodes: usize,
    pub timeout_ms: u64,
}

impl P2PGeneticsNetwork {
    /// Creates a new instance
    pub fn new(config: P2PNetworkConfig) -> Self {
        Self {
            nodes: HashMap::new(),
            config,
        }
    }



    pub fn add_node(&mut self, node: GeneticsNode) -> Result<(), BearDogError> {
        if self.nodes.len() >= self.config.max_nodes {
            return Err(BearDogError::validation("P2P genetics network at capacity"));
        }

        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Removes node
    pub fn remove_node(&mut self, node_id: &str) -> Option<GeneticsNode> {
        self.nodes.remove(node_id)
    }

    /// Gets node
    pub fn get_node(&self, node_id: &str) -> Option<&GeneticsNode> {
        self.nodes.get(node_id)
    }



    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
