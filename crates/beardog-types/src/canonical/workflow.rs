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


/// # Canonical Workflow Types - Minimal Version
///
/// **TEMPORARY MINIMAL IMPLEMENTATION** for build stability

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid;

/// **CANONICAL** Workflow Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkflowType {
    Security,
    Compliance,
    Approval,
    Deployment,
    Monitoring,
    Generic,
    KeyRotation,
    PolicyChange,
    ConfigurationChange,
    UserProvisioning,
}

impl Default for WorkflowType {
    fn default() -> Self {
        Self::Generic
    }
}

/// **CANONICAL** Workflow Status - Single source of truth for all workflow states
/// 
/// This enum consolidates all workflow status variants from across the codebase
/// into a single canonical definition that replaces all other WorkflowStatus types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkflowStatus {
    /// Workflow has been created but not yet submitted
    Created,
    /// Workflow is in pending state (initial state)
    Pending,
    /// Workflow is waiting for required approvals
    PendingApprovals,
    /// Workflow has been approved and ready for execution
    Approved,
    /// Workflow has been rejected by approvers
    Rejected,
    /// Workflow has expired due to timeout
    Expired,
    /// Workflow has been cancelled by initiator or system
    Cancelled,
    /// Workflow is currently being executed (alias for InProgress)
    Running,
    /// Workflow is currently being executed
    InProgress,
    /// Workflow execution completed successfully
    Completed,
    /// Workflow execution failed with error
    Failed,
    /// Workflow is paused and waiting for manual intervention
    Paused,
    /// Workflow is being retried after a failure
    Retrying,
}

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Created
    }
}

impl std::fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Created => write!(f, "Created"),
            Self::Pending => write!(f, "Pending"),
            Self::PendingApprovals => write!(f, "PendingApprovals"),
            Self::Approved => write!(f, "Approved"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Expired => write!(f, "Expired"),
            Self::Cancelled => write!(f, "Cancelled"),
            Self::Running => write!(f, "Running"),
            Self::InProgress => write!(f, "InProgress"),
            Self::Completed => write!(f, "Completed"),
            Self::Failed => write!(f, "Failed"),
            Self::Paused => write!(f, "Paused"),
            Self::Retrying => write!(f, "Retrying"),
        }
    }
}

impl WorkflowStatus {
    /// Check if the workflow is in a terminal state (cannot transition further)
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::Rejected | Self::Expired
        )
    }

    /// Check if the workflow is currently active (being processed)
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            Self::Running | Self::InProgress | Self::Retrying
        )
    }

    /// Check if the workflow is waiting for external action
    pub fn is_waiting(&self) -> bool {
        matches!(
            self,
            Self::PendingApprovals | Self::Paused
        )
    }

    /// Check if the workflow completed successfully
    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Check if the workflow failed
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// **CANONICAL** Workflow Execution State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionState {
    pub current_step: usize,
    pub completed_steps: Vec<usize>,
    pub failed_steps: Vec<usize>,
    pub status: WorkflowStatus,
    pub error_message: Option<String>,
}

impl Default for WorkflowExecutionState {
    fn default() -> Self {
        Self {
            current_step: 0,
            completed_steps: Vec::new(),
            failed_steps: Vec::new(),
            status: WorkflowStatus::Pending,
            error_message: None,
        }
    }
}

/// **CANONICAL** Workflow Priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for WorkflowPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// **CANONICAL** Workflow Step Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowStepStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
}

impl Default for WorkflowStepStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// **CANONICAL** Workflow Execution Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionContext {
    pub workflow_id: String,
    pub execution_id: String,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub variables: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

/// **CANONICAL** Workflow Retry Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub retry_on_failure: bool,
}

impl Default for WorkflowRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(300),
            backoff_multiplier: 2.0,
            retry_on_failure: true,
        }
    }
}

