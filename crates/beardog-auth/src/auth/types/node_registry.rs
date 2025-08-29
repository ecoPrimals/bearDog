use super::authorization::{AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation};
use super::genetics::{BearDogGenetics, NodeCapability};
use super::spawning::SpawnedBearDog;
use super::workflow::{CrossNodeWorkflowRequest, WorkflowStatus};
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,

    pub address: String,

    pub capabilities: Vec<NodeCapability>,

    pub trust_level: f64,

    pub last_seen: DateTime<Utc>,

    pub genetics: Option<BearDogGenetics>,
}

pub trait NodeRegistry: Send + Sync {
    /// Get node information by ID
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if node is not found or registry is unavailable
    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError>;

    /// Register a new node in the registry
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if registration fails or node already exists
    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError>;

    /// Get the trust level for a specific node
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if node is not found or trust calculation fails
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError>;

    /// Update the trust level for a specific node
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if node is not found or trust update fails
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> Result<(), BearDogError>;
}

pub trait ProofVerifier: Send + Sync {
    /// Verify an authorization proof
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if proof verification fails or proof is invalid
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> Result<bool, BearDogError>;

    /// Generate an authorization proof
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if proof generation fails or parameters are invalid
    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError>;
}

pub trait WorkflowEngine: Send + Sync {
    /// Submit a cross-node workflow request
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if workflow submission fails or request is invalid
    fn submit_workflow(
        &mut self,
        request: CrossNodeWorkflowRequest,
    ) -> Result<String, BearDogError>;

    /// Get the status of a workflow by ID
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if workflow is not found or status query fails
    fn get_workflow_status(&self, workflow_id: &str) -> Result<WorkflowStatus, BearDogError>;
}

pub struct CrossNodeAuthEngine {
    pub config: super::authorization::CrossNodeAuthConfig,

    pub active_authorizations: HashMap<String, CrossNodeAuthorization>,

    pub spawned_beardogs: HashMap<String, SpawnedBearDog>,

    pub genetics_registry: HashMap<String, BearDogGenetics>,

    pub node_registry: Box<dyn NodeRegistry + Send + Sync>,

    pub proof_verifier: Box<dyn ProofVerifier + Send + Sync>,

    pub workflow_engine: Option<Box<dyn WorkflowEngine + Send + Sync>>,
}
