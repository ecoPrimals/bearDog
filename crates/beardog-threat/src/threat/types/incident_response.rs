// SPDX-License-Identifier: AGPL-3.0-or-later

//! Human-driven incident workflow tied to threat events.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Tracks human-driven incident workflow tied to a threat event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    /// Incident identifier
    pub id: String,
    /// Id of the originating threat event.
    pub threat_id: String,
    /// Response status
    /// Current status of the component
    pub status: ResponseStatus,
    /// Response team assigned
    /// The assigned team value
    pub assigned_team: String,
    /// Response start time
    /// The started at value
    pub started_at: SystemTime,
    /// Response completion time
    /// Optional completed at
    pub completed_at: Option<SystemTime>,
    /// Response actions taken
    /// Collection of actions
    pub actions: Vec<ResponseAction>,
    /// Response notes
    /// The notes value
    pub notes: String,
}

/// Status of incident response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    /// Response initiated
    Initiated,
    /// Response in progress
    InProgress,
    /// Response escalated
    Escalated,
    /// Response completed
    Completed,
    /// Response failed
    Failed,
}

/// Single step executed as part of an incident-response playbook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    /// Action identifier
    pub id: String,
    /// Action type
    /// The action type value
    pub action_type: String,
    /// Action description
    /// The description value
    pub description: String,
    /// Action timestamp
    pub timestamp: SystemTime,
    /// Action result
    /// The result value
    pub result: String,
    /// Action success status
    /// Whether success is enabled
    pub success: bool,
}
