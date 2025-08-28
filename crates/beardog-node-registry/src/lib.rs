use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod registry {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct NodeRegistry {
        nodes: HashMap<String, NodeInfo>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NodeInfo {
        pub node_id: String,
        pub endpoint: String,
        pub capabilities: Vec<String>,
        pub status: NodeStatus,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NodeStatus {
        Active,
        Inactive,
        Maintenance,
    }

    impl NodeRegistry {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
            self.nodes.insert(node_info.node_id.clone(), node_info);
            Ok(())
        }

        pub fn get_node(&self, node_id: &str) -> Option<&NodeInfo> {
            self.nodes.get(node_id)
        }

        pub fn list_active_nodes(&self) -> Vec<&NodeInfo> {
            self.nodes
                .values()
                .filter(|node| matches!(node.status, NodeStatus::Active))
                .collect()
        }
    }
}
