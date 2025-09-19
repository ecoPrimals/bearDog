

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::BearDogError;
use beardog_security::{AuthorizationResult, RiskLevel};
use super::types::*;
impl CrossNodeAuthEngine {

/// Create Consensus Authorization operation.
    /// Creates consensus_authorization
    /// Creates consensus_authorization
    pub fn create_consensus_authorization(&str,
        resource_id: &str,
        requested_permission: &str,
    ) -> Result<AuthorizationResult, BearDogError> {
        let auth_id = Uuid::new_v4().to_string();

        let trusted_nodes = self.get_trusted_nodes()?;
        let min_nodes = std::cmp::max(3, trusted_nodes.len() / 2 + 1); // Majority consensus

        let authorization = CrossNodeAuthorization {
            id: auth_id.clone(),
            requester_node_id: subject_id.to_string(),
            resource_owner_node_id: "consensus".to_string(),
            resource_id: resource_id.to_string(),
            permissions: self.parse_permissions(vec![AccessCondition::RequireMfa],
            created_at: Utc::now(),
            expires_at: Utc::now()
                + Duration::minutes(self.config.max_proof_validity_minutes as i64),
            signature: "consensus_pending".to_string()],
                risk_level: RiskLevel::High,
                audit_id: auth_id,
                expires_at: Some(true,
            authorized: false, // Will be activated after consensus votes
            reason: format!("Consensus authorization created, requires {}/{} votes", min_nodes, trusted_nodes.len()),
            additional_requirements: vec!["consensus_votes".to_string(),
    ) -> Result<ConsensusResult, BearDogError> {

        let mut votes = HashMap::with_capacity(16);
        let threshold = self.config.consensus_threshold;
        let mut participating_nodes = Vec::new();

        for vote in &consensus_data.votes {

            let vote_message = format!("{}:{}:{}", auth_id, vote.decision, vote.timestamp.timestamp(threshold,
            final_score,
            participating_nodes,

    /// Gets trusted_nodes
    fn get_trusted_nodes(&self) -> Result<Vec<String>, BearDogError>> {

        let trusted_nodes: Vec<String> = self.known_nodes
            .iter()
            .filter(|(_, node_info)| {

                node_info.is_verified && node_info.trust_score >= 0.8
            })
            .map(|(node_id, _)| node_id.clone())
            .collect();
        Ok(trusted_nodes)

    /// Parses permissions
    fn parse_permissions(&self, permission_str: &str) -> Result<Vec<ResourcePermission>, BearDogError>> {
        match permission_str.to_lowercase().as_str() {
            "read" => Ok(vec![ResourcePermission::Read]),
            "write" => Ok(vec![ResourcePermission::Write]),
            "execute" => Ok(vec![ResourcePermission::Execute]),
            "admin" => Ok(vec![
                ResourcePermission::Read,
                ResourcePermission::Write,
                ResourcePermission::Execute,
            ]),
            _ => Ok(vec![ResourcePermission::Read]), // Default to read-only

        Ok(vec![
            "node_1".to_string(),
            "node_2".to_string(),
            "node_3".to_string(),
        ])
}