/// **CANONICAL** Audit Action - Single source of truth for all workflow audit actions
/// 
/// This enum consolidates all audit action variants from across the codebase
/// into a single canonical definition that replaces all other AuditAction types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditAction {
    /// Workflow was created
    WorkflowCreated,
    /// Workflow was submitted for approval
    WorkflowSubmitted,
    /// Workflow was started/initiated
    WorkflowStarted,
    /// Workflow was approved by approvers
    WorkflowApproved,
    /// Workflow was rejected by approvers  
    WorkflowRejected,
    /// Workflow was executed
    WorkflowExecuted,
    /// Workflow completed successfully
    WorkflowCompleted,
    /// Workflow failed during execution
    WorkflowFailed,
    /// Workflow was cancelled
    WorkflowCancelled,
    /// Workflow was paused
    WorkflowPaused,
    /// Workflow was resumed from pause
    WorkflowResumed,
    /// Workflow was restarted
    WorkflowRestarted,
    /// Workflow parameters were updated
    ParameterUpdated,
    /// Workflow status was changed
    StatusChanged,
    /// Policy was applied to workflow
    PolicyApplied,
    /// Approval was requested
    ApprovalRequested,
    /// Approval was granted
    ApprovalGranted,
    /// Approval was denied
    ApprovalDenied,
    /// Approval was submitted
    ApprovalSubmitted,
    /// Timeout occurred during workflow
    TimeoutOccurred,
    /// System action was performed
    SystemAction,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WorkflowCreated => write!(f, "Workflow Created"),
            Self::WorkflowSubmitted => write!(f, "Workflow Submitted"),
            Self::WorkflowStarted => write!(f, "Workflow Started"),
            Self::WorkflowApproved => write!(f, "Workflow Approved"),
            Self::WorkflowRejected => write!(f, "Workflow Rejected"),
            Self::WorkflowExecuted => write!(f, "Workflow Executed"),
            Self::WorkflowCompleted => write!(f, "Workflow Completed"),
            Self::WorkflowFailed => write!(f, "Workflow Failed"),
            Self::WorkflowCancelled => write!(f, "Workflow Cancelled"),
            Self::WorkflowPaused => write!(f, "Workflow Paused"),
            Self::WorkflowResumed => write!(f, "Workflow Resumed"),
            Self::WorkflowRestarted => write!(f, "Workflow Restarted"),
            Self::ParameterUpdated => write!(f, "Parameter Updated"),
            Self::StatusChanged => write!(f, "Status Changed"),
            Self::PolicyApplied => write!(f, "Policy Applied"),
            Self::ApprovalRequested => write!(f, "Approval Requested"),
            Self::ApprovalGranted => write!(f, "Approval Granted"),
            Self::ApprovalDenied => write!(f, "Approval Denied"),
            Self::ApprovalSubmitted => write!(f, "Approval Submitted"),
            Self::TimeoutOccurred => write!(f, "Timeout Occurred"),
            Self::SystemAction => write!(f, "System Action"),
        }
    }
}

/// **CANONICAL** Workflow Audit Entry - Single source of truth for audit entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAuditEntry {
    /// Unique entry identifier
    pub id: String,
    /// Workflow ID this entry relates to
    pub workflow_id: String,
    /// Timestamp of the action
    pub timestamp: DateTime<Utc>,
    /// Action that was performed
    pub action: AuditAction,
    /// User who performed the action
    pub user: String,
    /// Actor who performed the action (may differ from user)
    pub actor: String,
    /// IP address of the actor
    pub actor_ip: Option<String>,
    /// User agent of the actor
    pub user_agent: Option<String>,
    /// Event type classification
    pub event_type: String,
    /// Human-readable description
    pub description: String,
    /// Additional context data
    pub context: HashMap<String, serde_json::Value>,
    /// Entry metadata
    pub metadata: HashMap<String, String>,
    /// Result of the action
    pub result: Option<String>,
}

impl WorkflowAuditEntry {
    /// Create a new audit entry
    pub fn new(
        workflow_id: String,
        action: AuditAction,
        user: String,
        description: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            workflow_id,
            timestamp: Utc::now(),
            action,
            user: user.clone(),
            actor: user,
            actor_ip: None,
            user_agent: None,
            event_type: "workflow_audit".to_string(),
            description,
            context: HashMap::new(),
            metadata: HashMap::new(),
            result: None,
        }
    }

    /// Create audit entry with additional context
    pub fn with_context(mut self, context: HashMap<String, serde_json::Value>) -> Self {
        self.context = context;
        self
    }

    /// Create audit entry with metadata
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// Create audit entry with result
    pub fn with_result(mut self, result: String) -> Self {
        self.result = Some(result);
        self
    }
}
