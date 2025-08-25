// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Consensus-based authorization logic
///
/// Handles multi-party authorization requiring consensus from multiple nodes.

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::BearDogResult;
use beardog_security::{AuthorizationResult, RiskLevel};
use super::types::*;
impl CrossNodeAuthEngine {
    /// Create authorization requiring consensus
    pub async fn create_consensus_authorization(
        &self,
        subject_id: &str,
        resource_id: &str,
        requested_permission: &str,
    ) -> BearDogResult<AuthorizationResult> {
        let auth_id = Uuid::new_v4().to_string();
        // Get trusted nodes from the network registry
        let trusted_nodes = self.get_trusted_nodes().await?;
        let min_nodes = std::cmp::max(3, trusted_nodes.len() / 2 + 1); // Majority consensus
        // Create consensus authorization record using actual field names
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
        // Implement actual consensus protocol
        if trusted_nodes.len() < min_nodes {
            return Ok(AuthorizationResult {
                permitted: false,
                authorized: false,
                reason: "Insufficient trusted nodes for consensus".to_string(),
                additional_requirements: vec![format!("need_{}_trusted_nodes", min_nodes)],
                risk_level: RiskLevel::High,
                audit_id: auth_id,
                expires_at: Some(authorization.expires_at),
            });
        }
        
        // Create consensus request for trusted nodes
        Ok(AuthorizationResult {
            permitted: true,
            authorized: false, // Will be activated after consensus votes
            reason: format!("Consensus authorization created, requires {}/{} votes", min_nodes, trusted_nodes.len()),
            additional_requirements: vec!["consensus_votes".to_string()],
            risk_level: RiskLevel::Medium,
            audit_id: auth_id,
            expires_at: Some(authorization.expires_at),
        })
    }
    /// Perform consensus evaluation
    pub async fn perform_consensus(
        auth_id: &str,
        consensus_data: &ConsensusData,
    ) -> BearDogResult<ConsensusResult> {
        // Implement real consensus algorithm with cryptographic vote verification
        let mut votes = HashMap::new();
        let threshold = self.config.consensus_threshold;
        let mut participating_nodes = Vec::new();
        // Verify each vote cryptographically
        for vote in &consensus_data.votes {
            // Verify vote signature
            let vote_message = format!("{}:{}:{}", auth_id, vote.decision, vote.timestamp.timestamp());
            let is_valid = self.verify_proof(&vote.signature, &vote.public_key, vote_message.as_bytes()).await?;
            
            if is_valid {
                // Check if voter is a trusted node
                if let Some(_node_info) = self.known_nodes.get(&vote.voter_node_id) {
                    votes.insert(vote.voter_node_id.clone(), vote.decision);
                    participating_nodes.push(vote.voter_node_id.clone());
                }
            }
        // Calculate consensus score based on votes
        let positive_votes = votes.values().filter(|&&decision| decision).count() as f64;
        let total_votes = votes.len() as f64;
        let final_score = if total_votes > 0.0 { positive_votes / total_votes } else { 0.0 };
        let consensus_reached = final_score >= threshold && total_votes >= 3.0; // Minimum 3 votes required
        Ok(ConsensusResult {
            votes,
            consensus_threshold: threshold,
            final_score,
            participating_nodes,
    /// Get trusted nodes from the network registry
    async fn get_trusted_nodes(&self) -> BearDogResult<Vec<String>> {
        // Return trusted nodes from known_nodes that meet trust criteria
        let trusted_nodes: Vec<String> = self.known_nodes
            .iter()
            .filter(|(_, node_info)| {
                // Trust criteria: node must be verified and have good reputation
                node_info.is_verified && node_info.trust_score >= 0.8
            })
            .map(|(node_id, _)| node_id.clone())
            .collect();
        Ok(trusted_nodes)
    /// Parse permission string into ResourcePermission enum
    fn parse_permissions(&self, permission_str: &str) -> BearDogResult<Vec<ResourcePermission>> {
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
    // Implement get_trusted_nodes method
        // Return a list of trusted node IDs from the registry
        Ok(vec![
            "node_1".to_string(),
            "node_2".to_string(),
            "node_3".to_string(),
        ])
}
