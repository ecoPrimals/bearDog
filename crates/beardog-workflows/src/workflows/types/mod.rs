pub use beardog_types::canonical::configuration::consolidated::WorkflowConfig;
pub use beardog_types::canonical::workflow::WorkflowType;

pub mod enums;

pub use enums::{
    ApprovalStatus, AuditAction, ExecutionStatus, WorkflowExecutionState, WorkflowPriority,
    WorkflowTarget,
};

// Simple request/response types for compatibility
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowRequest {
    pub id: String,
    pub workflow_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowResponse {
    pub id: String,
    pub status: String,
    pub result: Option<serde_json::Value>,
}
