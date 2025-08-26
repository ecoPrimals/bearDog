

use serde::{Deserialize, Serialize};

pub use beardog_types::canonical::workflow::{WorkflowStatus, AuditAction};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowType {

    KeyRotation,

    KeyDeletion,

    PolicyChange,

    ConfigurationChange,

    UserProvisioning,

    EmergencyAccess,

    SystemMaintenance,

    ComplianceAudit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalDecision {

    Granted,

    Rejected(String),

    Abstained,

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

pub enum WorkflowAction {

    Create,

    Update,

    Delete,

    Execute,

    Validate,

    Approve,

    Reject,

    Cancel,

    Pause,

    Resume,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorkflowPriority {

    Low,

    Normal,

    High,

    Critical,

    Emergency,
}

pub enum WorkflowExecutionState {

    NotStarted,

    Preparing,

    Running,

    Paused,

    WaitingForDependency,

    CompletedSuccess,

    CompletedWithErrors,

    Failed,

    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTarget {
    System,

    User(String),

    SystemResource(String),

    CryptographicKey(String),

    SecurityPolicy(String),

    Configuration(String),

    Service(String),

    Database(String),

    NetworkResource(String),

    Custom {
        resource_type: String,
        identifier: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationStatus {

    Pending,

    Delivered,

    Failed,

    Retrying,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {

    Pending,

    Running,

    Success,

    Failed,

    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub message: String,
    pub execution_duration_ms: u64,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub status: ExecutionStatus,
    pub result: ExecutionResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub metrics: ExecutionMetrics,
    pub result: ExecutionResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_ms: u64,
    pub memory_used: u64,
    pub cpu_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSubmission {
    pub workflow_id: String,
    pub approver_id: String,
    pub decision: ApprovalDecision,
    pub signature: String,
    pub comments: Option<String>,

    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    pub approval_id: String,
    pub tier_level: u32,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub notification_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {

    Pending,

    Submitted,

    Granted,

    Denied,

    Expired,
}

