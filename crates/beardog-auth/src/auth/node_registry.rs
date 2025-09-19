// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::auth::types::{NodeInfo, NodeRegistry};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemoryNodeRegistry {
    nodes: RwLock<HashMap<String, NodeInfo>>,
}

impl Default for InMemoryNodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryNodeRegistry {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
        }
    }
}

impl NodeRegistry for InMemoryNodeRegistry {
    /// Gets node_info
    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError> {
        let nodes = self
            .nodes
            .read()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire read lock: {}", e)))?;

        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Node {} not found", node_id)))
    }

    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
        let mut nodes = self
            .nodes
            .write()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire write lock: {}", e)))?;

        nodes.insert(node_info.node_id.clone(), node_info);
        Ok(())
    }

    /// Gets trust_level
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError> {
        let node_info = self.get_node_info(node_id)?;
        Ok(node_info.trust_level)
    }

    /// Updates trust_level
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> Result<(), BearDogError> {
        let mut nodes = self
            .nodes
            .write()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire write lock: {}", e)))?;

        if let Some(node) = nodes.get_mut(node_id) {
            node.trust_level = trust_level;
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Node {} not found",
                node_id
            )))
        }
    }
}
