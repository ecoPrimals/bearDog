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


/// Consensus types for multi-party authorization
///
/// Types related to consensus-based authorization and voting.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Data for consensus evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusData {
    /// The authorization being voted on
    pub authorization_id: String,
    /// Voting nodes and their votes
    pub votes: HashMap<String, ConsensusVote>,
    /// Timestamp when consensus started
    pub started_at: DateTime<Utc>,
    /// Deadline for consensus completion
    pub deadline: DateTime<Utc>,
    /// Additional context for the vote
    pub context: HashMap<String, String>,
}
/// Individual consensus vote
pub struct ConsensusVote {
    /// Node that cast the vote
    pub voter_id: String,
    /// Whether the vote is in favor
    pub in_favor: bool,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Reason for the vote
    pub reason: String,
    /// When the vote was cast
    pub cast_at: DateTime<Utc>,
/// Result of consensus evaluation
pub struct ConsensusResult {
    /// All votes received
    /// Required consensus threshold
    pub consensus_threshold: f64,
    /// Final consensus score
    pub final_score: f64,
    /// Nodes that participated
    pub participating_nodes: Vec<String>,
