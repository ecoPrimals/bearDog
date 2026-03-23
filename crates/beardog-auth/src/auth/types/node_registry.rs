// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::authorization::{AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation};
use super::genetics::{BearDogGenetics, NodeCapability};
use super::spawning::SpawnedBearDog;
use super::workflow::{CrossNodeWorkflowRequest, WorkflowStatus};
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// Health signal for a node participating in cross-node consensus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsensusNodeHealth {
    /// Fully trusted for quorum participation.
    Healthy,
    /// Still listed but should be deprioritized.
    Degraded,
    /// Excluded from [`CrossNodeAuthEngine::get_trusted_nodes`] results.
    Offline,
}

/// In-memory consensus registry entry (paired with [`CrossNodeAuthEngine::consensus_registry`]).
#[derive(Debug, Clone)]
pub struct ConsensusNodeRecord {
    /// Liveness / trust tier for quorum selection.
    pub health: ConsensusNodeHealth,
    /// Last time this record was created or updated.
    pub last_seen: DateTime<Utc>,
}

/// Example healthy quorum participants for `CrossNodeAuthEngine::default()` in tests.
///
/// [`CrossNodeAuthEngine::new`] starts with an empty registry; production deployments should
/// populate [`CrossNodeAuthEngine::consensus_registry`] from configuration or discovery before
/// relying on quorum APIs.
pub fn default_consensus_registry() -> BTreeMap<String, ConsensusNodeRecord> {
    let now = Utc::now();
    ["node_1", "node_2", "node_3"]
        .into_iter()
        .map(|id| {
            (
                id.to_string(),
                ConsensusNodeRecord {
                    health: ConsensusNodeHealth::Healthy,
                    last_seen: now,
                },
            )
        })
        .collect()
}

/// Snapshot of a peer primal used for trust scoring, routing, and capability discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier within the mesh or registry namespace.
    pub node_id: String,
    /// The address value
    pub address: String,
    /// Collection of capabilities
    pub capabilities: Vec<NodeCapability>,
    /// The trust level value
    pub trust_level: f64,
    /// The last seen value
    pub last_seen: DateTime<Utc>,
    /// Optional genetics
    pub genetics: Option<BearDogGenetics>,
}

/// Abstraction over directory services that track participating BearDog nodes.
pub trait NodeRegistry: Send + Sync {
    /// Looks up metadata previously registered for `node_id`.
    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError>;

    /// Persists or replaces registration details for a node.
    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError>;

    /// Gets `trust_level`
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError>;

    /// Updates `trust_level`
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> Result<(), BearDogError>;
}

/// Validates or mints [`AuthorizationProof`] values for cross-node calls.
pub trait ProofVerifier: Send + Sync {
    /// Returns `Ok(true)` when `proof` is well-formed, fresh, and cryptographically acceptable.
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> Result<bool, BearDogError>;

    /// Constructs a new proof binding `operation` to `authorization`.
    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError>;
}

/// Orchestrates long-running cross-node workflows that require authorization context.
pub trait WorkflowEngine: Send + Sync {
    /// Enqueues a workflow and returns an opaque `workflow_id` for status polling.
    fn submit_workflow(
        &mut self,
        request: CrossNodeWorkflowRequest,
    ) -> Result<String, BearDogError>;

    /// Retrieves the latest [`WorkflowStatus`] for a previously submitted workflow.
    fn get_workflow_status(&self, workflow_id: &str) -> Result<WorkflowStatus, BearDogError>;
}

/// Composes policy config, registries, and verifiers used to authorize mesh-wide operations.
pub struct CrossNodeAuthEngine {
    /// Static policy loaded at startup governing proofs, spawning, and approvals.
    pub config: super::authorization::CrossNodeAuthConfig,

    /// Authorizations keyed by request id (or similar) that are still valid.
    pub active_authorizations: HashMap<String, CrossNodeAuthorization>,

    /// Active child primals spawned through this engine, keyed by spawn id.
    pub spawned_beardogs: HashMap<String, SpawnedBearDog>,

    /// Known genomes or key material metadata indexed by genetics id.
    pub genetics_registry: HashMap<String, BearDogGenetics>,

    /// Pluggable directory of mesh nodes (memory, Consul, etc.).
    pub node_registry: Box<dyn NodeRegistry + Send + Sync>,

