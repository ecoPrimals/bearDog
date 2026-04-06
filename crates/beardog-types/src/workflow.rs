// SPDX-License-Identifier: AGPL-3.0-or-later

// Workflow Types and Configurations
//
// This module provides workflow-related types and configurations for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique workflow identifier
    pub id: Uuid,
    /// Human-readable workflow name
    /// Name of the item
    pub name: String,
    /// Detailed description of the workflow purpose
    /// The description value
    pub description: String,
    /// The version value
    pub version: String,
    /// Ordered list of workflow steps to execute
    /// Collection of steps
    pub steps: Vec<WorkflowStep>,
    /// Events that can trigger this workflow
    /// Collection of triggers
    pub triggers: Vec<WorkflowTrigger>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp when the workflow was created
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Timestamp when the workflow was last updated
    /// The updated at value
    pub updated_at: DateTime<Utc>,
}

/// Workflow execution instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Unique execution identifier
    pub id: Uuid,
    /// Reference to the workflow being executed
    pub workflow_id: Uuid,
    /// Current execution status
    /// Current status of the component
    pub status: ExecutionStatus,
    /// Timestamp when execution started
    /// The started at value
    pub started_at: DateTime<Utc>,
    /// Timestamp when execution completed (if finished)
    /// Optional completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Results of individual step executions
    /// Collection of step executions
    pub step_executions: Vec<StepExecution>,
    /// Runtime context and variables
    /// Mapping of context
    pub context: HashMap<String, serde_json::Value>,
    /// Error message if execution failed
    /// Optional error message
    pub error_message: Option<String>,
}

/// Individual workflow step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Unique step identifier within the workflow
    pub id: String,
    /// Human-readable step name
    /// Name of the item
    pub name: String,
    /// The step type value
    pub step_type: StepType,
    /// Step-specific configuration parameters
    pub configuration: HashMap<String, serde_json::Value>,
    /// Collection of dependencies
    pub dependencies: Vec<String>,
    /// Optional timeout in seconds for this workflow step
    pub timeout_seconds: Option<u64>,
    /// Number of times to retry on failure
    /// Number of retry
    pub retry_count: u32,
}

/// Step execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecution {
    /// Reference to the step being executed
    pub step_id: String,
    /// Current execution status of this step
    /// Current status of the component
    pub status: ExecutionStatus,
    /// Timestamp when step execution started
    /// The started at value
    pub started_at: DateTime<Utc>,
    /// Timestamp when step execution completed (if finished)
    /// Optional completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Output data produced by the step
    /// Optional output
    pub output: Option<serde_json::Value>,
    /// Error message if step execution failed
    /// Optional error message
    pub error_message: Option<String>,
    /// Number of retry attempts made
    /// Number of `retry_attempts`
    pub retry_attempts: u32,
}

/// Workflow trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
    /// Unique trigger identifier
    pub id: String,
    /// Type of trigger mechanism
    /// The trigger type value
    pub trigger_type: TriggerType,
    /// Trigger-specific configuration
    pub configuration: HashMap<String, serde_json::Value>,
    /// Whether this trigger is currently active
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Workflow approval request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowApproval {
    /// Unique approval request identifier
    pub id: Uuid,
    /// Reference to the workflow execution requiring approval
    pub workflow_execution_id: Uuid,
    /// Step that requires approval
    pub step_id: String,
    /// Timestamp when approval was requested
    /// The requested at value
    pub requested_at: DateTime<Utc>,
    /// Timestamp when approval was granted (if approved)
    /// Optional approved at
    pub approved_at: Option<DateTime<Utc>>,
    /// User who approved the request
    /// Optional approved by
    pub approved_by: Option<String>,
    /// Current approval status
    /// Current status of the approval
    pub approval_status: ApprovalStatus,
    /// Additional comments from approver
    /// Optional comments
    pub comments: Option<String>,
}

