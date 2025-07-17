//! Node registry types and traits for cross-node authentication
//!
//! This module contains all types and traits related to node registration,
//! verification, and the main authentication engine.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use beardog_errors::BearDogResult;

use super::authorization::{AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation};
use super::genetics::{BearDogGenetics, NodeCapability};
use super::spawning::SpawnedBearDog;
use super::workflow::{CrossNodeWorkflowRequest, WorkflowStatus};

/// Node information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier
    pub id: String,
    /// Network address of the node
    pub address: String,
    /// Capabilities available on this node
    pub capabilities: Vec<NodeCapability>,
    /// Trust level for this node (0.0 to 1.0)
    pub trust_level: f64,
    /// Timestamp when node was last seen
    pub last_seen: DateTime<Utc>,
    /// Optional genetics information for the node
    pub genetics: Option<BearDogGenetics>,
}

/// Trait for node registry operations
pub trait NodeRegistry: Send + Sync {
    /// Get node information by node ID
    fn get_node_info(&self, node_id: &str) -> BearDogResult<NodeInfo>;
    /// Register a new node in the registry
    fn register_node(&mut self, node_info: NodeInfo) -> BearDogResult<()>;
    /// Get the trust level for a node
    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64>;
    /// Update the trust level for a node
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()>;
}

/// Trait for proof verification
pub trait ProofVerifier: Send + Sync {
    /// Verify an authorization proof
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> BearDogResult<bool>;
    /// Generate a proof for an authorization and operation
    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof>;
}

/// Trait for workflow engine integration
pub trait WorkflowEngine: Send + Sync {
    /// Submit a workflow request for execution
    fn submit_workflow(&mut self, request: CrossNodeWorkflowRequest) -> BearDogResult<String>;
    /// Get the status of a workflow by ID
    fn get_workflow_status(&self, workflow_id: &str) -> BearDogResult<WorkflowStatus>;
}

/// Main cross-node authorization engine
pub struct CrossNodeAuthEngine {
    /// Configuration for the authorization engine
    pub config: super::authorization::CrossNodeAuthConfig,
    /// Currently active authorizations
    pub active_authorizations: HashMap<String, CrossNodeAuthorization>,
    /// Registry of spawned BearDog instances
    pub spawned_beardogs: HashMap<String, SpawnedBearDog>,
    /// Registry of BearDog genetics
    pub genetics_registry: HashMap<String, BearDogGenetics>,
    /// Node registry for tracking nodes
    pub node_registry: Box<dyn NodeRegistry + Send + Sync>,
    /// Proof verifier for authorization proofs
    pub proof_verifier: Box<dyn ProofVerifier + Send + Sync>,
    /// Optional workflow engine for automated workflows
    pub workflow_engine: Option<Box<dyn WorkflowEngine + Send + Sync>>,
}
