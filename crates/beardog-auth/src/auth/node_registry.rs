// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::auth::types::{NodeInfo, NodeRegistry};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use parking_lot::RwLock;

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
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
        }
    }
}

impl NodeRegistry for InMemoryNodeRegistry {
    /// Gets `node_info`
    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError> {
        // parking_lot::RwLock never panics - cleaner API!
        let nodes = self.nodes.read();

        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Node {node_id} not found")))
    }

    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
        // parking_lot::RwLock never panics - cleaner API!
        let mut nodes = self.nodes.write();

        nodes.insert(node_info.node_id.clone(), node_info);
        Ok(())
    }

    /// Gets `trust_level`
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError> {
        let node_info = self.get_node_info(node_id)?;
        Ok(node_info.trust_level)
    }

    /// Updates `trust_level`
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> Result<(), BearDogError> {
        // parking_lot::RwLock never panics - cleaner API!
        let mut nodes = self.nodes.write();

        if let Some(node) = nodes.get_mut(node_id) {
            node.trust_level = trust_level;
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Node {node_id} not found")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::types::genetics::NodeCapability;
    use chrono::Utc;

    fn create_test_node_info(node_id: &str) -> NodeInfo {
        const TEST_PORT: u16 = 8080;
        NodeInfo {
            node_id: node_id.to_string(),
            address: format!("127.0.0.1:{TEST_PORT}"),
            capabilities: vec![NodeCapability::BasicOperations],
            trust_level: 0.8,
            last_seen: Utc::now(),
            genetics: None,
        }
    }

    #[test]
    fn test_new_registry_is_empty() {
        let registry = InMemoryNodeRegistry::new();
        let result = registry.get_node_info("nonexistent");

        assert!(result.is_err(), "New registry should not have any nodes");
    }

    #[test]
    fn test_register_and_retrieve_node() {
        let mut registry = InMemoryNodeRegistry::new();
        let node_info = create_test_node_info("node-1");

        let register_result = registry.register_node(node_info.clone());
        assert!(register_result.is_ok(), "Node registration should succeed");

        let retrieve_result = registry.get_node_info("node-1");
        assert!(retrieve_result.is_ok(), "Node retrieval should succeed");

        let retrieved = retrieve_result.unwrap();
        const TEST_PORT: u16 = 8080;
        assert_eq!(retrieved.node_id, "node-1");
        assert_eq!(retrieved.address, format!("127.0.0.1:{TEST_PORT}"));
    }

    #[test]
    fn test_get_nonexistent_node() {
        let registry = InMemoryNodeRegistry::new();
        let result = registry.get_node_info("nonexistent");

        assert!(result.is_err(), "Retrieving nonexistent node should fail");
    }

    #[test]
    fn test_get_trust_level_success() {
        let mut registry = InMemoryNodeRegistry::new();
        let node_info = create_test_node_info("node-1");
        registry.register_node(node_info).unwrap();

        let result = registry.get_trust_level("node-1");
        assert!(result.is_ok(), "Getting trust level should succeed");
        assert_eq!(
            result.unwrap(),
            0.8,
            "Trust level should match registered value"
        );
    }

    #[test]
    fn test_get_trust_level_nonexistent_node() {
        let registry = InMemoryNodeRegistry::new();
        let result = registry.get_trust_level("nonexistent");

        assert!(
            result.is_err(),
            "Getting trust level for nonexistent node should fail"
        );
    }

    #[test]
    fn test_update_trust_level_success() {
        let mut registry = InMemoryNodeRegistry::new();
        let node_info = create_test_node_info("node-1");
        registry.register_node(node_info).unwrap();

        let update_result = registry.update_trust_level("node-1", 0.95);
        assert!(update_result.is_ok(), "Updating trust level should succeed");

        let new_level = registry.get_trust_level("node-1").unwrap();
        assert_eq!(new_level, 0.95, "Trust level should be updated");
    }

    #[test]
    fn test_update_trust_level_nonexistent_node() {
        let mut registry = InMemoryNodeRegistry::new();
        let result = registry.update_trust_level("nonexistent", 0.9);

        assert!(
            result.is_err(),
            "Updating trust level for nonexistent node should fail"
        );
    }

    #[test]
    fn test_register_overwrites_existing_node() {
        let mut registry = InMemoryNodeRegistry::new();

        let node1 = create_test_node_info("node-1");
        registry.register_node(node1).unwrap();

        let mut node2 = create_test_node_info("node-1");
        node2.trust_level = 0.5;
        registry.register_node(node2).unwrap();

        let retrieved = registry.get_node_info("node-1").unwrap();
        assert_eq!(
            retrieved.trust_level, 0.5,
            "Second registration should overwrite first"
        );
    }

    #[test]
    fn test_multiple_nodes_registration() {
        let mut registry = InMemoryNodeRegistry::new();

        registry
            .register_node(create_test_node_info("node-1"))
            .unwrap();
        registry
            .register_node(create_test_node_info("node-2"))
            .unwrap();
        registry
            .register_node(create_test_node_info("node-3"))
            .unwrap();

        assert!(registry.get_node_info("node-1").is_ok());
        assert!(registry.get_node_info("node-2").is_ok());
        assert!(registry.get_node_info("node-3").is_ok());
    }

    #[test]
    fn test_default_constructor() {
        let registry1 = InMemoryNodeRegistry::new();
        let registry2 = InMemoryNodeRegistry::default();

        // Both should work identically
        assert!(registry1.get_node_info("test").is_err());
        assert!(registry2.get_node_info("test").is_err());
    }
}
