//! Type definitions and data structures for workflows
//!
//! Contains all structs, enums, and type aliases for the workflows module.

use chrono::{DateTime, Duration, Utc};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::{config::WorkflowConfig, BearDogResult};

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

/// Approval decision types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    /// Approver approved the workflow
    Approved,
    /// Approver rejected the workflow
    Rejected,
    /// Approver delegated decision to another person
    Delegated,
}

impl std::fmt::Display for ApprovalDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApprovalDecision::Approved => write!(f, "Approved"),
            ApprovalDecision::Rejected => write!(f, "Rejected"),
            ApprovalDecision::Delegated => write!(f, "Delegated"),
        }
    }
}

/// Workflow action for audit trail
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowAction {
    /// Workflow was initiated
    Initiated,
    /// Workflow was approved
    Approved,
    /// Workflow was rejected
    Rejected,
    /// Approval was delegated
    Delegated,
    /// Workflow expired
    Expired,
    /// Workflow was cancelled
    Cancelled,
    /// Workflow was executed
    Executed,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed during execution
    Failed,
}

/// Workflow priority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowPriority {
    /// Low priority workflow
    Low,
    /// Normal priority workflow
    Normal,
    /// High priority workflow
    High,
    /// Critical priority workflow
    Critical,
    /// Emergency priority workflow
    Emergency,
}

/// Workflow execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowExecutionState {
    /// Workflow has been created but not yet queued for execution
    Created,
    /// Workflow is queued and waiting for execution
    Queued,
    /// Workflow is currently being executed
    Running,
    /// Workflow execution completed successfully
    Completed,
    /// Workflow execution failed with an error
    Failed,
    /// Workflow execution was cancelled by user or system
    Cancelled,
}

/// Core workflow data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique workflow identifier
    pub id: String,
    /// Type of workflow being executed
    pub workflow_type: WorkflowType,
    /// User who initiated the workflow
    pub initiator: String,
    /// Target of the workflow operation
    pub target: WorkflowTarget,
    /// Workflow-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Approval requirements for this workflow
    pub approval_requirements: ApprovalRequirements,
    /// Current status of the workflow
    pub status: WorkflowStatus,
    /// Timestamp when workflow was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when workflow expires
    pub expires_at: DateTime<Utc>,
    /// List of approval records
    pub approvals: Vec<ApprovalRecord>,
    /// Complete audit trail of workflow actions
    pub audit_trail: Vec<WorkflowAuditEntry>,
    /// Additional metadata for the workflow
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Target entity for workflow operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowTarget {
    /// System-wide operation
    System,
    /// Operation targeting a specific user
    User {
        /// User identifier
        user_id: String,
    },
    /// Operation targeting a specific resource
    Resource {
        /// Resource identifier
        resource_id: String,
    },
    /// Operation targeting a security policy
    Policy {
        /// Policy identifier
        policy_id: String,
    },
    /// Operation targeting a cryptographic key
    Key {
        /// Key identifier
        key_id: String,
    },
}

/// Approval requirements for a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {
    pub required_approvals: u32,
    pub required_roles: Vec<String>,
    pub approval_hierarchy: Vec<ApprovalTier>,
    pub min_approval_time: Duration,
    pub max_approval_time: Duration,
    pub delegation_allowed: bool,
    pub self_approval_allowed: bool,
}

/// Approval tier in hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTier {
    pub tier_level: u32,
    pub required_approvals: u32,
    pub eligible_roles: Vec<String>,
    pub eligible_users: Vec<String>,
    pub description: String,
}

/// Approval record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String,
    pub workflow_id: String,
    pub approver: String,
    pub approver_role: String,
    pub decision: ApprovalDecision,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Pending approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    pub id: String,
    pub workflow_id: String,
    pub approver: String,
    pub approver_role: String,
    pub tier_level: u32,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub notification_sent: bool,
}

/// Workflow audit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: WorkflowAction,
    pub actor: String,
    pub details: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Workflow request for initiation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    pub workflow_type: WorkflowType,
    pub initiator: String,
    pub target: WorkflowTarget,
    pub parameters: HashMap<String, serde_json::Value>,
    pub reason: String,
    pub priority: WorkflowPriority,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Workflow response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {
    pub workflow_id: String,
    pub status: WorkflowStatus,
    pub required_approvals: ApprovalRequirements,
    pub pending_approvers: Vec<String>,
    pub estimated_completion: DateTime<Utc>,
    pub tracking_url: Option<String>,
}

