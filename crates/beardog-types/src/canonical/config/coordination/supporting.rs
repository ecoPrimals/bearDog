// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(missing_docs)]

//! Placeholder and supporting structures for coordination configuration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkloadThresholds {
    /// Number of `max_concurrent_tasks`
    pub max_concurrent_tasks: u32,
    /// The max cpu usage value
    pub max_cpu_usage: f64,
    /// The max memory usage value
    pub max_memory_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExpertiseRotation {
    /// Collection of expertise areas
    pub expertise_areas: Vec<String>,
    /// The rotation frequency value
    pub rotation_frequency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceThresholds {
    /// The min success rate value
    pub min_success_rate: f64,
    pub max_response_time: Duration,
    /// The min availability value
    pub min_availability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextChange {
    pub timestamp: DateTime<Utc>,
    /// The old context value
    pub old_context: Arc<str>,
    /// The new context value
    pub new_context: Arc<str>,
    /// The change reason value
    pub change_reason: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DelegationEvent {
    pub timestamp: DateTime<Utc>,
    /// The event type value
    pub event_type: Arc<str>,
    pub delegation_id: Arc<str>,
    /// The details value
    pub details: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollaborationSession {
    pub session_id: Arc<str>,
    /// Collection of participants
    pub participants: Vec<String>,
    /// The started at value
    pub started_at: DateTime<Utc>,
    /// Current status of the component
    pub status: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmergenceFactor {
    pub factor_id: Arc<str>,
    /// The description value
    pub description: Arc<str>,
    /// The weight value
    pub weight: f64,
    /// The current value value
    pub current_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NaturalLeadership {
    /// Mapping of current leaders
    pub current_leaders: HashMap<String, String>,
    /// Mapping of leadership strength
    pub leadership_strength: HashMap<String, f64>,
    /// Collection of emergence patterns
    pub emergence_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdaptiveHierarchy {
    /// Mapping of current structure
    pub current_structure: HashMap<String, Vec<String>>,
    /// Collection of adaptation rules
    pub adaptation_rules: Vec<String>,
    /// Collection of structure history
    pub structure_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmergenceEvent {
    pub timestamp: DateTime<Utc>,
    /// The event type value
    pub event_type: Arc<str>,
    /// Collection of participants
    pub participants: Vec<String>,
    /// The outcome value
    pub outcome: Arc<str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RollbackStrategy {
    pub strategy_id: Arc<str>,
    /// Collection of rollback steps
    pub rollback_steps: Vec<String>,
    pub validation_checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationCheck {
    pub check_id: Arc<str>,
    /// The check type value
    pub check_type: Arc<str>,
    /// Collection of success criteria
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringMechanism {
    pub mechanism_id: Arc<str>,
    /// The monitoring type value
    pub monitoring_type: Arc<str>,
    /// The frequency value
    pub frequency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedbackSystem {
    pub system_id: Arc<str>,
    /// The feedback type value
    pub feedback_type: Arc<str>,
    /// The collection method value
    pub collection_method: Arc<str>,
}
