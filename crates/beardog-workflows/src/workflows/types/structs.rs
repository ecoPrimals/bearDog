//! Workflow struct definitions
//!
//! Contains all struct definitions for the workflows module.

use chrono::{DateTime, Duration, Utc};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use super::enums::*;
use beardog_config::integration::WorkflowConfig;
use beardog_errors::BearDogResult;

/// Core workflow structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique workflow identifier
    pub id: String,
    /// Workflow type
    pub workflow_type: WorkflowType,
    /// Current status
    pub status: WorkflowStatus,
    /// Target resource or entity
    pub target: WorkflowTarget,
    /// Required approval configuration
    pub approval_requirements: ApprovalRequirements,
    /// Workflow priority level
    pub priority: WorkflowPriority,
    /// When workflow was created
    pub created_at: DateTime<Utc>,
    /// When workflow expires (calculated from timeout_duration)
    pub expires_at: DateTime<Utc>,
    /// Requested by user ID
    pub requested_by: String,
    /// User who initiated the workflow (alias for requested_by for backward compatibility)
    pub initiator: String,
    /// Description of what this workflow does
    pub description: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Workflow timeout duration
    pub timeout_duration: Option<Duration>,
    /// Custom properties specific to workflow type
    pub properties: HashMap<String, serde_json::Value>,
    /// Approval records for this workflow
    pub approvals: Vec<ApprovalRecord>,
    /// Audit trail of all events on this workflow
    pub audit_trail: Vec<WorkflowAuditEntry>,
}

/// Approval requirements configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {
    /// List of approval tiers (evaluated in order)
    pub tiers: Vec<ApprovalTier>,
    /// Minimum number of approvals required
    pub minimum_approvals: u32,
    /// Whether all tiers must approve (vs any tier)
    pub require_all_tiers: bool,
    /// Timeout for approval process
    pub approval_timeout: Option<Duration>,
    /// Whether approvals can be delegated
    pub allow_delegation: bool,
}

/// Single approval tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalTier {
    /// Tier name/identifier
    pub name: String,
    /// Required approvers for this tier
    pub required_approvers: Vec<String>,
    /// Minimum approvals needed from this tier
    pub minimum_approvals: u32,
    /// Whether this tier can be skipped
    pub optional: bool,
}

/// Record of an approval decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    /// Unique approval ID
    pub id: String,
    /// Associated workflow ID
    pub workflow_id: String,
    /// Approver user ID
    pub approver_id: String,
    /// Approval decision
    pub decision: ApprovalDecision,
    /// Optional reason for decision
    pub reason: Option<String>,
    /// When decision was made
    pub decided_at: DateTime<Utc>,
    /// Digital signature of the approval (if required)
    pub signature: Option<String>,
    /// IP address of approver
    pub approver_ip: Option<String>,
}

/// Pending approval waiting for decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    /// Approval ID
    pub id: String,
    /// Associated workflow ID
    pub workflow_id: String,
    /// Approver user ID
    pub approver_id: String,
    /// Which tier this approval belongs to
    pub tier_name: String,
    /// When approval was requested
    pub requested_at: DateTime<Utc>,
    /// When approval expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Number of reminder notifications sent
    pub reminder_count: u32,
}

/// Audit entry for workflow events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAuditEntry {
    /// Audit entry ID
    pub id: String,
    /// Associated workflow ID
    pub workflow_id: String,
    /// Type of event that occurred
    pub event_type: String,
    /// User who triggered the event
    pub user_id: String,
    /// When event occurred
    pub timestamp: DateTime<Utc>,
    /// Event details as JSON
    pub details: serde_json::Value,
}

/// Workflow request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    /// Workflow type being requested
    pub workflow_type: WorkflowType,
    /// Target of the workflow
    pub target: WorkflowTarget,
    /// User requesting the workflow
    pub requested_by: String,
    /// User initiating the workflow (alias for requested_by for backward compatibility)
    pub initiator: String,
    /// Description of the request
    pub description: String,
    /// Request priority
    pub priority: WorkflowPriority,
    /// Custom properties for the workflow
    pub properties: HashMap<String, serde_json::Value>,
}

/// Response to workflow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {
    /// Created workflow ID
    pub workflow_id: String,
    /// Whether request was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Current workflow status
    pub status: WorkflowStatus,
    /// Any additional response data
    pub data: Option<serde_json::Value>,
}

/// Approval submission structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalSubmission {
    /// Workflow being approved/rejected
    pub workflow_id: String,
    /// Approver user ID
    pub approver_id: String,
    /// Decision being made
    pub decision: ApprovalDecision,
    /// Optional reason for decision
    pub reason: Option<String>,
    /// Digital signature (if required)
    pub signature: Option<String>,
}

