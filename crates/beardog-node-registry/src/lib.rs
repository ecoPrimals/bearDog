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


/// BearDog node-registry module
///
/// This module provides node registry functionality for the BearDog ecosystem.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node registry functionality
pub mod registry {
    use super::*;
    
    /// Node registry service
    #[derive(Debug, Clone, Default)]
    pub struct NodeRegistry {
        nodes: HashMap<String, NodeInfo>,
    }
    
    /// Node information
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NodeInfo {
        pub node_id: String,
        pub endpoint: String,
        pub capabilities: Vec<String>,
        pub status: NodeStatus,
    }
    
    /// Node status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NodeStatus {
        Active,
        Inactive,
        Maintenance,
    }
    
    impl NodeRegistry {
        /// Create a new node registry
        pub fn new() -> Self {
            Self::default()
        }
        
        /// Register a new node
        pub fn register_node(&mut self, node_info: NodeInfo) -> BearDogResult<()> {
            self.nodes.insert(node_info.node_id.clone(), node_info);
            Ok(())
        }
        
        /// Get node information
        pub fn get_node(&self, node_id: &str) -> Option<&NodeInfo> {
            self.nodes.get(node_id)
        }
        
        /// List all active nodes
        pub fn list_active_nodes(&self) -> Vec<&NodeInfo> {
            self.nodes
                .values()
                .filter(|node| matches!(node.status, NodeStatus::Active))
                .collect()
        }
    }
}
