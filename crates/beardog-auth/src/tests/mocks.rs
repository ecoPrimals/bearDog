

use beardog_errors::BearDogResult;
use beardog_types::canonical::WorkflowStatus;

use crate::auth::types::*;

use chrono::Utc;

pub struct MockNodeRegistry;
impl NodeRegistry for MockNodeRegistry {}

    fn get_node_info(&self, node_id: &str) -> BearDogResult<NodeInfo> {
        let trust_level = match node_id {
            "trusted-node" => 0.9,
            "medium-node" => 0.5,
            "untrusted-node" => 0.1,
            _ => 0.0,
        };
        let capabilities = match node_id {
            "trusted-node" => vec![
                NodeCapability::StorageProvider,
                NodeCapability::ComputeProvider,
                NodeCapability::SecurityAnalysis,
            ],
            "medium-node" => vec![
            "untrusted-node" => vec![NodeCapability::StorageProvider],
            _ => vec![],
        Ok(NodeInfo {
            id: node_id.to_string(),
            address: format_args!("127.0.0.1:800{}", node_id.len().to_string()),
            capabilities,
            trust_level,
            last_seen: Utc::now(),
            genetics: None,
        })
    }
    fn register_node(&mut self, _node_info: NodeInfo) -> BearDogResult<()> {
        Ok(())}

    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64> {
        match node_id {
            "trusted-node" => Ok(0.9),
            "medium-node" => Ok(0.5),
            "untrusted-node" => Ok(0.1),
            _ => Ok(0.0),
        }
    fn update_trust_level(&mut self, _node_id: &str, _trust_level: f64) -> BearDogResult<()> {
}
pub struct MockProofVerifier;
impl ProofVerifier for MockProofVerifier {}

    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> BearDogResult<bool> {

        Ok(!proof.proof_signature.is_empty())}

    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        _operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof> {
        Ok(AuthorizationProof {
            authorization_id: authorization.id.clone(),
            operation: _operation.clone(),
            proof_signature: "mock-signature".to_string(),
            timestamp: Utc::now(),}

pub struct MockWorkflowEngine;
impl WorkflowEngine for MockWorkflowEngine {}

    fn submit_workflow(&mut self, _request: CrossNodeWorkflowRequest) -> BearDogResult<String> {
        Ok("mock-workflow-id".to_string())}

    fn get_workflow_status(&self, _workflow_id: &str) -> BearDogResult<WorkflowStatus> {
        Ok(WorkflowStatus::Completed)