/// Approval submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSubmission {
    pub workflow_id: String,
    pub approver: String,
    pub decision: ApprovalDecision,
    pub reason: Option<String>,
    pub signature: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Approval response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub success: bool,
    pub workflow_status: WorkflowStatus,
    pub remaining_approvals: u32,
    pub message: String,
}

/// Workflow execution context
#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    pub workflow_id: String,
    pub workflow_type: WorkflowType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Workflow completion result
#[derive(Debug, Clone)]
pub struct WorkflowCompletionResult {
    pub workflow_id: String,
    pub success: bool,
    pub error: Option<String>,
    pub completed_at: DateTime<Utc>,
}

/// Workflow runtime instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    pub id: String,
    pub workflow_type: WorkflowType,
    pub state: WorkflowExecutionState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Workflow instance performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstanceMetrics {
    pub execution_time: Duration,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub network_io: u64,
}

/// Aggregate workflow metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetrics {
    pub active_workflows: u64,
    pub completed_workflows: u64,
    pub failed_workflows: u64,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email_enabled: bool,
    pub email_server: Option<String>,
    pub email_username: Option<String>,
    pub email_password: Option<String>,
    pub sms_enabled: bool,
    pub sms_provider: Option<String>,
    pub sms_api_key: Option<String>,
    pub webhook_enabled: bool,
    pub webhook_url: Option<String>,
    pub webhook_secret: Option<String>,
    pub slack_enabled: bool,
    pub slack_webhook_url: Option<String>,
    pub teams_enabled: bool,
    pub teams_webhook_url: Option<String>,
    pub notification_template_path: Option<String>,
    pub notification_retry_attempts: u32,
    pub notification_retry_delay: Duration,
}

/// Policy configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub default_approval_timeout: Duration,
    pub emergency_approval_timeout: Duration,
    pub max_concurrent_workflows: u32,
    pub max_workflow_age: Duration,
    pub auto_cleanup_enabled: bool,
    pub risk_scoring_enabled: bool,
    pub compliance_checking_enabled: bool,
    pub audit_retention_period: Duration,
    pub notification_escalation_enabled: bool,
    pub escalation_intervals: Vec<Duration>,
    pub delegation_policies: HashMap<String, Vec<String>>,
    pub role_hierarchies: HashMap<String, Vec<String>>,
    pub approval_matrix: HashMap<WorkflowType, ApprovalRequirements>,
    pub emergency_contacts: Vec<String>,
    pub business_hours: Option<BusinessHours>,
}

/// Business hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    pub start_time: String,    // HH:MM format
    pub end_time: String,      // HH:MM format
    pub days_of_week: Vec<u8>, // 0=Sunday, 1=Monday, etc.
    pub timezone: String,
}

/// Multi-party workflow engine for secure collaborative operations
#[derive(Clone)]
pub struct MultiPartyWorkflowEngine {
    pub config: Arc<WorkflowConfig>,
    pub workflow_store: Arc<dyn WorkflowStore>,
    pub approval_store: Arc<dyn ApprovalStore>,
    pub notification_engine: Arc<NotificationEngine>,
    pub policy_engine: Arc<WorkflowPolicyEngine>,
    pub scheduler: Arc<WorkflowScheduler>,

    // Active workflows
    pub active_workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    pub pending_approvals: Arc<RwLock<HashMap<String, Vec<PendingApproval>>>>,

    // Workflow execution state
    pub workflow_processors: Arc<RwLock<HashMap<WorkflowType, Box<dyn WorkflowProcessor>>>>,
    pub execution_queue: Arc<Mutex<VecDeque<WorkflowExecution>>>,

    // New fields
    pub node_registry: Arc<RwLock<HashMap<String, String>>>,
    pub consensus_engine: Arc<RwLock<ConsensusEngine>>,
    pub security_provider: Arc<RwLock<WorkflowSecurityProvider>>,
    pub metrics: Arc<RwLock<WorkflowMetrics>>,
}

/// Notification engine for workflow events
#[derive(Debug, Clone)]
pub struct NotificationEngine {
    pub config: NotificationConfig,
}

/// Workflow policy engine
pub struct WorkflowPolicyEngine {
    pub config: PolicyConfig,
}

/// Workflow scheduler
pub struct WorkflowScheduler {
    pub cleanup_enabled: bool,
}

/// In-memory workflow storage
pub struct InMemoryWorkflowStore {
    pub workflows: RwLock<HashMap<String, Workflow>>,
}

/// In-memory approval storage
pub struct InMemoryApprovalStore {
    pub approvals: RwLock<HashMap<String, Vec<ApprovalRecord>>>,
}

