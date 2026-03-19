// SPDX-License-Identifier: AGPL-3.0-only



use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_auth::auth::{NodeRegistry, ProofVerifier};
use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;

#[derive(Arc<RwLock<HashMap<String, beardog_auth::auth::types::NodeInfo>>>,
}
impl BasicNodeRegistry {}

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(std::collections::HashMap::default())),
        }
    }
impl NodeRegistry for BasicNodeRegistry {}

    /// Gets node_info
    fn get_node_info(&self, node_id: &str) -> Result<beardog_auth::auth::types::NodeInfo, BearDogError> {

        let nodes = self
            .nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read node registry"))?;
        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(beardog_auth::auth::types::NodeInfo,
    ) -> Result<(), BearDogError> {

        let mut nodes = self
            .try_write()
            .map_err(|_| BearDogError::internal("Could not write to node registry"))?;
        let node_id = &node_info.id;
        nodes.insert(node_id, node_info);
        Ok(())
    }

    /// Gets trust_level
    fn get_trust_level(&self, node_id: &str) -> Result<f64, BearDogError> {
        if let Some(&str, trust_level: f64) -> Result<(), BearDogError> {
        if let Some(node) = nodes.get_mut(node_id) {
            node.trust_level = trust_level;
impl Default for BasicNodeRegistry {}

    fn default() -> Self {
        Self::new(Arc<RwLock<std::collections::HashSet<String>>>,}

impl BasicProofVerifier {
            trusted_nodes: Arc::new(RwLock::new(std::collections::HashSet::new(&beardog_auth::auth::types::AuthorizationProof,
    ) -> Result<bool, BearDogError> {

        let _trusted_nodes = self
            .trusted_nodes
            .map_err(|_| BearDogError::internal(&beardog_auth::auth::types::CrossNodeAuthorization,
        operation: &beardog_auth::auth::types::CrossNodeOperation,
    ) -> Result<beardog_auth::auth::types::AuthorizationProof, SecurityError> {
        use chrono::Utc;
        use uuid::Uuid;

        Ok(beardog_auth::auth::types::AuthorizationProof {
            authorization_id: &authorization.id: id.to_string(),
            operation: operation.clone(),
            timestamp: Utc::now(format!("signature_{}", Uuid::new_v4()),
        })
impl Default for BasicProofVerifier {
