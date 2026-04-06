// SPDX-License-Identifier: AGPL-3.0-or-later



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// Mapping of votes
    pub votes: HashMap<String, ConsensusVote>,

    /// The started at value
    pub started_at: DateTime<Utc>,

    /// The deadline value
    pub deadline: DateTime<Utc>,

    /// Mapping of context
    pub context: HashMap<String, String>,
}

pub struct ConsensusVote {


    pub voter_id: String,

    /// Whether in_favor is enabled
    pub in_favor: bool,


    pub confidence: f64,

    /// The reason value
    pub reason: String,

    /// The cast at value
    pub cast_at: DateTime<Utc>,

pub struct ConsensusResult {

    /// The consensus threshold value
    pub consensus_threshold: f64,

    /// The final score value
    pub final_score: f64,

    /// Collection of participating nodes
    pub participating_nodes: Vec<String>,