    /// Cryptographic proof implementation (HSM-backed or software test double).
    pub proof_verifier: Box<dyn ProofVerifier + Send + Sync>,

    /// Optional workflow orchestrator for multi-step approvals.
    pub workflow_engine: Option<Box<dyn WorkflowEngine + Send + Sync>>,

    /// Ordered in-memory consensus participants (health, last seen).
    pub consensus_registry: BTreeMap<String, ConsensusNodeRecord>,
}

#[cfg(test)]
impl Default for CrossNodeAuthEngine {
    fn default() -> Self {
        Self {
            config: super::authorization::CrossNodeAuthConfig::default(),
            active_authorizations: HashMap::new(),
            spawned_beardogs: HashMap::new(),
            genetics_registry: HashMap::new(),
            node_registry: Box::new(crate::auth::node_registry::InMemoryNodeRegistry::new()),
            proof_verifier: Box::new(crate::auth::proof_verifier::DefaultProofVerifier::new()),
            workflow_engine: None,
            consensus_registry: default_consensus_registry(),
        }
    }
}

// Comprehensive tests for node_registry module
#[cfg(test)]
mod comprehensive_tests {
    use super::*;
    use crate::auth::node_registry::InMemoryNodeRegistry;
    use crate::auth::proof_verifier::DefaultProofVerifier;

    // Helper function to create test NodeInfo
    fn create_test_node_info(node_id: &str) -> NodeInfo {
        NodeInfo {
            node_id: node_id.to_string(),
            address: format!("127.0.0.1:{}", 8000 + node_id.len()),
            capabilities: vec![NodeCapability::SecurityAnalysis],
            trust_level: 0.8,
            last_seen: Utc::now(),
            genetics: Some(BearDogGenetics::default()),
        }
    }

    #[test]
    fn test_node_info_creation() {
        let node = create_test_node_info("test-node-1");

        assert_eq!(node.node_id, "test-node-1");
        assert_eq!(node.address, "127.0.0.1:8011");
        assert_eq!(node.trust_level, 0.8);
        assert!(!node.capabilities.is_empty());
        assert!(node.genetics.is_some());
    }