/// Response to approval submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    /// Whether submission was successful
    pub success: bool,
    /// Status message
    pub message: String,
    /// Updated workflow status
    pub workflow_status: WorkflowStatus,
}

/// Workflow execution context
#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    /// Workflow being executed
    pub workflow: Workflow,
    /// Current execution state
    pub state: WorkflowExecutionState,
    /// Execution start time
    pub started_at: DateTime<Utc>,
}

/// Result of workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCompletionResult {
    /// Whether execution was successful
    pub success: bool,
    /// Completion message
    pub message: String,
    /// Any result data
    pub result_data: Option<serde_json::Value>,
    /// Execution duration in milliseconds
    pub execution_duration_ms: u64,
}

/// Workflow instance with runtime data
#[derive(Debug, Clone)]
pub struct WorkflowInstance {
    /// Base workflow definition
    pub workflow: Workflow,
    /// All approval records
    pub approvals: Vec<ApprovalRecord>,
    /// Pending approvals
    pub pending_approvals: Vec<PendingApproval>,
    /// Audit trail
    pub audit_trail: Vec<WorkflowAuditEntry>,
}

/// Metrics for a workflow instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstanceMetrics {
    /// Total time from creation to completion
    pub total_duration_ms: Option<u64>,
    /// Time spent waiting for approvals
    pub approval_duration_ms: Option<u64>,
    /// Time spent in execution
    pub execution_duration_ms: Option<u64>,
    /// Number of approvers involved
    pub approver_count: u32,
}

/// Overall workflow system metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowMetrics {
    /// Total workflows processed
    pub total_workflows: u64,
    /// Workflows currently pending
    pub pending_workflows: u64,
    /// Average processing time in milliseconds
    pub average_processing_time_ms: u64,
}

/// Notification configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Whether notifications are enabled
    pub enabled: bool,
    /// Email notification settings
    pub email: Option<EmailNotificationConfig>,
    /// SMS notification settings  
    pub sms: Option<SmsNotificationConfig>,
    /// Slack notification settings
    pub slack: Option<SlackNotificationConfig>,
    /// Webhook notification settings
    pub webhook: Option<WebhookNotificationConfig>,
    /// Teams notification settings
    pub teams: Option<TeamsNotificationConfig>,
    /// Discord notification settings
    pub discord: Option<DiscordNotificationConfig>,
    /// Custom notification settings
    pub custom: HashMap<String, serde_json::Value>,
}

/// Email notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotificationConfig {
    /// SMTP server settings
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// SMTP username
    pub username: String,
    /// From email address
    pub from_address: String,
    /// Whether to use TLS
    pub use_tls: bool,
}

/// SMS notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsNotificationConfig {
    /// SMS provider (twilio, aws-sns, etc.)
    pub provider: String,
    /// API key or credentials
    pub api_key: String,
    /// From phone number
    pub from_number: String,
}

/// Slack notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackNotificationConfig {
    /// Slack bot token
    pub bot_token: String,
    /// Default channel for notifications
    pub default_channel: String,
}

/// Webhook notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotificationConfig {
    /// Webhook URL
    pub url: String,
    /// Optional authentication headers
    pub headers: HashMap<String, String>,
    /// HTTP method to use
    pub method: String,
}

/// Teams notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsNotificationConfig {
    /// Teams webhook URL
    pub webhook_url: String,
}

/// Discord notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordNotificationConfig {
    /// Discord webhook URL
    pub webhook_url: String,
    /// Optional username override
    pub username: Option<String>,
}

/// Notification message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMessage {
    /// Message subject/title
    pub subject: String,
    /// Message body/content
    pub body: String,
    /// Priority level
    pub priority: WorkflowPriority,
}

/// Result of notification delivery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    /// Notification ID
    pub id: String,
    /// Delivery status
    pub status: NotificationStatus,
    /// Status message
    pub message: String,
    /// Timestamp of delivery attempt
    pub attempted_at: DateTime<Utc>,
    /// Number of retry attempts
    pub retry_count: u32,
}

impl NotificationResult {
    /// Create successful notification result
    pub fn success(id: String, message: String) -> Self {
        Self {
            id,
            status: NotificationStatus::Delivered,
            message,
            attempted_at: Utc::now(),
            retry_count: 0,
        }
    }

    /// Create failed notification result
    pub fn failure(id: String, error: String) -> Self {
        Self {
            id,
            status: NotificationStatus::Failed,
            message: error,
            attempted_at: Utc::now(),
            retry_count: 0,
        }
    }
}

