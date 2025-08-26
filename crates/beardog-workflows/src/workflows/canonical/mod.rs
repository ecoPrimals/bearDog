

pub mod approval;
pub mod configuration;
pub mod core;
pub mod execution;
pub mod processing;

pub use approval::{
    ApprovalRecord, ApprovalStore, InMemoryApprovalStore
};

pub use configuration::{
    ApprovalDecision,
    WorkflowEngineConfig, WorkflowPolicyConfig,

    ApprovalRequirements, ApprovalResponse, ApprovalSubmission, ApprovalTier, PendingApproval
};

pub use core::types::{
    CanonicalWorkflow, WorkflowId, WorkflowAuditEntry, ProcessorName, ErrorMessage
};

pub use execution::{
    WorkflowExecutionStatus, WorkflowStore, WorkflowNotificationEngine, InMemoryWorkflowStore
};

pub use processing::{
    WorkflowProcessingResult
};

pub use beardog_types::canonical::workflow::{WorkflowType, WorkflowPriority};

pub use beardog_types::canonical::workflow::WorkflowStatus;

pub type Workflow = CanonicalWorkflow;
pub type WorkflowApproval = ApprovalRecord;
pub type WorkflowMetrics = execution::metrics::WorkflowMetrics;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {

    pub id: String,

    pub workflow_type: WorkflowType,

    pub data: HashMap<String, serde_json::Value>,

    pub metadata: HashMap<String, String>,

    pub priority: WorkflowPriority,

    pub initiator: String,

    pub parameters: HashMap<String, serde_json::Value>,

    pub description: String,

    pub properties: HashMap<String, serde_json::Value>,

    pub definition: Option<serde_json::Value>,

    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {

    pub data: HashMap<String, serde_json::Value>,

    pub success: bool,

    pub message: String,

    pub workflow_id: Option<String>,

    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,

    pub metadata: HashMap<String, String>,
}

pub use super::zero_cost_engine::WorkflowEngine;
pub use crate::workflows::canonical::processing::WorkflowExecution;

pub use beardog_types::canonical::workflow::AuditAction;