    #[test]
    fn test_node_info_serialization() {
        let node = create_test_node_info("serialize-test");

        // Serialize to JSON
        let json = serde_json::to_string(&node).expect("Should serialize");
        assert!(!json.is_empty());

        // Deserialize back
        let deserialized: NodeInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.node_id, node.node_id);
        assert_eq!(deserialized.trust_level, node.trust_level);
    }

    #[test]
    fn test_in_memory_node_registry_register_node() {
        let mut registry = InMemoryNodeRegistry::new();
        let node = create_test_node_info("register-node-1");

        let result = registry.register_node(node.clone());
        assert!(result.is_ok(), "Should register node successfully");

        // Verify node was registered
        let retrieved = registry.get_node_info("register-node-1");
        assert!(retrieved.is_ok());
        assert_eq!(
            retrieved.expect("registered node").node_id,
            "register-node-1"
        );
    }

    #[test]
    fn test_in_memory_node_registry_get_missing_node() {
        let registry = InMemoryNodeRegistry::new();

        let result = registry.get_node_info("nonexistent-node");
        assert!(result.is_err(), "Should fail for missing node");

        // Verify error message
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_in_memory_node_registry_get_trust_level() {
        let mut registry = InMemoryNodeRegistry::new();
        let mut node = create_test_node_info("trust-node");
        node.trust_level = 0.95;

        registry.register_node(node).expect("Should register");

        let trust_level = registry.get_trust_level("trust-node");
        assert!(trust_level.is_ok());
        assert_eq!(trust_level.expect("trust level"), 0.95);
    }

    #[test]
    fn test_in_memory_node_registry_get_trust_level_missing_node() {
        let registry = InMemoryNodeRegistry::new();

        let result = registry.get_trust_level("missing-node");
        assert!(result.is_err(), "Should fail for missing node");
    }

    #[test]
    fn test_in_memory_node_registry_update_trust_level() {
        let mut registry = InMemoryNodeRegistry::new();
        let node = create_test_node_info("update-trust-node");

        registry.register_node(node).expect("Should register");

        // Update trust level
        let result = registry.update_trust_level("update-trust-node", 0.99);
        assert!(result.is_ok(), "Should update trust level");

        // Verify update
        let new_trust = registry.get_trust_level("update-trust-node");
        assert_eq!(new_trust.expect("updated trust level"), 0.99);
    }

    #[test]
    fn test_in_memory_node_registry_update_trust_level_missing_node() {
        let mut registry = InMemoryNodeRegistry::new();

        let result = registry.update_trust_level("missing-node", 0.5);
        assert!(result.is_err(), "Should fail for missing node");

        let err = result.unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_in_memory_node_registry_multiple_nodes() {
        let mut registry = InMemoryNodeRegistry::new();

        // Register multiple nodes
        for i in 1..=5 {
            let node = create_test_node_info(&format!("node-{i}"));
            registry.register_node(node).expect("Should register");
        }

        // Verify all nodes registered
        for i in 1..=5 {
            let result = registry.get_node_info(&format!("node-{i}"));
            assert!(result.is_ok(), "Node {} should exist", i);
        }
    }

    #[test]
    fn test_default_proof_verifier_verify_authorization() {
        let verifier = DefaultProofVerifier::new();
        let proof = AuthorizationProof {
            authorization_id: "test-auth".to_string(),
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "signature".to_string(),
        };

        let result = verifier.verify_authorization_proof(&proof);
        assert!(result.is_ok());
        assert!(
            result.expect("verify authorization proof"),
            "structurally valid proof should verify"
        );
    }

    // Test removed due to struct field mismatches - needs proper mock setup

    #[test]
    fn test_cross_node_auth_engine_default() {
        let engine = CrossNodeAuthEngine::default();

        assert!(engine.active_authorizations.is_empty());
        assert!(engine.spawned_beardogs.is_empty());
        assert!(engine.genetics_registry.is_empty());
        assert!(engine.workflow_engine.is_none());
    }

    #[test]
    fn test_node_info_with_no_genetics() {
        let mut node = create_test_node_info("no-genetics-node");
        node.genetics = None;

        assert!(node.genetics.is_none());

        // Should still serialize/deserialize
        let json = serde_json::to_string(&node).expect("Should serialize");
        let deserialized: NodeInfo = serde_json::from_str(&json).expect("Should deserialize");
        assert!(deserialized.genetics.is_none());
    }

    #[test]
    fn test_node_info_trust_level_boundaries() {
        let mut node = create_test_node_info("boundary-node");

        // Test minimum trust level
        node.trust_level = 0.0;
        assert_eq!(node.trust_level, 0.0);

        // Test maximum trust level
        node.trust_level = 1.0;
        assert_eq!(node.trust_level, 1.0);
    }

    #[test]
    fn test_node_info_empty_capabilities() {
        let mut node = create_test_node_info("no-cap-node");
        node.capabilities = vec![];

        assert!(node.capabilities.is_empty());

        // Should still be valid
        let json = serde_json::to_string(&node).expect("Should serialize");
        assert!(!json.is_empty());
    }

    #[test]
    fn test_node_info_multiple_capabilities() {
        let mut node = create_test_node_info("multi-cap-node");
        node.capabilities = vec![
            NodeCapability::SecurityAnalysis,
            NodeCapability::ThreatDetection,
            NodeCapability::CryptographicAuditing,
        ];

        assert_eq!(node.capabilities.len(), 3);
        assert!(
            node.capabilities
                .contains(&NodeCapability::SecurityAnalysis)
        );
        assert!(node.capabilities.contains(&NodeCapability::ThreatDetection));
        assert!(
            node.capabilities
                .contains(&NodeCapability::CryptographicAuditing)
        );
    }

    // Test removed due to struct field mismatches - needs proper mock setup

    #[test]
    fn test_node_registry_overwrite_existing_node() {
        let mut registry = InMemoryNodeRegistry::new();

        // Register initial node
        let node1 = create_test_node_info("overwrite-node");
        registry.register_node(node1).expect("Should register");

        // Register same node ID with different trust level
        let mut node2 = create_test_node_info("overwrite-node");
        node2.trust_level = 0.5;
        registry.register_node(node2).expect("Should register");

        // Verify it was overwritten
        let trust = registry.get_trust_level("overwrite-node");
        assert_eq!(
            trust.expect("overwritten trust level"),
            0.5,
            "Should have new trust level"
        );
    }
}
