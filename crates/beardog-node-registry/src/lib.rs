// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! # `BearDog` Node Registry
//!
//! This crate provides decentralized node registry functionality for the `BearDog` ecosystem.
//! It enables service discovery, node coordination, and status tracking across distributed
//! `BearDog` deployments.
//!
//! ## Features
//!
//! - Node registration and discovery
//! - Status tracking (Active, Inactive, Maintenance)
//! - Capability-based filtering
//!
//! ## Example
//!
//! ```rust
//! use beardog_node_registry::registry::{NodeRegistry, NodeInfo, NodeStatus};
//!
//! let mut registry = NodeRegistry::new();
//! let node = NodeInfo {
//!     node_id: "node-1".to_string(),
//!     endpoint: "http://localhost:8080".to_string(),
//!     capabilities: vec!["compute".to_string()],
//!     status: NodeStatus::Active,
//! };
//! registry.register_node(node).unwrap();
//! ```

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Node registry module providing core registry functionality
pub mod registry {
    use super::{BearDogError, Deserialize, HashMap, Serialize};

    /// A registry for tracking nodes in the `BearDog` ecosystem
    ///
    /// Provides methods for registering, querying, and filtering nodes
    /// based on their status and capabilities.
    #[derive(Debug, Clone, Default)]
    pub struct NodeRegistry {
        nodes: HashMap<String, NodeInfo>,
    }

    /// Information about a node in the registry
    #[derive(Debug, Clone)]
    pub struct NodeInfo {
        /// Unique identifier for the node
        pub node_id: String,
        /// The endpoint URL for the node
        pub endpoint: String,
        /// Collection of capabilities this node provides
        pub capabilities: Vec<String>,
        /// Current operational status of the node
        pub status: NodeStatus,
    }

    /// Operational status of a node
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NodeStatus {
        /// Node is active and available
        Active,
        /// Node is inactive or disabled
        Inactive,
        /// Node is under maintenance
        Maintenance,
    }

    impl NodeRegistry {
        /// Creates a new empty node registry
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        /// Registers a node in the registry
        ///
        /// # Errors
        /// Returns an error if registration fails
        pub fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
            self.nodes.insert(node_info.node_id.clone(), node_info);
            Ok(())
        }

        /// Gets a node by its ID
        #[must_use]
        pub fn get_node(&self, node_id: &str) -> Option<&NodeInfo> {
            self.nodes.get(node_id)
        }

        /// Lists all active nodes in the registry
        #[must_use]
        pub fn list_active_nodes(&self) -> Vec<&NodeInfo> {
            self.nodes
                .values()
                .filter(|node| matches!(node.status, NodeStatus::Active))
                .collect()
        }
    }

    #[cfg(test)]
    #[allow(clippy::unwrap_used, clippy::expect_used)]
    mod tests {
        use super::*;

        #[test]
        fn test_node_registry_new() {
            let registry = NodeRegistry::new();
            assert!(registry.get_node("nonexistent").is_none());
        }

        #[test]
        fn test_node_registry_default() {
            let registry = NodeRegistry::default();
            assert!(registry.list_active_nodes().is_empty());
        }

        #[test]
        fn test_node_registry_register_node() {
            let mut registry = NodeRegistry::new();
            let node = NodeInfo {
                node_id: "node-1".to_string(),
                endpoint: "http://localhost:8080".to_string(),
                capabilities: vec!["compute".to_string(), "storage".to_string()],
                status: NodeStatus::Active,
            };
            let result = registry.register_node(node);
            assert!(result.is_ok());
        }

        #[test]
        fn test_node_registry_get_node() {
            let mut registry = NodeRegistry::new();
            let node = NodeInfo {
                node_id: "node-2".to_string(),
                endpoint: "http://localhost:9090".to_string(),
                capabilities: vec!["security".to_string()],
                status: NodeStatus::Active,
            };
            registry.register_node(node).expect("register");

            let retrieved = registry.get_node("node-2");
            assert!(retrieved.is_some());
            let retrieved = retrieved.expect("node");
            assert_eq!(retrieved.node_id, "node-2");
            assert_eq!(retrieved.endpoint, "http://localhost:9090");
        }

        #[test]
        fn test_node_registry_get_nonexistent() {
            let registry = NodeRegistry::new();
            assert!(registry.get_node("does-not-exist").is_none());
        }

        #[test]
        fn test_node_registry_list_active_nodes() {
            let mut registry = NodeRegistry::new();

            let active_node = NodeInfo {
                node_id: "active-node".to_string(),
                endpoint: "http://active:8080".to_string(),
                capabilities: vec![],
                status: NodeStatus::Active,
            };
            let inactive_node = NodeInfo {
                node_id: "inactive-node".to_string(),
                endpoint: "http://inactive:8080".to_string(),
                capabilities: vec![],
                status: NodeStatus::Inactive,
            };
            let maintenance_node = NodeInfo {
                node_id: "maintenance-node".to_string(),
                endpoint: "http://maintenance:8080".to_string(),
                capabilities: vec![],
                status: NodeStatus::Maintenance,
            };

            registry.register_node(active_node).expect("register");
            registry.register_node(inactive_node).expect("register");
            registry.register_node(maintenance_node).expect("register");

            let active_nodes = registry.list_active_nodes();
            assert_eq!(active_nodes.len(), 1);
            assert_eq!(active_nodes[0].node_id, "active-node");
        }

        #[test]
        fn test_node_info_clone() {
            let node = NodeInfo {
                node_id: "clone-test".to_string(),
                endpoint: "http://clone:8080".to_string(),
                capabilities: vec!["cap1".to_string(), "cap2".to_string()],
                status: NodeStatus::Active,
            };
            let cloned = node.clone();
            assert_eq!(node.node_id, cloned.node_id);
            assert_eq!(node.capabilities, cloned.capabilities);
        }

        #[test]
        fn test_node_status_serialization() {
            let status = NodeStatus::Active;
            let serialized = serde_json::to_string(&status).expect("serialize");
            let deserialized: NodeStatus = serde_json::from_str(&serialized).expect("deserialize");
            assert!(matches!(deserialized, NodeStatus::Active));
        }

        #[test]
        fn test_node_status_variants() {
            let active = NodeStatus::Active;
            let inactive = NodeStatus::Inactive;
            let maintenance = NodeStatus::Maintenance;

            assert!(matches!(active, NodeStatus::Active));
            assert!(matches!(inactive, NodeStatus::Inactive));
            assert!(matches!(maintenance, NodeStatus::Maintenance));
        }

        #[test]
        fn test_node_registry_overwrite() {
            let mut registry = NodeRegistry::new();

            let node1 = NodeInfo {
                node_id: "node-x".to_string(),
                endpoint: "http://old:8080".to_string(),
                capabilities: vec![],
                status: NodeStatus::Active,
            };
            registry.register_node(node1).expect("register");

            let node2 = NodeInfo {
                node_id: "node-x".to_string(),
                endpoint: "http://new:9090".to_string(),
                capabilities: vec!["updated".to_string()],
                status: NodeStatus::Inactive,
            };
            registry.register_node(node2).expect("register");

            let retrieved = registry.get_node("node-x").expect("node");
            assert_eq!(retrieved.endpoint, "http://new:9090");
            assert!(matches!(retrieved.status, NodeStatus::Inactive));
        }
    }
}