/// Policy configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Maximum workflow timeout
    pub max_timeout_hours: u32,
    /// Default approval timeout
    pub default_approval_timeout_hours: u32,
    /// Maximum number of pending workflows per user
    pub max_pending_per_user: u32,
    /// Business hours configuration
    pub business_hours: BusinessHours,
    /// Whether emergency workflows bypass normal approval
    pub emergency_bypass_enabled: bool,
    /// Minimum approval requirements by workflow type
    pub type_requirements: HashMap<WorkflowType, u32>,
    /// Maximum priority level users can request
    pub max_user_priority: HashMap<String, WorkflowPriority>,
    /// Whether audit logging is required
    pub audit_required: bool,
    /// Retention period for workflow data (days)
    pub data_retention_days: u32,
    /// Whether digital signatures are required for approvals
    pub require_signatures: bool,
    /// Allowed IP ranges for approvals
    pub allowed_approval_ips: Vec<String>,
    /// Custom policy rules
    pub custom_rules: HashMap<String, serde_json::Value>,
}

/// Business hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    /// Start hour (24-hour format)
    pub start_hour: u32,
    /// End hour (24-hour format)
    pub end_hour: u32,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u32>,
    /// Timezone identifier
    pub timezone: String,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            email: None,
            sms: None,
            slack: None,
            webhook: None,
            teams: None,
            discord: None,
            custom: HashMap::new(),
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        let mut type_requirements = HashMap::new();
        type_requirements.insert(WorkflowType::KeyRotation, 2);
        type_requirements.insert(WorkflowType::KeyDeletion, 3);
        type_requirements.insert(WorkflowType::PolicyChange, 2);
        type_requirements.insert(WorkflowType::ConfigurationChange, 1);
        type_requirements.insert(WorkflowType::UserProvisioning, 1);
        type_requirements.insert(WorkflowType::EmergencyAccess, 1);
        type_requirements.insert(WorkflowType::SystemMaintenance, 2);
        type_requirements.insert(WorkflowType::ComplianceAudit, 2);

        Self {
            max_timeout_hours: 72,
            default_approval_timeout_hours: 24,
            max_pending_per_user: 10,
            business_hours: BusinessHours {
                start_hour: 9,
                end_hour: 17,
                days_of_week: vec![1, 2, 3, 4, 5], // Monday-Friday
                timezone: "UTC".to_string(),
            },
            emergency_bypass_enabled: true,
            type_requirements,
            max_user_priority: HashMap::new(),
            audit_required: true,
            data_retention_days: 90,
            require_signatures: false,
            allowed_approval_ips: Vec::new(),
            custom_rules: HashMap::new(),
        }
    }
}

impl WorkflowMetrics {
    /// Create a new workflow metrics instance with default values
    pub fn new() -> Self {
        Self {
            total_workflows: 0,
            pending_workflows: 0,
            average_processing_time_ms: 0,
        }
    }

    /// Update metrics with completed workflow
    pub fn update_completed(&mut self, duration_ms: u64) {
        self.total_workflows += 1;
        self.average_processing_time_ms =
            (self.average_processing_time_ms * (self.total_workflows - 1) + duration_ms)
                / self.total_workflows;
    }

    /// Increment pending workflow count
    pub fn increment_pending(&mut self) {
        self.pending_workflows += 1;
    }

    /// Decrement pending workflow count
    pub fn decrement_pending(&mut self) {
        if self.pending_workflows > 0 {
            self.pending_workflows -= 1;
        }
    }
}

/// Multi-party workflow engine for secure collaborative operations
#[derive(Clone)]
pub struct MultiPartyWorkflowEngine {
    /// Configuration for the workflow engine
    pub config: Arc<WorkflowConfig>,
    /// Storage backend for workflow data
    pub workflow_store: Arc<dyn WorkflowStore>,
    /// Storage backend for approval data
    pub approval_store: Arc<dyn ApprovalStore>,
    /// Engine for sending notifications
    pub notification_engine: Arc<NotificationEngine>,
    /// Policy engine for workflow rules
    pub policy_engine: Arc<WorkflowPolicyEngine>,
    /// Scheduler for workflow management
    pub scheduler: Arc<WorkflowScheduler>,

    /// Currently active workflows
    pub active_workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    /// Pending approvals for workflows
    pub pending_approvals: Arc<RwLock<HashMap<String, Vec<PendingApproval>>>>,

    /// Workflow processors for different workflow types
    pub workflow_processors: Arc<RwLock<HashMap<WorkflowType, Box<dyn WorkflowProcessor>>>>,
    /// Queue for workflow executions
    pub execution_queue: Arc<Mutex<VecDeque<WorkflowExecution>>>,

    /// Registry of participating nodes
    pub node_registry: Arc<RwLock<HashMap<String, String>>>,
    /// Consensus engine for multi-party coordination
    pub consensus_engine: Arc<RwLock<ConsensusEngine>>,
    /// Security provider for workflow security
    pub security_provider: Arc<RwLock<WorkflowSecurityProvider>>,
    /// Metrics for workflow performance
    pub metrics: Arc<RwLock<WorkflowMetrics>>,
}

