

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;
use super::authorization::{AuthorizationProof, CrossNodeAuthorization, CrossNodeOperation};
use super::genetics::{BearDogGenetics, NodeCapability};
use super::spawning::SpawnedBearDog;
use super::workflow::{CrossNodeWorkflowRequest, WorkflowStatus};

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

    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError>;

    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError>;

    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError>;

    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> Result<(), BearDogError>;
}

pub trait ProofVerifier: Send + Sync {
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> Result<bool, BearDogError>;

    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> Result<AuthorizationProof, BearDogError>;
}

pub trait WorkflowEngine: Send + Sync {
    fn submit_workflow(&mut self, request: CrossNodeWorkflowRequest) -> Result<String, BearDogError>;

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
