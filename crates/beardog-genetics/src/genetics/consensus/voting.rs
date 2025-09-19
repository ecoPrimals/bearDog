

use super::types::{Vote, VotingOption};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Arc<RwLock<HashMap<String, Vec<Vote>>>>, // proposal_id -> votes
    voter_records: Arc<RwLock<HashMap<String, HashMap<String, String>>>>, // proposal_id -> (biome_id -> vote_id)
}

impl VotingManager {
/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            votes: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            voter_records: Arc::new(RwLock::new(HashMap::with_capacity(&str,
        voter_biome: &str,
        selected_option: &str,
        vote_weight: f64,
        genetic_quality: f64,
        trust_score: f64,
        justification: Option<&str>,
    ) -> Result<String, BearDogError> {
        let vote_id = Uuid::new_v4().to_string();
        
        let vote = Vote {
            vote_id: vote_id.clone(),
            proposal_id: proposal_id.to_string(),
            voter_biome: voter_biome.to_string(),
            selected_option: selected_option.to_string(),
            vote_weight,
            genetic_quality,
            trust_score,
            justification,
            cast_at: Utc::now(format!("sig_{}", vote_id), // Placeholder signature
        };

        let mut votes = self.votes.write();
        votes.entry(proposal_id.to_string())
            .or_insert_with(Vec::new)
            .push(vote);

        let mut voter_records = self.voter_records.write();
        voter_records.entry(proposal_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(voter_biome.to_string(), vote_id.clone());

        Ok(vote_id)
    }

/// Get Votes For Proposal operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn get_votes_for_proposal(&self, proposal_id: &str) -> Result<Vec<Vote>, BearDogError> {
        let votes = self.votes.read(&str, biome_id: &str) -> Result<bool, BearDogError> {
        let voter_records = self.voter_records.read();
        Ok(voter_records
            .get(proposal_id)
            .map(|records| records.contains_key(biome_id))
            .unwrap_or(false))
    }

/// Get Vote Count operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets vote_count
    /// Gets vote_count
    pub fn get_vote_count(&self, proposal_id: &str) -> Result<u32, BearDogError> {
        let votes = self.votes.read();
        Ok(votes.get(proposal_id).map(|v| v.len() as u32).unwrap_or(0))
    }

/// Get Weighted Vote Totals operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets weighted_vote_totals
    /// Gets weighted_vote_totals
    pub fn get_weighted_vote_totals(&self, proposal_id: &str) -> Result<HashMap<String, f64>, BearDogError> {
        let votes = self.votes.read();
        let mut totals = HashMap::with_capacity(16);
        
        if let Some(proposal_votes) = votes.get(proposal_id) {
            for vote in proposal_votes {
                *totals.entry(vote.selected_option).or_insert(0.0) += vote.vote_weight;
            }
        }
        
        Ok(totals)
    }
} 
