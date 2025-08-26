

pub use crate::canonical::{WorkflowExecutionState, WorkflowStatus, WorkflowType};

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
