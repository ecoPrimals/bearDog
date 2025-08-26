

use beardog_types::canonical::workflow::{WorkflowPriority, WorkflowType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::super::approval::ApprovalRecord;
use beardog_types::configuration::ApprovalRequirements;
use beardog_types::canonical::workflow::WorkflowStatus;

pub type ProcessorName = String;
pub type WorkflowId = String;
pub type ErrorMessage = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalWorkflow {

    pub id: String,

    pub workflow_type: WorkflowType,

    pub status: WorkflowStatus,

    pub priority: WorkflowPriority,

    pub description: Option<String>,

    pub target: String,

    pub initiator: String,

    pub requested_by: String,

    pub expires_at: Option<DateTime<Utc>>,

    pub timeout_duration: Option<Duration>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub metadata: HashMap<String, String>,

    pub parameters: HashMap<String, serde_json::Value>,

    pub properties: HashMap<String, serde_json::Value>,

    pub context: WorkflowContext,

    pub approval_requirements: Option<ApprovalRequirements>,

    pub approvals: Vec<ApprovalRecord>,

    pub audit_trail: Vec<WorkflowAuditEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {

    pub initiated_by: String,

    pub target: String,

    pub data: HashMap<String, serde_json::Value>,
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self {
            initiated_by: "system".to_string(),
            target: "default".to_string(),
            data: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {

    pub success: bool,

    pub message: String,

    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {

    pub workflow_type: WorkflowType,

    pub initiator: String,

    pub description: String,

    pub parameters: HashMap<String, serde_json::Value>,

    pub properties: HashMap<String, serde_json::Value>,

    pub priority: WorkflowPriority,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {

    pub success: bool,

    pub message: String,

    pub workflow_id: Option<String>,

    pub estimated_completion: Option<DateTime<Utc>>,

    pub metadata: HashMap<String, serde_json::Value>,
}

pub use beardog_types::canonical::workflow::AuditAction;

pub use beardog_types::canonical::workflow::WorkflowAuditEntry; 