// Trait definitions

/// Workflow storage trait
pub trait WorkflowStore: Send + Sync {
    fn store_workflow(&self, workflow: &Workflow) -> BoxFuture<'_, BearDogResult<()>>;
    fn get_workflow(&self, workflow_id: &str) -> BoxFuture<'_, BearDogResult<Option<Workflow>>>;
    fn update_workflow(&self, workflow: &Workflow) -> BoxFuture<'_, BearDogResult<()>>;
    fn list_workflows(
        &self,
        status: Option<WorkflowStatus>,
    ) -> BoxFuture<'_, BearDogResult<Vec<Workflow>>>;
}

/// Approval storage trait
pub trait ApprovalStore: Send + Sync {
    fn store_approval(&self, approval: &ApprovalRecord) -> BoxFuture<'_, BearDogResult<()>>;
    fn get_approvals(&self, workflow_id: &str)
        -> BoxFuture<'_, BearDogResult<Vec<ApprovalRecord>>>;
    fn list_pending_approvals(
        &self,
        approver: &str,
    ) -> BoxFuture<'_, BearDogResult<Vec<PendingApproval>>>;
}

/// Workflow processor trait
pub trait WorkflowProcessor: Send + Sync {
    fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> BoxFuture<'_, BearDogResult<WorkflowCompletionResult>>;
    fn get_processor_name(&self) -> &'static str;
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

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            email_enabled: false,
            email_server: None,
            email_username: None,
            email_password: None,
            sms_enabled: false,
            sms_provider: None,
            sms_api_key: None,
            webhook_enabled: false,
            webhook_url: None,
            webhook_secret: None,
            slack_enabled: false,
            slack_webhook_url: None,
            teams_enabled: false,
            teams_webhook_url: None,
            notification_template_path: None,
            notification_retry_attempts: 3,
            notification_retry_delay: Duration::minutes(5),
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            default_approval_timeout: Duration::hours(24),
            emergency_approval_timeout: Duration::hours(2),
            max_concurrent_workflows: 100,
            max_workflow_age: Duration::days(30),
            auto_cleanup_enabled: true,
            risk_scoring_enabled: true,
            compliance_checking_enabled: true,
            audit_retention_period: Duration::days(365),
            notification_escalation_enabled: true,
            escalation_intervals: vec![Duration::hours(1), Duration::hours(4), Duration::hours(8)],
            delegation_policies: HashMap::new(),
            role_hierarchies: HashMap::new(),
            approval_matrix: HashMap::new(),
            emergency_contacts: Vec::new(),
            business_hours: None,
        }
    }
}

impl Default for WorkflowScheduler {
    fn default() -> Self {
        Self {
            cleanup_enabled: true,
        }
    }
}

impl MultiPartyWorkflowEngine {
    /// Create placeholder workflow engine for testing
    pub fn placeholder() -> Self {
        // Create a minimal placeholder with missing fields set to None/default
        Self {
            config: Arc::new(WorkflowConfig::default()),
            workflow_store: Arc::new(InMemoryWorkflowStore {
                workflows: RwLock::new(HashMap::new()),
            }),
            approval_store: Arc::new(InMemoryApprovalStore {
                approvals: RwLock::new(HashMap::new()),
            }),
            notification_engine: Arc::new(NotificationEngine {
                config: NotificationConfig::default(),
            }),
            policy_engine: Arc::new(WorkflowPolicyEngine {
                config: PolicyConfig::default(),
            }),
            scheduler: Arc::new(WorkflowScheduler::default()),
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
            workflow_processors: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(Mutex::new(VecDeque::new())),
            node_registry: Arc::new(RwLock::new(HashMap::new())),
            consensus_engine: Arc::new(RwLock::new(ConsensusEngine::new())),
            security_provider: Arc::new(RwLock::new(WorkflowSecurityProvider::new())),
            metrics: Arc::new(RwLock::new(WorkflowMetrics::new())),
        }
    }
}

// Add missing types for workflow engine
#[derive(Debug, Clone)]
pub struct ConsensusEngine {
    pub algorithm: String,
    pub threshold: f64,
}

impl ConsensusEngine {
    pub fn new() -> Self {
        Self {
            algorithm: "PBFT".to_string(),
            threshold: 0.67,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowSecurityProvider {
    pub policies: HashMap<String, String>,
}

impl WorkflowSecurityProvider {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }
}

impl WorkflowMetrics {
    pub fn new() -> Self {
        Self {
            active_workflows: 0,
            completed_workflows: 0,
            failed_workflows: 0,
        }
    }
}
