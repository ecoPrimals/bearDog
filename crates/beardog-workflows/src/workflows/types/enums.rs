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


/// Workflow enumeration types
///
/// Contains all enum definitions for the workflows module.
/// WorkflowStatus and AuditAction are now imported from canonical types.
use serde::{Deserialize, Serialize};

// Import canonical types instead of defining duplicates
pub use beardog_types::canonical::workflow::{WorkflowStatus, AuditAction};

/// Workflow types supported by the engine
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowType {
    /// Cryptographic key rotation workflow
    KeyRotation,
    /// Cryptographic key deletion workflow
    KeyDeletion,
    /// Security policy change workflow
    PolicyChange,
    /// System configuration change workflow
    ConfigurationChange,
    /// User account provisioning workflow
    UserProvisioning,
    /// Emergency access request workflow
    EmergencyAccess,
    /// System maintenance workflow
    SystemMaintenance,
    /// Compliance audit workflow
    ComplianceAudit,
}
// WorkflowStatus is now imported from beardog_types::canonical::workflow
// (removed duplicate definition)

/// Approval decision enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalDecision {
    /// Approval granted
    Granted,
    /// Approval rejected with reason
    Rejected(String),
    /// Approval abstained
    Abstained,
    /// Approval pending review
    Pending,
}

impl std::fmt::Display for ApprovalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalDecision::Granted => write!(f, "Granted"),
            ApprovalDecision::Rejected(reason) => write!(f, "Rejected: {reason}"),
            ApprovalDecision::Abstained => write!(f, "Abstained"),
            ApprovalDecision::Pending => write!(f, "Pending"),
        }
    }
}
/// Workflow action types
pub enum WorkflowAction {
    /// Create or initiate action
    Create,
    /// Update existing resource
    Update,
    /// Delete resource
    Delete,
    /// Execute operation
    Execute,
    /// Validate configuration
    Validate,
    /// Approve pending request
    Approve,
    /// Reject pending request
    Reject,
    /// Cancel ongoing workflow
    Cancel,
    /// Pause workflow execution
    Pause,
    /// Resume paused workflow
    Resume,
}

/// Workflow priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorkflowPriority {
    /// Low priority (routine operations)
    Low,
    /// Normal priority (standard operations)
    Normal,
    /// High priority (important operations)
    High,
    /// Critical priority (urgent operations)
    Critical,
    /// Emergency priority (immediate action required)
    Emergency,
}
/// Workflow execution state
pub enum WorkflowExecutionState {
    /// Not started yet
    NotStarted,
    /// Preparing to execute
    Preparing,
    /// Currently running
    Running,
    /// Paused due to condition
    Paused,
    /// Waiting for external dependency
    WaitingForDependency,
    /// Successfully completed
    CompletedSuccess,
    /// Completed with errors
    CompletedWithErrors,
    /// Failed to complete
    Failed,
    /// Cancelled during execution
    Cancelled,
}

/// Workflow target types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTarget {
    System,
    /// Target is a user account
    User(String),
    /// Target is a system resource
    SystemResource(String),
    /// Target is a cryptographic key
    CryptographicKey(String),
    /// Target is a security policy
    SecurityPolicy(String),
    /// Target is a configuration setting
    Configuration(String),
    /// Target is a service or application
    Service(String),
    /// Target is a database or data store
    Database(String),
    /// Target is a network resource
    NetworkResource(String),
    /// Target is a custom resource type
    Custom {
        resource_type: String,
        identifier: String,
    },
}

/// Notification delivery status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationStatus {
    /// Pending delivery
    Pending,
    /// Successfully delivered
    Delivered,
    /// Delivery failed
    Failed,
    /// Delivery retrying
    Retrying,
    /// Delivery cancelled
    Cancelled,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowType::KeyRotation => write!(f, "Key Rotation"),
            WorkflowType::KeyDeletion => write!(f, "Key Deletion"),
            WorkflowType::PolicyChange => write!(f, "Policy Change"),
            WorkflowType::ConfigurationChange => write!(f, "Configuration Change"),
            WorkflowType::UserProvisioning => write!(f, "User Provisioning"),
            WorkflowType::EmergencyAccess => write!(f, "Emergency Access"),
            WorkflowType::SystemMaintenance => write!(f, "System Maintenance"),
            WorkflowType::ComplianceAudit => write!(f, "Compliance Audit"),
        }
    }
}

// Display implementation is now in canonical WorkflowStatus
// (removed duplicate implementation)

/// Execution status for workflow processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    /// Execution pending
    Pending,
    /// Execution running
    Running,
    /// Execution completed successfully
    Success,
    /// Execution failed
    Failed,
    /// Execution cancelled
    Cancelled,
}

/// Execution result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub message: String,
    pub execution_duration_ms: u64,
    pub data: Option<serde_json::Value>,
}

/// Processing result for workflow processors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub status: ExecutionStatus,
    pub result: ExecutionResult,
}

/// Workflow result for processor implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub metrics: ExecutionMetrics,
    pub result: ExecutionResult,
}

/// Execution metrics for workflow processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: u64,
    pub memory_used: u64,
    pub cpu_time_ms: u64,
}

/// Approval submission from users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSubmission {
    pub workflow_id: String,
    pub approver_id: String,
    pub decision: ApprovalDecision,
    pub signature: String,
    pub comments: Option<String>,
    // Additional fields for compatibility
    pub reason: String,
}

/// Pending approval tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    pub approval_id: String,
    pub tier_level: u32,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub notification_sent: bool,
}

/// Approval status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {
    /// Approval is pending
    Pending,
    /// Approval has been submitted
    Submitted,
    /// Approval has been granted
    Granted,
    /// Approval has been denied
    Denied,
    /// Approval has expired
    Expired,
}

// AuditAction is now imported from beardog_types::canonical::workflow
// (removed duplicate definition)

