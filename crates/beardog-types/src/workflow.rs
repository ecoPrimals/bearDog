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


/// Workflow types and definitions
///
/// Re-exports canonical workflow types and provides legacy compatibility.

pub use crate::canonical::{WorkflowExecutionState, WorkflowStatus, WorkflowType};
// Legacy workflow types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: String,
    pub name: String,
    pub workflow_type: WorkflowType,
    pub steps: Vec<WorkflowStep>,
}
pub struct WorkflowStep {
    pub step_type: String,
    pub config: HashMap<String, serde_json::Value>,
}


pub struct WorkflowExecution {
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub state: WorkflowExecutionState,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
pub struct WorkflowTrigger {
    pub trigger_type: String,
}


pub struct ApprovalRequest {
    pub workflow_execution_id: String,
    pub step_id: String,
    pub requested_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
