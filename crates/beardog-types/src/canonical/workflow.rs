use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkflowStatus {
    Created,

    Pending,

    PendingApprovals,

    Approved,

    Rejected,

    Expired,

    Cancelled,

    Running,

    InProgress,

    Completed,

    Failed,

    Paused,

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
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::Rejected | Self::Expired
        )
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Running | Self::InProgress | Self::Retrying)
    }

    pub fn is_waiting(&self) -> bool {
        matches!(self, Self::PendingApprovals | Self::Paused)
    }

    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Completed)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed)
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionContext {
    pub workflow_id: String,
    pub execution_id: String,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub variables: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AuditAction {
    WorkflowCreated,

    WorkflowSubmitted,

    WorkflowStarted,

    WorkflowApproved,

    WorkflowRejected,

    WorkflowExecuted,

    WorkflowCompleted,

    WorkflowFailed,

    WorkflowCancelled,

    WorkflowPaused,

    WorkflowResumed,

    WorkflowRestarted,

    ParameterUpdated,

    StatusChanged,

    PolicyApplied,

    ApprovalRequested,

    ApprovalGranted,

    ApprovalDenied,

    ApprovalSubmitted,

    TimeoutOccurred,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAuditEntry {
    pub id: String,

    pub workflow_id: String,

    pub timestamp: DateTime<Utc>,

    pub action: AuditAction,

    pub user: String,

    pub actor: String,

    pub actor_ip: Option<String>,

    pub user_agent: Option<String>,

    pub event_type: String,

    pub description: String,

    pub context: HashMap<String, serde_json::Value>,

    pub metadata: HashMap<String, String>,

    pub result: Option<String>,
}

impl WorkflowAuditEntry {
    pub fn new(workflow_id: &str, action: AuditAction, user: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            workflow_id: workflow_id.to_string(),
            timestamp: Utc::now(),
            action,
            user: user.to_string(),
            actor: user.to_string(),
            actor_ip: None,
            user_agent: None,
            event_type: "workflow_audit".to_string(),
            description: description.to_string(),
            context: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(16),
            result: None,
        }
    }

    pub fn with_context(mut self, context: HashMap<&str, serde_json::Value>) -> Self {
        self.context = context
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<&str, &str>) -> Self {
        self.metadata = metadata
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    pub fn with_result(mut self, result: &str) -> Self {
        self.result = Some(result.to_string());
        self
    }
}
