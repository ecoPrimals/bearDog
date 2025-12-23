// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

// Canonical workflow types - modernized and unified
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditAction {
    /// Represents create variant
    Create,
    /// Represents update variant
    Update,
    /// Represents delete variant
    Delete,
    /// Represents execute variant
    Execute,
    /// Represents approve variant
    Approve,
    /// Represents reject variant
    Reject,
    /// Represents cancel variant
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// State indicating created
    Created,
    /// Operation in progress
    Pending,
    /// Operation in progress
    InProgress,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed,
    /// State indicating cancelled
    Cancelled,
    /// State indicating suspended
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// State indicating queued
    Queued,
    /// Currently running
    Running,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed,
    /// State indicating cancelled
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowExecutionState {
    /// State indicating initialized
    Initialized,
    /// Currently executing
    Executing,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed,
    /// State indicating cancelled
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowPriority {
    /// Represents low variant
    Low,
    /// Represents normal variant
    Normal,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
    /// Represents emergency variant
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowTarget {
    /// Represents system variant
    System,
    /// Represents user variant
    User,
    /// Represents service variant
    Service,
    /// Represents resource variant
    Resource,
    /// Represents policy variant
    Policy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of workflow
pub enum WorkflowType {
    /// Represents key rotation variant
    KeyRotation,

    /// Represents key deletion variant
    KeyDeletion,

    /// Represents policy change variant
    PolicyChange,

    /// Represents configuration change variant
    ConfigurationChange,

    /// Currently userprovisioning
    UserProvisioning,

    /// Represents emergency access variant
    EmergencyAccess,

    /// Represents system maintenance variant
    SystemMaintenance,

    /// Represents compliance audit variant
    ComplianceAudit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalDecision {
    /// State indicating granted
    Granted,

    /// State indicating rejected
    Rejected(String),

    /// State indicating abstained
    Abstained,

    /// Operation in progress
    Pending,
}

impl std::fmt::Display for ApprovalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Granted => write!(f, "Granted"),
            Self::Rejected(reason) => write!(f, "Rejected: {reason}"),
            Self::Abstained => write!(f, "Abstained"),
            Self::Pending => write!(f, "Pending"),
        }
    }
}

pub enum WorkflowAction {
    /// Represents create variant
    Create,

    /// Represents update variant
    Update,

    /// Represents delete variant
    Delete,

    /// Represents execute variant
    Execute,

    /// Represents validate variant
    Validate,

    /// Represents approve variant
    Approve,

    /// Represents reject variant
    Reject,

    /// Represents cancel variant
    Cancel,

    /// Represents pause variant
    Pause,

    /// Represents resume variant
    Resume,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowIdentifier {
    Named { identifier: String },
    Generated { uuid: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationStatus {
    /// Operation in progress
    Pending,

    /// State indicating delivered
    Delivered,

    /// Error or failure state
    Failed,

    /// Currently retrying
    Retrying,

    /// State indicating cancelled
    Cancelled,
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyRotation => write!(f, "Key Rotation"),
            Self::KeyDeletion => write!(f, "Key Deletion"),
            Self::PolicyChange => write!(f, "Policy Change"),
            Self::ConfigurationChange => write!(f, "Configuration Change"),
            Self::UserProvisioning => write!(f, "User Provisioning"),
            Self::EmergencyAccess => write!(f, "Emergency Access"),
            Self::SystemMaintenance => write!(f, "System Maintenance"),
            Self::ComplianceAudit => write!(f, "Compliance Audit"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether success is enabled
    pub success: bool,
    /// The message value
    pub message: String,
    /// Number of `execution_duration_ms`
    pub execution_duration_ms: u64,
    /// Optional data
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Current status of the component
    pub status: ExecutionStatus,
    /// The result value
    pub result: ExecutionResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Number of `duration_ms`
    pub duration_ms: u64,
    /// Number of `memory_used`
    pub memory_used: u64,
    pub cpu_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub approval_id: String,
    pub approver_id: String,
    /// The decision value
    pub decision: ApprovalDecision,
    /// The signature value
    pub signature: String,
    /// Optional comments
    pub comments: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// The reason value
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationConfig {
    pub escalation_id: String,
    /// Number of `tier_level`
    pub tier_level: u32,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Whether `notification_sent` is enabled
    pub notification_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    /// Operation in progress
    Pending,
    /// State indicating submitted
    Submitted,
    /// State indicating granted
    Granted,
    /// State indicating denied
    Denied,
    /// State indicating expired
    Expired,
}
