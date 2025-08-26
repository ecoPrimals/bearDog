

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_auth::auth::{NodeRegistry, ProofVerifier};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SecurityResult;

#[derive(Debug, Clone)]
pub struct BasicNodeRegistry {
    nodes: Arc<RwLock<HashMap<String, beardog_auth::auth::types::NodeInfo>>>,
}
impl BasicNodeRegistry {}

    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(ahash::HashMap::default())),
        }
    }
impl NodeRegistry for BasicNodeRegistry {}

    fn get_node_info(&self, node_id: &str) -> BearDogResult<beardog_auth::auth::types::NodeInfo> {

        let nodes = self
            .nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read node registry"))?;
        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Node {node_id} not found"))},
            })
    fn register_node(
        &mut self,
        node_info: beardog_auth::auth::types::NodeInfo,
    ) -> BearDogResult<()> {

        let mut nodes = self
            .try_write()
            .map_err(|_| BearDogError::internal("Could not write to node registry"))?;
        nodes.insert(node_info.id.clone(), node_info);
        Ok(())}

    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64> {
        if let Some(node) = nodes.get(node_id) {
            Ok(node.trust_level)
        } else {
            Ok(0.0)
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()> {
        if let Some(node) = nodes.get_mut(node_id) {
            node.trust_level = trust_level;
impl Default for BasicNodeRegistry {}

    fn default() -> Self {
        Self::new()

pub struct BasicProofVerifier {

    trusted_nodes: Arc<RwLock<std::collections::HashSet<String>>>,}

impl BasicProofVerifier {
            trusted_nodes: Arc::new(RwLock::new(std::collections::HashSet::new())),}

impl ProofVerifier for BasicProofVerifier {
    fn verify_authorization_proof(
        &self,
        proof: &beardog_auth::auth::types::AuthorizationProof,
    ) -> BearDogResult<bool> {

        let _trusted_nodes = self
            .trusted_nodes
            .map_err(|_| BearDogError::internal("Could not read trusted nodes"))?;

        Ok(!proof.proof_signature.is_empty() && !proof.authorization_id.is_empty())}

    fn generate_proof(
        authorization: &beardog_auth::auth::types::CrossNodeAuthorization,
        operation: &beardog_auth::auth::types::CrossNodeOperation,
    ) -> Result<beardog_auth::auth::types::AuthorizationProof, SecurityError> {
        use chrono::Utc;
        use uuid::Uuid;

        Ok(beardog_auth::auth::types::AuthorizationProof {
            authorization_id: authorization.id.clone(),
            operation: operation.clone(),
            timestamp: Utc::now(),
            proof_signature: format_args!("signature_{}", Uuid::new_v4().to_string()),
        })
impl Default for BasicProofVerifier {