/// Notification engine for workflow events
#[derive(Debug, Clone)]
pub struct NotificationEngine {
    /// Configuration for notifications
    pub config: NotificationConfig,
}

/// Workflow policy engine
pub struct WorkflowPolicyEngine {
    /// Configuration for workflow policies
    pub config: PolicyConfig,
}

/// Workflow scheduler
pub struct WorkflowScheduler {
    /// Whether automatic cleanup of old workflows is enabled
    pub cleanup_enabled: bool,
    /// Reference to active workflows for cleanup operations
    pub workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    /// Policy configuration for cleanup thresholds
    pub policy_config: Arc<PolicyConfig>,
    /// Background task handle for cleanup operations (with interior mutability)
    pub cleanup_task_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

/// In-memory workflow storage
pub struct InMemoryWorkflowStore {
    /// In-memory storage for workflows
    pub workflows: RwLock<HashMap<String, Workflow>>,
}

/// In-memory approval storage
pub struct InMemoryApprovalStore {
    /// In-memory storage for approval records
    pub approvals: RwLock<HashMap<String, Vec<ApprovalRecord>>>,
}

/// Consensus engine for distributed workflow decisions
#[derive(Debug, Clone)]
pub struct ConsensusEngine {
    /// Consensus algorithm used (e.g., PBFT, Raft)
    pub algorithm: String,
    /// Threshold for consensus agreement (0.0 to 1.0)
    pub threshold: f64,
}

impl Default for ConsensusEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsensusEngine {
    /// Create a new consensus engine with default settings
    pub fn new() -> Self {
        Self {
            algorithm: "PBFT".to_string(),
            threshold: 0.67,
        }
    }
}

/// Security provider for workflow access control and validation
#[derive(Debug, Clone)]
pub struct WorkflowSecurityProvider {
    /// Security policies for workflow operations
    pub policies: HashMap<String, String>,
}

impl Default for WorkflowSecurityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowSecurityProvider {
    /// Create a new workflow security provider with default settings
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }
}

impl Default for WorkflowScheduler {
    fn default() -> Self {
        Self {
            cleanup_enabled: true,
            workflows: Arc::new(RwLock::new(HashMap::new())),
            policy_config: Arc::new(PolicyConfig::default()),
            cleanup_task_handle: Arc::new(RwLock::new(None)),
        }
    }
}

impl WorkflowScheduler {
    /// Create a new workflow scheduler with specific workflow storage and policy
    pub fn new(
        workflows: Arc<RwLock<HashMap<String, Workflow>>>,
        policy_config: Arc<PolicyConfig>,
    ) -> Self {
        Self {
            cleanup_enabled: true,
            workflows,
            policy_config,
            cleanup_task_handle: Arc::new(RwLock::new(None)),
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
            metrics: Arc::new(RwLock::new(WorkflowMetrics::default())),
        }
    }
}

/// Workflow storage trait
pub trait WorkflowStore: Send + Sync {
    /// Store a workflow in the storage backend
    fn store_workflow(&self, workflow: &Workflow) -> BoxFuture<'_, BearDogResult<()>>;
    /// Retrieve a workflow by its ID from storage
    fn get_workflow(&self, workflow_id: &str) -> BoxFuture<'_, BearDogResult<Option<Workflow>>>;
    /// Update an existing workflow in storage
    fn update_workflow(&self, workflow: &Workflow) -> BoxFuture<'_, BearDogResult<()>>;
    /// List workflows, optionally filtered by status
    fn list_workflows(
        &self,
        status: Option<WorkflowStatus>,
    ) -> BoxFuture<'_, BearDogResult<Vec<Workflow>>>;
}

/// Approval storage trait
pub trait ApprovalStore: Send + Sync {
    /// Store an approval record in the storage backend
    fn store_approval(&self, approval: &ApprovalRecord) -> BoxFuture<'_, BearDogResult<()>>;
    /// Get all approval records for a specific workflow
    fn get_approvals(&self, workflow_id: &str)
        -> BoxFuture<'_, BearDogResult<Vec<ApprovalRecord>>>;
    /// List all pending approvals that need attention
    fn list_pending_approvals(
        &self,
        approver: &str,
    ) -> BoxFuture<'_, BearDogResult<Vec<PendingApproval>>>;
}

/// Workflow processor trait
pub trait WorkflowProcessor: Send + Sync {
    /// Process a workflow and return completion results
    fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> BoxFuture<'_, BearDogResult<WorkflowCompletionResult>>;
    /// Get the name of this processor
    fn get_processor_name(&self) -> &'static str;
}
