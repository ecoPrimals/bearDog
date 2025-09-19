// Define WorkflowConfig locally until it's available in beardog_types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub struct WorkflowConfig {
    /// Number of max_concurrent_workflows
    pub max_concurrent_workflows: usize,
    pub default_timeout_seconds: u64,
    /// Whether enable_logging is enabled
    pub enable_logging: bool,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            max_concurrent_workflows: 100,
            default_timeout_seconds: 300,
            enable_logging: true,
        }
    }
}

// Re-export WorkflowType from enums module
pub use enums::WorkflowType;

pub mod enums;

pub use enums::{
    ApprovalStatus, AuditAction, ExecutionStatus, WorkflowExecutionState, WorkflowPriority,
    WorkflowTarget,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowRequest {
    pub id: String,
    /// The workflow type value
    pub workflow_type: String,
    /// The data value
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowResponse {
    pub id: String,
    /// Current status of the component
    pub status: String,
    /// Optional result
    pub result: Option<serde_json::Value>,
}