/// Execution status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// Workflow is waiting to start
    Pending,
    /// Workflow is currently executing
    Running,
    /// Workflow has completed successfully
    Completed,
    /// Workflow has failed with an error
    Failed,
    /// Workflow has been cancelled and will not continue
    Cancelled,
    /// Workflow is paused and can be resumed
    Paused,
    /// Workflow is waiting for manual approval before proceeding
    WaitingForApproval,
}

/// Step type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of step
pub enum StepType {
    /// Execute an action or operation
    Action,
    /// Conditional step that branches based on evaluation
    Condition,
    /// Loop over a set of items
    Loop,
    /// Execute multiple steps in parallel
    Parallel,
    /// Step requiring manual approval before proceeding
    Approval,
    /// Call an external webhook
    Webhook,
    /// Execute a custom script
    Script,
    /// Custom step type with specified behavior
    Custom(String),
}

/// Trigger type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of trigger
pub enum TriggerType {
    /// Manually triggered by user action
    Manual,
    /// Triggered on a schedule
    Scheduled,
    /// Triggered by an event
    Event,
    /// Triggered by webhook call
    Webhook,
    /// Triggered by file system changes
    FileChange,
    /// Custom trigger type with specified behavior
    Custom(String),
}

/// Approval status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    /// Approval is pending review
    Pending,
    /// Approval has been granted
    Approved,
    /// Approval has been rejected
    Rejected,
    /// Approval request has expired
    Expired,
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Untitled Workflow".to_string(),
            description: String::new(),
            version: "1.0.0".to_string(),
            steps: vec![],
            triggers: vec![],
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Default for WorkflowExecution {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            workflow_id: Uuid::new_v4(),
            status: ExecutionStatus::Pending,
            started_at: Utc::now(),
            completed_at: None,
            step_executions: vec![],
            context: HashMap::new(),
            error_message: None,
        }
    }
}

impl ExecutionStatus {
    /// Check if execution status is terminal (completed, failed, or cancelled)
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    /// Check if execution status is active (running, pending, or waiting)
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(
            self,
            Self::Running | Self::Pending | Self::WaitingForApproval
        )
    }
}

// Re-export canonical workflow types when available
// Note: Using unified canonical providers for workflow functionality
pub use crate::canonical::providers_unified::traits::*;

#[cfg(test)]
mod workflow_type_tests {
    use super::{
        ApprovalStatus, ExecutionStatus, StepType, TriggerType, Workflow, WorkflowExecution,
    };
    use serde_json::json;

    #[test]
    fn execution_status_terminal_and_active() {
        assert!(ExecutionStatus::Completed.is_terminal());
        assert!(ExecutionStatus::Failed.is_terminal());
        assert!(ExecutionStatus::Cancelled.is_terminal());
        assert!(!ExecutionStatus::Running.is_terminal());

        assert!(ExecutionStatus::Running.is_active());
        assert!(ExecutionStatus::Pending.is_active());
        assert!(ExecutionStatus::WaitingForApproval.is_active());
        assert!(!ExecutionStatus::Completed.is_active());
    }

    #[test]
    fn workflow_defaults_are_serializable() {
        let w: Workflow = Workflow::default();
        let v = serde_json::to_value(&w).expect("serialize workflow default");
        assert_eq!(
            v.get("name").and_then(|x| x.as_str()),
            Some("Untitled Workflow")
        );
    }

    #[test]
    fn workflow_execution_default_roundtrip() {
        let e: WorkflowExecution = WorkflowExecution::default();
        let json = serde_json::to_string(&e).expect("to string");
        let back: WorkflowExecution = serde_json::from_str(&json).expect("from str");
        assert_eq!(back.status, ExecutionStatus::Pending);
    }

    #[test]
    fn step_and_trigger_enums_json() {
        let st = StepType::Custom("x".into());
        assert_eq!(serde_json::to_value(&st).unwrap(), json!({"Custom": "x"}));
        let tt = TriggerType::Custom("y".into());
        assert_eq!(serde_json::to_value(&tt).unwrap(), json!({"Custom": "y"}));
        let ap = ApprovalStatus::Rejected;
        assert_eq!(serde_json::to_string(&ap).unwrap(), "\"Rejected\"");
    }
}
