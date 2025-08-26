

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusData {

    pub authorization_id: String,

    pub votes: HashMap<String, ConsensusVote>,

    pub started_at: DateTime<Utc>,

    pub deadline: DateTime<Utc>,

    pub context: HashMap<String, String>,
}

pub struct ConsensusVote {

    pub voter_id: String,

    pub in_favor: bool,

    pub confidence: f64,

    pub reason: String,

    pub cast_at: DateTime<Utc>,

pub struct ConsensusResult {

    pub consensus_threshold: f64,

    pub final_score: f64,

    pub participating_nodes: Vec<String>,
