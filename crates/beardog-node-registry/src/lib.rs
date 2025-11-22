// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod registry {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct NodeRegistry {
        nodes: HashMap<String, NodeInfo>,
    }

    #[derive(Debug, Clone)]
    pub struct NodeInfo {
        pub node_id: String,
        /// The endpoint value
        pub endpoint: String,
        /// Collection of capabilities
        pub capabilities: Vec<String>,
        /// Current status of the component
        pub status: NodeStatus,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NodeStatus {
        /// Active or enabled state
        Active,
        /// Inactive or disabled state
        Inactive,
        /// Represents maintenance variant
        Maintenance,
    }

    impl NodeRegistry {
        /// New operation.
        /// Creates a new instance
        pub fn new() -> Self {
            Self::default()
        }

        /// Register Node operation.
        ///
        /// # Errors
        /// Returns an error if the operation fails.
        pub fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
            self.nodes.insert(node_info.node_id.clone(), node_info);
            Ok(())
        }

        /// Get Node operation.
        /// Gets node
        pub fn get_node(&self, node_id: &str) -> Option<&NodeInfo> {
            self.nodes.get(node_id)
        }

        /// List Active Nodes operation.
        pub fn list_active_nodes(&self) -> Vec<&NodeInfo> {
            self.nodes
                .values()
                .filter(|node| matches!(node.status, NodeStatus::Active))
                .collect()
        }
    }
}
