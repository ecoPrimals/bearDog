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
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
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

pub trait NodeRegistry: Send + Sync {
    /// Gets `node_info`
    fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError>;

    fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError>;

    /// Gets `trust_level`
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError>;

    /// Updates `trust_level`
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
    fn submit_workflow(
        &mut self,
        request: CrossNodeWorkflowRequest,
    ) -> Result<String, BearDogError>;

    /// Gets `workflow_status`
    fn get_workflow_status(&self, workflow_id: &str) -> Result<WorkflowStatus, BearDogError>;
}

pub struct CrossNodeAuthEngine {
    pub config: super::authorization::CrossNodeAuthConfig,

    /// Mapping of active authorizations
    pub active_authorizations: HashMap<String, CrossNodeAuthorization>,

    /// Mapping of spawned beardogs
    pub spawned_beardogs: HashMap<String, SpawnedBearDog>,

    /// Mapping of genetics registry
    pub genetics_registry: HashMap<String, BearDogGenetics>,

    /// The node registry value
    pub node_registry: Box<dyn NodeRegistry + Send + Sync>,

    /// The proof verifier value
    pub proof_verifier: Box<dyn ProofVerifier + Send + Sync>,

    /// Optional workflow engine
    pub workflow_engine: Option<Box<dyn WorkflowEngine + Send + Sync>>,
}

#[cfg(test)]
impl Default for CrossNodeAuthEngine {
    fn default() -> Self {
        Self {
            config: super::authorization::CrossNodeAuthConfig::default(),
            active_authorizations: HashMap::new(),
            spawned_beardogs: HashMap::new(),
            genetics_registry: HashMap::new(),
            node_registry: Box::new(test_helpers::MockNodeRegistry::default()),
            proof_verifier: Box::new(test_helpers::MockProofVerifier),
            workflow_engine: None,
        }
    }
}

// Test helper implementations
#[cfg(test)]
mod test_helpers {
    use super::*;

    #[derive(Default)]
    pub struct MockNodeRegistry {
        nodes: HashMap<String, NodeInfo>,
    }

    impl NodeRegistry for MockNodeRegistry {
        fn get_node_info(&self, node_id: &str) -> Result<NodeInfo, BearDogError> {
            self.nodes
                .get(node_id)
                .cloned()
                .ok_or_else(|| BearDogError::not_found(format!("Node not found: {}", node_id)))
        }

        fn register_node(&mut self, node_info: NodeInfo) -> Result<(), BearDogError> {
            self.nodes.insert(node_info.node_id.clone(), node_info);
            Ok(())
        }

        fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError> {
            self.get_node_info(node_id).map(|n| n.trust_level)
        }

        fn update_trust_level(
            &mut self,
            node_id: &str,
            trust_level: f64,
        ) -> Result<(), BearDogError> {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.trust_level = trust_level;
                Ok(())
            } else {
                Err(BearDogError::not_found(format!(
                    "Node not found: {}",
                    node_id
                )))
            }
        }
    }

    pub struct MockProofVerifier;

    impl ProofVerifier for MockProofVerifier {
        fn verify_authorization_proof(
            &self,
            _proof: &AuthorizationProof,
        ) -> Result<bool, BearDogError> {
            Ok(true)
        }

        fn generate_proof(
            &self,
            _authorization: &CrossNodeAuthorization,
            _operation: &CrossNodeOperation,
        ) -> Result<AuthorizationProof, BearDogError> {
            Ok(AuthorizationProof {
                authorization_id: "test-auth".to_string(),
                operation: CrossNodeOperation::default(),
                timestamp: Utc::now(),
                proof_signature: "test-signature".to_string(),
            })
        }
    }
}
