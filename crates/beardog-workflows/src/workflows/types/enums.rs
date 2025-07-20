//! Workflow enumeration types
//!
//! Contains all enum definitions for the workflows module.

use serde::{Deserialize, Serialize};

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

/// Workflow status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
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
    /// Workflow is currently being executed
    InProgress,
    /// Workflow execution completed successfully
    Completed,
    /// Workflow execution failed with error
    Failed,
}

/// Approval decision enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    /// Approval granted
    Approved,
    /// Approval rejected with reason
    Rejected,
    /// Approval abstained
    Abstained,
}

impl std::fmt::Display for ApprovalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalDecision::Approved => write!(f, "Approved"),
            ApprovalDecision::Rejected => write!(f, "Rejected"),
            ApprovalDecision::Abstained => write!(f, "Abstained"),
        }
    }
}

/// Workflow action types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl std::fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowStatus::PendingApprovals => write!(f, "Pending Approvals"),
            WorkflowStatus::Approved => write!(f, "Approved"),
            WorkflowStatus::Rejected => write!(f, "Rejected"),
            WorkflowStatus::Expired => write!(f, "Expired"),
            WorkflowStatus::Cancelled => write!(f, "Cancelled"),
            WorkflowStatus::InProgress => write!(f, "In Progress"),
            WorkflowStatus::Completed => write!(f, "Completed"),
            WorkflowStatus::Failed => write!(f, "Failed"),
        }
    }
}
