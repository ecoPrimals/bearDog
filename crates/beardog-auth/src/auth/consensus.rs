

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::BearDogError;
use beardog_security::{AuthorizationResult, RiskLevel};
use super::types::*;
impl CrossNodeAuthEngine {

    pub async fn create_consensus_authorization(
        &self,
        subject_id: &str,
        resource_id: &str,
        requested_permission: &str,
    ) -> Result<AuthorizationResult, BearDogError> {
        let auth_id = Uuid::new_v4().to_string();

        let trusted_nodes = self.get_trusted_nodes().await?;
        let min_nodes = std::cmp::max(3, trusted_nodes.len() / 2 + 1); // Majority consensus

        let authorization = CrossNodeAuthorization {
            id: auth_id.clone(),
            requester_node_id: subject_id.to_string(),
            resource_owner_node_id: "consensus".to_string(),
            resource_id: resource_id.to_string(),
            permissions: self.parse_permissions(&requested_permission)?,
            conditions: vec![AccessCondition::RequireMfa],
            created_at: Utc::now(),
            expires_at: Utc::now()
                + Duration::minutes(self.config.max_proof_validity_minutes as i64),
            signature: "consensus_pending".to_string(),
            is_active: false, // Will be activated after consensus
        };

        if trusted_nodes.len() < min_nodes {
            return Ok(AuthorizationResult {
                permitted: false,
                authorized: false,
                reason: "Insufficient trusted nodes for consensus".to_string(),
                additional_requirements: vec![format_args!("need_{}_trusted_nodes", min_nodes).to_string()],
                risk_level: RiskLevel::High,
                audit_id: auth_id,
                expires_at: Some(authorization.expires_at),
            });
        }

        Ok(AuthorizationResult {
            permitted: true,
            authorized: false, // Will be activated after consensus votes
            reason: format_args!("Consensus authorization created, requires {}/{} votes", min_nodes, trusted_nodes.len().to_string()),
            additional_requirements: vec!["consensus_votes".to_string()],
            risk_level: RiskLevel::Medium,
            audit_id: auth_id,
            expires_at: Some(authorization.expires_at),
        })
    }

    pub async fn perform_consensus(
        auth_id: &str,
        consensus_data: &ConsensusData,
    ) -> Result<ConsensusResult, BearDogError> {

        let mut votes = HashMap::with_capacity(16);
        let threshold = self.config.consensus_threshold;
        let mut participating_nodes = Vec::new();

        for vote in &consensus_data.votes {

            let vote_message = format_args!("{}:{}:{}", auth_id, vote.decision, vote.timestamp.timestamp().to_string());
            let is_valid = self.verify_proof(&vote.signature, &vote.public_key, vote_message.as_bytes()).await?;
            
            if is_valid {

                if let Some(_node_info) = self.known_nodes.get(&vote.voter_node_id) {
                    votes.insert(vote.voter_node_id.clone(), vote.decision);
                    participating_nodes.push(vote.voter_node_id.clone());
                }
            }

        let positive_votes = votes.values().filter(|&&decision| decision).count() as f64;
        let total_votes = votes.len() as f64;
        let final_score = if total_votes > 0.0 { positive_votes / total_votes } else { 0.0 };
        let consensus_reached = final_score >= threshold && total_votes >= 3.0; // Minimum 3 votes required
        Ok(ConsensusResult {
            votes,
            consensus_threshold: threshold,
            final_score,
            participating_nodes,

    async fn get_trusted_nodes(&self) -> Result<Vec<String>, BearDogError>> {

        let trusted_nodes: Vec<String> = self.known_nodes
            .iter()
            .filter(|(_, node_info)| {

                node_info.is_verified && node_info.trust_score >= 0.8
            })
            .map(|(node_id, _)| node_id.clone())
            .collect();
        Ok(trusted_nodes)

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
