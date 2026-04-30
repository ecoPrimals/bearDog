// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    missing_docs,
    reason = "coordination config types — serde-derived structs with self-documenting field names"
)]

//! Consensus, rotation, expertise, and leadership types for coordination.

use super::supporting::{ExpertiseRotation, PerformanceThresholds, WorkloadThresholds};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of consensus
pub enum ConsensusType {
    /// Simple majority voting
    Majority,
    /// Requires supermajority (2/3+)
    Supermajority { threshold: f64 },
    /// All participants must agree
    Unanimous,
    /// Weighted voting based on expertise/stake
    Weighted { weights: HashMap<String, f64> },
    /// Consensus through iterative discussion
    Deliberative { max_rounds: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionThresholds {
    /// The routine threshold value
    pub routine_threshold: f64,
    /// The significant threshold value
    pub significant_threshold: f64,
    /// The critical threshold value
    pub critical_threshold: f64,
    /// The emergency threshold value
    pub emergency_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationCriteria {
    /// Rotate based on time intervals
    pub time_based: Option<Duration>,
    /// Rotate based on workload
    /// Optional workload based
    pub workload_based: Option<WorkloadThresholds>,
    /// Rotate based on expertise needs
    /// Optional expertise based
    pub expertise_based: Option<ExpertiseRotation>,
    pub performance_based: Option<PerformanceThresholds>,
}

/// Mapping of expertise areas to participants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseMapping {
    /// Map of expertise areas to qualified participants
    /// Mapping of expertise areas
    pub expertise_areas: HashMap<String, Vec<ExpertiseEntry>>,
    /// Mapping of participant scores
    pub participant_scores: HashMap<String, f64>,
    /// Last updated timestamp
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseEntry {
    /// Participant identifier
    pub participant_id: Arc<str>,
    /// Expertise level (0.0-1.0)
    /// The expertise level value
    pub expertise_level: f64,
    /// Evidence of expertise
    pub evidence: Vec<String>,
    /// Last validated timestamp
    pub last_validated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationSchedule {
    /// How often to evaluate rotation needs
    /// The evaluation frequency value
    pub evaluation_frequency: Duration,
    /// Minimum time in leadership role
    /// The minimum tenure value
    pub minimum_tenure: Duration,
    /// Maximum time in leadership role
    /// The maximum tenure value
    pub maximum_tenure: Duration,
    /// Next scheduled evaluation
    /// The next evaluation value
    pub next_evaluation: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderInfo {
    /// Leader identifier
    pub leader_id: Arc<str>,
    /// When leadership started
    /// The started at value
    pub started_at: DateTime<Utc>,
    /// Expected end of leadership
    /// The expected end value
    pub expected_end: DateTime<Utc>,
    /// The leadership reason value
    pub leadership_reason: Arc<str>,
    pub performance_metrics: HashMap<Arc<str>, f64>,
}
