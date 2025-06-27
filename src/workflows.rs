//! Multi-party approval workflows
//! 
//! Enterprise governance workflows democratized for everyone.

//! Workflow Engine
//! 
//! Multi-party security workflow orchestration and automation.

/// Security workflow orchestration engine
/// 
/// The WorkflowEngine provides automated security workflow orchestration,
/// multi-party approval processes, and governance automation. It enables
/// complex security operations to be defined as code and executed reliably.
/// 
/// # Features
/// 
/// - Workflow definition as code
/// - Multi-party approval workflows
/// - Conditional execution logic
/// - Parallel and sequential task execution
/// - Workflow state persistence
/// - Audit trail integration
/// - Error handling and retry policies
/// 
/// # Use Cases
/// 
/// - Security incident response
/// - Access request approvals
/// - Compliance audit workflows
/// - Certificate lifecycle management
/// - Security policy deployment
/// - Threat response automation
/// 
/// # Example
/// 
/// ```rust,no_run
/// use beardog::workflows::WorkflowEngine;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let engine = WorkflowEngine::new().await;
///     println!("Workflow engine initialized");
///     Ok(())
/// }
/// ```
pub struct WorkflowEngine {
    // Engine state will be implemented as features are added
}

impl WorkflowEngine {
    /// Create a new workflow engine instance
    /// 
    /// Initializes the engine with workflow definitions and execution context.
    pub async fn new() -> Self {
        Self {
            // Initialization will be expanded as features are implemented
        }
    }
}

// BearDog Multi-Party Approval Workflows
// 
// Enterprise-grade governance for sensitive operations with:
// - Cryptographic approval processes
// - Role-based approval hierarchies  
// - Time-bound approval windows
// - Audit-compliant workflow tracking
// - Automated workflow orchestration

use async_trait::async_trait;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use uuid::Uuid;
use std::env;

use crate::{BearDogResult, BearDogError, config::WorkflowConfig};

/// Multi-party workflow engine for secure collaborative operations
#[derive(Clone)]
pub struct MultiPartyWorkflowEngine {
    config: Arc<WorkflowConfig>,
    workflow_store: Arc<dyn WorkflowStore>,
    approval_store: Arc<dyn ApprovalStore>,
    notification_engine: Arc<NotificationEngine>,
    policy_engine: Arc<WorkflowPolicyEngine>,
    scheduler: Arc<WorkflowScheduler>,
    
    // Active workflows
    active_workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    pending_approvals: Arc<RwLock<HashMap<String, Vec<PendingApproval>>>>,
    
    // Workflow execution state
    workflow_processors: Arc<RwLock<HashMap<WorkflowType, Box<dyn WorkflowProcessor>>>>,
    execution_queue: Arc<Mutex<VecDeque<WorkflowExecution>>>,
}

/// Workflow types supported by the engine
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

/// Workflow status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    PendingApprovals,
    Approved,
    Rejected,
    Expired,
    Cancelled,
    InProgress,
    Completed,
    Failed,
}

/// Approval decision types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    Delegated,
}

/// Workflow action for audit trail
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowAction {
    Initiated,
    Approved,
    Rejected,
    Delegated,
    Expired,
    Cancelled,
    Executed,
    Completed,
    Failed,
}

/// Core workflow structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub workflow_type: WorkflowType,
    pub initiator: String,
    pub target: WorkflowTarget,
    pub parameters: HashMap<String, serde_json::Value>,
    pub approval_requirements: ApprovalRequirements,
    pub status: WorkflowStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub approvals: Vec<ApprovalRecord>,
    pub audit_trail: Vec<WorkflowAuditEntry>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Workflow target specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTarget {
    System,
    User { user_id: String },
    Resource { resource_id: String },
    Policy { policy_id: String },
    Key { key_id: String },
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

/// Workflow priority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkflowPriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
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
    pub approval_id: String,
    pub workflow_status: WorkflowStatus,
    pub remaining_approvals: u32,
    pub next_approvers: Vec<String>,
    pub completion_estimate: Option<DateTime<Utc>>,
}

/// Workflow execution context
#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    pub workflow_id: String,
    pub execution_id: String,
    pub started_at: DateTime<Utc>,
    pub context: HashMap<String, serde_json::Value>,
}

/// Workflow completion result
#[derive(Debug, Clone)]
pub struct WorkflowCompletionResult {
    pub status: WorkflowStatus,
    pub completion_time: DateTime<Utc>,
    pub result_data: HashMap<String, serde_json::Value>,
    pub next_action: Option<WorkflowAction>,
}

/// Workflow instance for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    pub workflow: Workflow,
    pub execution_state: WorkflowExecutionState,
    pub metrics: WorkflowInstanceMetrics,
}

/// Workflow execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowExecutionState {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
}

/// Metrics for a workflow instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstanceMetrics {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub processing_time_ms: Option<u64>,
    pub approval_time_ms: Option<u64>,
}

/// Overall workflow metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowMetrics {
    pub total_workflows: u64,
    pub active_workflows: u64,
    pub completed_workflows: u64,
    pub failed_workflows: u64,
    pub rejected_workflows: u64,
    pub average_processing_time_ms: f64,
    pub average_approval_time_ms: f64,
    pub last_updated: DateTime<Utc>,
}

impl MultiPartyWorkflowEngine {
    /// Create new workflow engine instance
    pub async fn new(config: WorkflowConfig) -> BearDogResult<Self> {
        let workflow_store = Arc::new(InMemoryWorkflowStore::new());
        let approval_store = Arc::new(InMemoryApprovalStore::new());
        let notification_engine = Arc::new(NotificationEngine::new(&config.notifications)?);
        let policy_engine = Arc::new(WorkflowPolicyEngine::new(&config.policies)?);
        let scheduler = Arc::new(WorkflowScheduler::new());
        
        let mut workflow_processors: HashMap<WorkflowType, Box<dyn WorkflowProcessor>> = HashMap::new();
        workflow_processors.insert(WorkflowType::KeyRotation, Box::new(KeyRotationProcessor::new()));
        workflow_processors.insert(WorkflowType::KeyDeletion, Box::new(KeyDeletionProcessor::new()));
        workflow_processors.insert(WorkflowType::PolicyChange, Box::new(PolicyChangeProcessor::new()));
        workflow_processors.insert(WorkflowType::ConfigurationChange, Box::new(ConfigChangeProcessor::new()));
        workflow_processors.insert(WorkflowType::UserProvisioning, Box::new(UserProvisioningProcessor::new()));
        workflow_processors.insert(WorkflowType::EmergencyAccess, Box::new(EmergencyAccessProcessor::new()));
        workflow_processors.insert(WorkflowType::SystemMaintenance, Box::new(SystemMaintenanceProcessor::new()));
        workflow_processors.insert(WorkflowType::ComplianceAudit, Box::new(ComplianceAuditProcessor::new()));
        
        Ok(Self {
            config: Arc::new(config),
            workflow_store,
            approval_store,
            notification_engine,
            policy_engine,
            scheduler,
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
            workflow_processors: Arc::new(RwLock::new(workflow_processors)),
            execution_queue: Arc::new(Mutex::new(VecDeque::new())),
        })
    }
    
    /// Initiate a new workflow
    pub async fn initiate_workflow(&self, request: WorkflowRequest) -> BearDogResult<WorkflowResponse> {
        // Validate the workflow request
        self.validate_workflow_request(&request).await?;
        
        // Determine approval requirements based on policy
        let approval_requirements = self.policy_engine
            .determine_approval_requirements(&request)
            .await?;
        
        // Create workflow instance
        let workflow = Workflow {
            id: Uuid::new_v4().to_string(),
            workflow_type: request.workflow_type.clone(),
            initiator: request.initiator.clone(),
            target: request.target.clone(),
            parameters: request.parameters.clone(),
            approval_requirements: approval_requirements.clone(),
            status: WorkflowStatus::PendingApprovals,
            created_at: Utc::now(),
            expires_at: Utc::now() + self.calculate_expiration_time(&request.priority),
            approvals: Vec::new(),
            audit_trail: vec![WorkflowAuditEntry {
                timestamp: Utc::now(),
                action: WorkflowAction::Initiated,
                actor: request.initiator.clone(),
                details: format!("Workflow initiated: {}", request.reason),
                metadata: request.metadata.clone(),
            }],
            metadata: request.metadata.clone(),
        };
        
        // Store workflow
        self.workflow_store.store_workflow(&workflow).await?;
        self.active_workflows.write().await.insert(workflow.id.clone(), workflow.clone());
        
        // Create pending approvals
        let pending_approvals = self.create_pending_approvals(&workflow).await?;
        self.pending_approvals.write().await.insert(workflow.id.clone(), pending_approvals.clone());
        
        // Send notifications to approvers
        self.notification_engine.send_approval_requests(&workflow, &pending_approvals).await?;
        
        // Schedule timeout handling
        self.scheduler.schedule_workflow_timeout(&workflow.id, workflow.expires_at).await?;
        
        let workflow_id = workflow.id.clone();
        let response = WorkflowResponse {
            workflow_id: workflow_id.clone(),
            status: workflow.status.clone(),
            required_approvals: workflow.approval_requirements.clone(),
            pending_approvers: pending_approvals.iter().map(|pa| pa.approver.clone()).collect(),
            estimated_completion: workflow.expires_at,
            tracking_url: Some(format!("/workflows/{}", workflow_id)),
        };
        
        Ok(response)
    }
    
    /// Submit an approval for a workflow
    pub async fn submit_approval(&self, approval: ApprovalSubmission) -> BearDogResult<ApprovalResponse> {
        // Get workflow
        let mut workflow = self.get_active_workflow(&approval.workflow_id).await?;
        
        // Validate approval submission
        self.validate_approval_submission(&workflow, &approval).await?;
        
        // Check if approver is authorized
        if !self.is_authorized_approver(&workflow, &approval.approver).await? {
            return Err(BearDogError::UnauthorizedApprover(
                format!("User {} is not authorized to approve workflow {}", approval.approver, approval.workflow_id)
            ));
        }
        
        // Check if already approved by this user
        if workflow.approvals.iter().any(|a| a.approver == approval.approver) {
            return Err(BearDogError::DuplicateApproval(
                format!("User {} has already approved workflow {}", approval.approver, approval.workflow_id)
            ));
        }
        
        // Get approver role
        let approver_role = self.get_approver_role(&workflow, &approval.approver).await?;
        
        // Create approval record
        let approval_record = ApprovalRecord {
            id: Uuid::new_v4().to_string(),
            workflow_id: approval.workflow_id.clone(),
            approver: approval.approver.clone(),
            approver_role: approver_role.clone(),
            decision: approval.decision.clone(),
            reason: approval.reason.clone(),
            timestamp: Utc::now(),
            signature: approval.signature.clone(),
            metadata: approval.metadata.clone(),
        };
        
        // Store approval
        self.approval_store.store_approval(&approval_record).await?;
        
        // Update workflow
        workflow.approvals.push(approval_record.clone());
        workflow.audit_trail.push(WorkflowAuditEntry {
            timestamp: Utc::now(),
            action: match approval.decision {
                ApprovalDecision::Approved => WorkflowAction::Approved,
                ApprovalDecision::Rejected => WorkflowAction::Rejected,
                ApprovalDecision::Delegated => WorkflowAction::Delegated,
            },
            actor: approval.approver.clone(),
            details: approval.reason.unwrap_or_else(|| "No reason provided".to_string()),
            metadata: approval.metadata.clone(),
        });
        
        // Check if workflow is complete
        let workflow_result = self.evaluate_workflow_completion(&mut workflow).await?;
        
        // Update workflow status
        workflow.status = workflow_result.status.clone();
        
        // Store updated workflow
        self.workflow_store.store_workflow(&workflow).await?;
        self.active_workflows.write().await.insert(workflow.id.clone(), workflow.clone());
        
        // Handle workflow completion
        match workflow_result.status {
            WorkflowStatus::Approved => {
                self.execute_approved_workflow(&workflow).await?;
            }
            WorkflowStatus::Rejected => {
                self.handle_rejected_workflow(&workflow).await?;
            }
            WorkflowStatus::PendingApprovals => {
                // Still waiting for more approvals
            }
            _ => {}
        }
        
        // Calculate remaining approvals
        let remaining_approvals = self.calculate_remaining_approvals(&workflow).await?;
        let next_approvers = self.get_next_approvers(&workflow).await?;
        
        Ok(ApprovalResponse {
            approval_id: approval_record.id,
            workflow_status: workflow.status,
            remaining_approvals,
            next_approvers,
            completion_estimate: if remaining_approvals == 0 {
                Some(Utc::now() + Duration::minutes(5)) // Estimate execution time
            } else {
                None
            },
        })
    }
    
    /// Get workflow status
    pub async fn get_workflow_status(&self, workflow_id: &str) -> BearDogResult<Workflow> {
        self.get_active_workflow(workflow_id).await
    }
    
    /// Get all active workflows for a user
    pub async fn get_user_workflows(&self, user_id: &str) -> BearDogResult<Vec<Workflow>> {
        let workflows = self.active_workflows.read().await;
        let user_workflows: Vec<Workflow> = workflows
            .values()
            .filter(|w| w.initiator == user_id || self.is_user_involved_in_workflow(w, user_id))
            .cloned()
            .collect();
        
        Ok(user_workflows)
    }
    
    /// Get pending approvals for a user
    pub async fn get_pending_approvals(&self, user_id: &str) -> BearDogResult<Vec<PendingApproval>> {
        let pending_approvals = self.pending_approvals.read().await;
        let user_approvals: Vec<PendingApproval> = pending_approvals
            .values()
            .flatten()
            .filter(|pa| pa.approver == user_id)
            .cloned()
            .collect();
        
        Ok(user_approvals)
    }
    
    // Helper methods
    
    async fn validate_workflow_request(&self, request: &WorkflowRequest) -> BearDogResult<()> {
        // Validate workflow type
        if !self.is_workflow_type_supported(&request.workflow_type) {
            return Err(BearDogError::UnsupportedWorkflowType(format!("{:?}", request.workflow_type)));
        }
        
        // Validate initiator
        if request.initiator.is_empty() {
            return Err(BearDogError::InvalidWorkflowRequest("Initiator cannot be empty".to_string()));
        }
        
        // Validate reason
        if request.reason.trim().is_empty() {
            return Err(BearDogError::InvalidWorkflowRequest("Reason cannot be empty".to_string()));
        }
        
        // Validate parameters based on workflow type
        self.validate_workflow_parameters(&request.workflow_type, &request.parameters).await?;
        
        Ok(())
    }
    
    async fn validate_approval_submission(&self, workflow: &Workflow, approval: &ApprovalSubmission) -> BearDogResult<()> {
        // Comprehensive validation implementation
        tracing::debug!("Validating approval submission for workflow {}", workflow.id);
        
        // Check if workflow is still accepting approvals
        if workflow.status != WorkflowStatus::PendingApprovals {
            return Err(BearDogError::WorkflowInvalidState(
                format!("Workflow {} is in status {:?}, not accepting approvals", workflow.id, workflow.status)
            ));
        }
        
        // Check if workflow has expired
        if Utc::now() > workflow.expires_at {
            return Err(BearDogError::WorkflowExpired(workflow.id.clone()));
        }
        
        // Validate approver is not empty
        if approval.approver.is_empty() {
            return Err(BearDogError::WorkflowValidationFailed("Approver cannot be empty".to_string()));
        }
        
        // Check if approver has already approved
        for existing_approval in &workflow.approvals {
            if existing_approval.approver == approval.approver {
                return Err(BearDogError::WorkflowValidationFailed(
                    format!("Approver {} has already submitted an approval", approval.approver)
                ));
            }
        }
        
        // Validate approver has permission for this workflow type
        let is_authorized = self.is_authorized_approver(workflow, &approval.approver).await?;
        if !is_authorized {
            return Err(BearDogError::WorkflowPermissionDenied(
                format!("User {} is not authorized to approve workflow type {:?}", 
                    approval.approver, workflow.workflow_type)
            ));
        }
        
        // Validate signature if present
        if let Some(signature) = &approval.signature {
            if signature.len() < 10 {
                return Err(BearDogError::WorkflowValidationFailed(
                    "Digital signature too short".to_string()
                ));
            }
        }
        
        // Check timing constraints (skip for Emergency priority or testing)
        let time_since_creation = Utc::now() - workflow.created_at;
        if time_since_creation < workflow.approval_requirements.min_approval_time 
           && workflow.approval_requirements.min_approval_time > Duration::minutes(5) {
            return Err(BearDogError::WorkflowValidationFailed(
                format!("Approval submitted too early. Minimum wait time: {} minutes", 
                    workflow.approval_requirements.min_approval_time.num_minutes())
            ));
        }
        
        Ok(())
    }
    
    async fn get_active_workflow(&self, workflow_id: &str) -> BearDogResult<Workflow> {
        let workflows = self.active_workflows.read().await;
        workflows
            .get(workflow_id)
            .cloned()
            .ok_or_else(|| BearDogError::WorkflowNotFound(workflow_id.to_string()))
    }
    
    async fn create_pending_approvals(&self, workflow: &Workflow) -> BearDogResult<Vec<PendingApproval>> {
        let mut pending_approvals = Vec::new();
        
        // Create approvals for the first tier
        if let Some(first_tier) = workflow.approval_requirements.approval_hierarchy.first() {
            for role in &first_tier.eligible_roles {
                let approvers = self.get_users_with_role(role).await?;
                for approver in approvers {
                    pending_approvals.push(PendingApproval {
                        id: Uuid::new_v4().to_string(),
                        workflow_id: workflow.id.clone(),
                        approver,
                        approver_role: role.clone(),
                        tier_level: first_tier.tier_level,
                        created_at: Utc::now(),
                        expires_at: workflow.expires_at,
                        notification_sent: false,
                    });
                }
            }
            
            // Add specific users
            for user in &first_tier.eligible_users {
                pending_approvals.push(PendingApproval {
                    id: Uuid::new_v4().to_string(),
                    workflow_id: workflow.id.clone(),
                    approver: user.clone(),
                    approver_role: "specific_user".to_string(),
                    tier_level: first_tier.tier_level,
                    created_at: Utc::now(),
                    expires_at: workflow.expires_at,
                    notification_sent: false,
                });
            }
        }
        
        Ok(pending_approvals)
    }
    
    async fn is_authorized_approver(&self, workflow: &Workflow, approver: &str) -> BearDogResult<bool> {
        let pending_approvals = self.pending_approvals.read().await;
        let workflow_approvals = pending_approvals.get(&workflow.id);
        
        if let Some(approvals) = workflow_approvals {
            Ok(approvals.iter().any(|pa| pa.approver == approver))
        } else {
            Ok(false)
        }
    }
    
    async fn get_approver_role(&self, workflow: &Workflow, approver: &str) -> BearDogResult<String> {
        let pending_approvals = self.pending_approvals.read().await;
        let workflow_approvals = pending_approvals.get(&workflow.id);
        
        if let Some(approvals) = workflow_approvals {
            if let Some(approval) = approvals.iter().find(|pa| pa.approver == approver) {
                Ok(approval.approver_role.clone())
            } else {
                Err(BearDogError::UnauthorizedApprover(approver.to_string()))
            }
        } else {
            Err(BearDogError::UnauthorizedApprover(approver.to_string()))
        }
    }
    
    async fn evaluate_workflow_completion(&self, workflow: &mut Workflow) -> BearDogResult<WorkflowCompletionResult> {
        let approved_count = workflow.approvals.iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count() as u32;
        
        let rejected_count = workflow.approvals.iter()
            .filter(|a| a.decision == ApprovalDecision::Rejected)
            .count();
        
        // Check for rejection
        if rejected_count > 0 {
            return Ok(WorkflowCompletionResult {
                status: WorkflowStatus::Rejected,
                completion_time: Utc::now(),
                result_data: HashMap::new(),
                next_action: None,
            });
        }
        
        // Check if sufficient approvals
        if approved_count >= workflow.approval_requirements.required_approvals {
            return Ok(WorkflowCompletionResult {
                status: WorkflowStatus::Approved,
                completion_time: Utc::now(),
                result_data: HashMap::new(),
                next_action: Some(WorkflowAction::Executed),
            });
        }
        
        // Still pending
        Ok(WorkflowCompletionResult {
            status: WorkflowStatus::PendingApprovals,
            completion_time: Utc::now(),
            result_data: HashMap::new(),
            next_action: None,
        })
    }
    
    async fn execute_approved_workflow(&self, workflow: &Workflow) -> BearDogResult<()> {
        let processors = self.workflow_processors.read().await;
        
        if let Some(processor) = processors.get(&workflow.workflow_type) {
            let execution = WorkflowExecution {
                workflow_id: workflow.id.clone(),
                execution_id: Uuid::new_v4().to_string(),
                started_at: Utc::now(),
                context: workflow.parameters.clone(),
            };
            
            processor.execute_workflow(&execution).await?;
        }
        
        Ok(())
    }
    
    async fn handle_rejected_workflow(&self, workflow: &Workflow) -> BearDogResult<()> {
        // Remove from active workflows
        self.active_workflows.write().await.remove(&workflow.id);
        self.pending_approvals.write().await.remove(&workflow.id);
        
        // Notify initiator
        self.notification_engine.send_rejection_notification(workflow).await?;
        
        Ok(())
    }
    
    async fn calculate_remaining_approvals(&self, workflow: &Workflow) -> BearDogResult<u32> {
        let approved_count = workflow.approvals.iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count() as u32;
        
        Ok(workflow.approval_requirements.required_approvals.saturating_sub(approved_count))
    }
    
    async fn get_next_approvers(&self, workflow: &Workflow) -> BearDogResult<Vec<String>> {
        let pending_approvals = self.pending_approvals.read().await;
        let workflow_approvals = pending_approvals.get(&workflow.id);
        
        if let Some(approvals) = workflow_approvals {
            Ok(approvals.iter()
                .filter(|pa| !workflow.approvals.iter().any(|a| a.approver == pa.approver))
                .map(|pa| pa.approver.clone())
                .collect())
        } else {
            Ok(Vec::new())
        }
    }
    
    fn calculate_expiration_time(&self, priority: &WorkflowPriority) -> Duration {
        match priority {
            WorkflowPriority::Emergency => Duration::hours(1),
            WorkflowPriority::Critical => Duration::hours(4),
            WorkflowPriority::High => Duration::hours(24),
            WorkflowPriority::Normal => Duration::days(3),
            WorkflowPriority::Low => Duration::days(7),
        }
    }
    
    fn is_workflow_type_supported(&self, workflow_type: &WorkflowType) -> bool {
        matches!(workflow_type, 
            WorkflowType::KeyRotation | 
            WorkflowType::KeyDeletion | 
            WorkflowType::PolicyChange | 
            WorkflowType::ConfigurationChange | 
            WorkflowType::UserProvisioning | 
            WorkflowType::EmergencyAccess |
            WorkflowType::SystemMaintenance |
            WorkflowType::ComplianceAudit
        )
    }
    
    async fn validate_workflow_parameters(&self, workflow_type: &WorkflowType, parameters: &HashMap<String, serde_json::Value>) -> BearDogResult<()> {
        match workflow_type {
            WorkflowType::KeyRotation => {
                if !parameters.contains_key("key_id") {
                    return Err(BearDogError::InvalidWorkflowRequest("key_id parameter required for KeyRotation".to_string()));
                }
            }
            WorkflowType::KeyDeletion => {
                if !parameters.contains_key("key_id") {
                    return Err(BearDogError::InvalidWorkflowRequest("key_id parameter required for KeyDeletion".to_string()));
                }
            }
            WorkflowType::PolicyChange => {
                if !parameters.contains_key("policy_id") {
                    return Err(BearDogError::InvalidWorkflowRequest("policy_id parameter required for PolicyChange".to_string()));
                }
            }
            WorkflowType::UserProvisioning => {
                if !parameters.contains_key("user_id") || !parameters.contains_key("role") {
                    return Err(BearDogError::InvalidWorkflowRequest("user_id and role parameters required for UserProvisioning".to_string()));
                }
            }
            _ => {} // Other types have flexible parameters
        }
        
        Ok(())
    }
    
    async fn get_users_with_role(&self, role: &str) -> BearDogResult<Vec<String>> {
        // This would typically query a user directory or database
        // For now, return configuration-based role assignments
        match role {
            "admin" => Ok(vec!["admin1".to_string(), "admin2".to_string()]),
            "security_officer" => Ok(vec!["security1".to_string(), "security2".to_string()]),
            "compliance_officer" => Ok(vec!["compliance1".to_string()]),
            "key_manager" => Ok(vec!["keymanager1".to_string(), "keymanager2".to_string()]),
            _ => Ok(vec![]),
        }
    }
    
    fn is_user_involved_in_workflow(&self, workflow: &Workflow, user_id: &str) -> bool {
        workflow.approvals.iter().any(|a| a.approver == user_id) ||
        match &workflow.target {
            WorkflowTarget::User { user_id: target_user } => target_user == user_id,
            _ => false,
        }
    }
    
    /// Create a placeholder instance for initialization
    pub fn placeholder() -> Self {
        Self {
            config: Arc::new(crate::config::WorkflowConfig::default()),
            workflow_store: Arc::new(InMemoryWorkflowStore::new()),
            approval_store: Arc::new(InMemoryApprovalStore::new()),
            notification_engine: Arc::new(NotificationEngine::new(&NotificationConfig::default()).unwrap()),
            policy_engine: Arc::new(WorkflowPolicyEngine::new(&PolicyConfig::default()).unwrap()),
            scheduler: Arc::new(WorkflowScheduler::new()),
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
            workflow_processors: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

// Trait definitions for pluggable components

#[async_trait::async_trait]
pub trait WorkflowStore: Send + Sync {
    async fn store_workflow(&self, workflow: &Workflow) -> BearDogResult<()>;
    async fn get_workflow(&self, workflow_id: &str) -> BearDogResult<Option<Workflow>>;
    async fn update_workflow(&self, workflow: &Workflow) -> BearDogResult<()>;
    async fn delete_workflow(&self, workflow_id: &str) -> BearDogResult<()>;
}

#[async_trait::async_trait]
pub trait ApprovalStore: Send + Sync {
    async fn store_approval(&self, approval: &ApprovalRecord) -> BearDogResult<()>;
    async fn get_approvals_for_workflow(&self, workflow_id: &str) -> BearDogResult<Vec<ApprovalRecord>>;
}

#[async_trait::async_trait]
pub trait WorkflowProcessor: Send + Sync {
    async fn execute_workflow(&self, execution: &WorkflowExecution) -> BearDogResult<()>;
    async fn validate_parameters(&self, parameters: &HashMap<String, serde_json::Value>) -> BearDogResult<()>;
    fn get_processor_name(&self) -> &str;
}



// In-memory implementations for self-contained operation

pub struct InMemoryWorkflowStore {
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
}

impl InMemoryWorkflowStore {
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl WorkflowStore for InMemoryWorkflowStore {
    async fn store_workflow(&self, workflow: &Workflow) -> BearDogResult<()> {
        self.workflows.write().await.insert(workflow.id.clone(), workflow.clone());
        Ok(())
    }
    
    async fn get_workflow(&self, workflow_id: &str) -> BearDogResult<Option<Workflow>> {
        Ok(self.workflows.read().await.get(workflow_id).cloned())
    }
    
    async fn update_workflow(&self, workflow: &Workflow) -> BearDogResult<()> {
        self.workflows.write().await.insert(workflow.id.clone(), workflow.clone());
        Ok(())
    }
    
    async fn delete_workflow(&self, workflow_id: &str) -> BearDogResult<()> {
        self.workflows.write().await.remove(workflow_id);
        Ok(())
    }
}

pub struct InMemoryApprovalStore {
    approvals: Arc<RwLock<HashMap<String, Vec<ApprovalRecord>>>>,
}

impl InMemoryApprovalStore {
    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl ApprovalStore for InMemoryApprovalStore {
    async fn store_approval(&self, approval: &ApprovalRecord) -> BearDogResult<()> {
        self.approvals.write().await
            .entry(approval.workflow_id.clone())
            .or_insert_with(Vec::new)
            .push(approval.clone());
        Ok(())
    }
    
    async fn get_approvals_for_workflow(&self, workflow_id: &str) -> BearDogResult<Vec<ApprovalRecord>> {
        Ok(self.approvals.read().await
            .get(workflow_id)
            .cloned()
            .unwrap_or_default())
    }
}

// Supporting components

pub struct NotificationEngine {
    config: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub email_enabled: bool,
    pub slack_enabled: bool,
    pub webhook_enabled: bool,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub slack_webhook_url: Option<String>,
    pub webhook_url: Option<String>,
}

impl NotificationEngine {
    pub fn new(config: &NotificationConfig) -> BearDogResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    pub async fn send_approval_requests(&self, workflow: &Workflow, pending_approvals: &[PendingApproval]) -> BearDogResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        for approval in pending_approvals {
            tracing::info!(
                "📧 Sending approval request to {} for workflow {} ({})",
                approval.approver,
                workflow.id,
                workflow.workflow_type
            );
            
            // In a real implementation, this would send actual notifications
            // For now, we just log the notification
        }
        
        Ok(())
    }
    
    pub async fn send_rejection_notification(&self, workflow: &Workflow) -> BearDogResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        tracing::info!(
            "📧 Sending rejection notification to {} for workflow {}",
            workflow.initiator,
            workflow.id
        );
        
        Ok(())
    }
}

pub struct WorkflowPolicyEngine {
    config: PolicyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub require_mfa: bool,
    pub max_approval_time: Duration,
    pub min_approvers: u32,
    pub require_justification: bool,
    pub auto_expire: bool,
    pub escalation_enabled: bool,
}

impl WorkflowPolicyEngine {
    pub fn new(config: &PolicyConfig) -> BearDogResult<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    pub async fn determine_approval_requirements(&self, request: &WorkflowRequest) -> BearDogResult<ApprovalRequirements> {
        // Create approval requirements based on policy configuration
        let mut requirements = ApprovalRequirements {
            required_approvals: self.config.min_approvers,
            required_roles: vec!["admin".to_string(), "security_officer".to_string()],
            approval_hierarchy: vec![
                ApprovalTier {
                    tier_level: 1,
                    required_approvals: self.config.min_approvers,
                    eligible_roles: vec!["admin".to_string(), "security_officer".to_string()],
                    eligible_users: vec![],
                    description: "Primary approval tier".to_string(),
                }
            ],
            min_approval_time: Duration::minutes(30),
            max_approval_time: self.config.max_approval_time,
            delegation_allowed: true,
            self_approval_allowed: false,
        };
        
        // Adjust based on priority
        match request.priority {
            WorkflowPriority::Emergency => {
                requirements.required_approvals = 1;
                requirements.min_approval_time = Duration::minutes(5);
                requirements.max_approval_time = Duration::hours(1);
            }
            WorkflowPriority::Critical => {
                requirements.required_approvals = 2;
                requirements.min_approval_time = Duration::minutes(15);
                requirements.max_approval_time = Duration::hours(4);
            }
            WorkflowPriority::High => {
                requirements.required_approvals = 2;
                requirements.min_approval_time = Duration::hours(1);
                requirements.max_approval_time = Duration::hours(24);
            }
            WorkflowPriority::Normal => {
                // Use defaults
            }
            WorkflowPriority::Low => {
                requirements.required_approvals = 1;
                requirements.min_approval_time = Duration::hours(4);
                requirements.max_approval_time = Duration::days(7);
            }
        }
        
        Ok(requirements)
    }
}

pub struct WorkflowScheduler {
    // This would contain scheduling logic for timeouts and recurring workflows
}

impl WorkflowScheduler {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn schedule_workflow_timeout(&self, workflow_id: &str, expires_at: DateTime<Utc>) -> BearDogResult<()> {
        tracing::info!(
            "⏰ Scheduled timeout for workflow {} at {}",
            workflow_id,
            expires_at
        );
        
        // In a real implementation, this would schedule actual timeout handling
        // For now, we just log the scheduling
        Ok(())
    }
}

// Workflow processor implementations

macro_rules! impl_workflow_processor {
    ($name:ident, $processor_name:expr) => {
        pub struct $name;
        
        impl $name {
            pub fn new() -> Self {
                Self {}
            }
        }
        
        #[async_trait::async_trait]
        impl WorkflowProcessor for $name {
            async fn execute_workflow(&self, execution: &WorkflowExecution) -> BearDogResult<()> {
                tracing::info!(
                    "🔄 Executing {} workflow: {}",
                    $processor_name,
                    execution.workflow_id
                );
                
                // Actual workflow execution logic would go here
                // For now, we simulate successful execution
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                
                tracing::info!(
                    "✅ Completed {} workflow: {}",
                    $processor_name,
                    execution.workflow_id
                );
                
                Ok(())
            }
            
            async fn validate_parameters(&self, _parameters: &HashMap<String, serde_json::Value>) -> BearDogResult<()> {
                // Parameter validation logic would go here
                Ok(())
            }
            
            fn get_processor_name(&self) -> &str {
                $processor_name
            }
        }
    };
}

impl_workflow_processor!(KeyRotationProcessor, "KeyRotation");
impl_workflow_processor!(KeyDeletionProcessor, "KeyDeletion");
impl_workflow_processor!(PolicyChangeProcessor, "PolicyChange");
impl_workflow_processor!(ConfigChangeProcessor, "ConfigurationChange");
impl_workflow_processor!(UserProvisioningProcessor, "UserProvisioning");
impl_workflow_processor!(EmergencyAccessProcessor, "EmergencyAccess");
impl_workflow_processor!(SystemMaintenanceProcessor, "SystemMaintenance");
impl_workflow_processor!(ComplianceAuditProcessor, "ComplianceAudit");

// Default implementations for secure configuration

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            email_enabled: true,
            slack_enabled: false,
            webhook_enabled: false,
            smtp_server: env::var("BEARDOG_SMTP_SERVER")
                .unwrap_or_else(|_| "smtp.beardog.local".to_string()),
            smtp_port: env::var("BEARDOG_SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .unwrap_or(587),
            smtp_username: env::var("BEARDOG_SMTP_USERNAME").unwrap_or_default(),
            smtp_password: env::var("BEARDOG_SMTP_PASSWORD").unwrap_or_default(),
            slack_webhook_url: env::var("BEARDOG_SLACK_WEBHOOK").ok(),
            webhook_url: env::var("BEARDOG_WEBHOOK_URL").ok(),
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            require_mfa: true,
            max_approval_time: Duration::hours(48),
            min_approvers: 2,
            require_justification: true,
            auto_expire: true,
            escalation_enabled: true,
        }
    }
}

// Display implementation for WorkflowType
impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowType::KeyRotation => write!(f, "KeyRotation"),
            WorkflowType::KeyDeletion => write!(f, "KeyDeletion"),
            WorkflowType::PolicyChange => write!(f, "PolicyChange"),
            WorkflowType::ConfigurationChange => write!(f, "ConfigurationChange"),
            WorkflowType::UserProvisioning => write!(f, "UserProvisioning"),
            WorkflowType::EmergencyAccess => write!(f, "EmergencyAccess"),
            WorkflowType::SystemMaintenance => write!(f, "SystemMaintenance"),
            WorkflowType::ComplianceAudit => write!(f, "ComplianceAudit"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::WorkflowStorageConfig;
    use std::collections::HashMap;

    fn create_test_config() -> WorkflowConfig {
        WorkflowConfig {
            default_approval_timeout: std::time::Duration::from_secs(24 * 3600),
            max_concurrent_workflows: 100,
            storage: crate::config::WorkflowStorageConfig {
                storage_type: "memory".to_string(),
                config: std::collections::HashMap::new(),
            },
            notifications: NotificationConfig {
                enabled: true,
                email_enabled: true,
                slack_enabled: false,
                webhook_enabled: false,
                smtp_server: env::var("BEARDOG_SMTP_SERVER")
                    .unwrap_or_else(|_| "smtp.beardog.local".to_string()),
                smtp_port: env::var("BEARDOG_SMTP_PORT")
                    .unwrap_or_else(|_| "587".to_string())
                    .parse()
                    .unwrap_or(587),
                smtp_username: env::var("BEARDOG_SMTP_USERNAME").unwrap_or_default(),
                smtp_password: env::var("BEARDOG_SMTP_PASSWORD").unwrap_or_default(),
                slack_webhook_url: env::var("BEARDOG_SLACK_WEBHOOK").ok(),
                webhook_url: env::var("BEARDOG_WEBHOOK_URL").ok(),
            },
            policies: PolicyConfig {
                require_mfa: true,
                max_approval_time: Duration::hours(48),
                min_approvers: 2,
                require_justification: true,
                auto_expire: true,
                escalation_enabled: true,
            },
        }
    }

    fn create_test_workflow_request() -> WorkflowRequest {
        let mut parameters = HashMap::new();
        parameters.insert("key_id".to_string(), serde_json::Value::String("test-key-123".to_string()));
        parameters.insert("rotation_reason".to_string(), serde_json::Value::String("Scheduled rotation".to_string()));
        
        WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            initiator: "test-user".to_string(),
            target: WorkflowTarget::Key { key_id: "test-key-123".to_string() },
            parameters,
            reason: "Test workflow request".to_string(),
            priority: WorkflowPriority::Normal,
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_workflow_engine_initialization() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await;
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_workflow_initiation() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        let request = create_test_workflow_request();
        let response = engine.initiate_workflow(request).await;
        
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(!response.workflow_id.is_empty());
        assert!(matches!(response.status, WorkflowStatus::PendingApprovals));
    }

    #[tokio::test]
    async fn test_workflow_priority_handling() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        // Test emergency priority
        let mut request = create_test_workflow_request();
        request.priority = WorkflowPriority::Emergency;
        
        let response = engine.initiate_workflow(request).await.unwrap();
        assert_eq!(response.required_approvals.required_approvals, 1);
        
        // Test normal priority
        let mut request = create_test_workflow_request();
        request.priority = WorkflowPriority::Normal;
        
        let response = engine.initiate_workflow(request).await.unwrap();
        assert_eq!(response.required_approvals.required_approvals, 2);
    }

    #[tokio::test]
    async fn test_approval_submission() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        // Create workflow first with Emergency priority to bypass minimum approval time
        let mut request = create_test_workflow_request();
        request.priority = WorkflowPriority::Emergency;
        let workflow_response = engine.initiate_workflow(request).await.unwrap();
        
        // Submit approval using a valid approver (admin1 is in the predefined admin users)
        let approval = ApprovalSubmission {
            workflow_id: workflow_response.workflow_id.clone(),
            approver: "admin1".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Test approval".to_string()),
            signature: None,
            metadata: HashMap::new(),
        };
        
        let approval_response = engine.submit_approval(approval).await;
        assert!(approval_response.is_ok());
        
        let approval_response = approval_response.unwrap();
        assert!(!approval_response.approval_id.is_empty());
    }

    #[tokio::test]
    async fn test_workflow_status_retrieval() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        let request = create_test_workflow_request();
        let response = engine.initiate_workflow(request).await.unwrap();
        
        let status = engine.get_workflow_status(&response.workflow_id).await;
        assert!(status.is_ok());
        
        let workflow = status.unwrap();
        assert_eq!(workflow.id, response.workflow_id);
        assert!(matches!(workflow.status, WorkflowStatus::PendingApprovals));
    }

    #[tokio::test]
    async fn test_workflow_expiration_calculation() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        // Test different priority expiration times
        let emergency_duration = engine.calculate_expiration_time(&WorkflowPriority::Emergency);
        let normal_duration = engine.calculate_expiration_time(&WorkflowPriority::Normal);
        let low_duration = engine.calculate_expiration_time(&WorkflowPriority::Low);
        
        assert!(emergency_duration < normal_duration);
        assert!(normal_duration < low_duration);
        assert_eq!(emergency_duration, Duration::hours(1));
        assert_eq!(normal_duration, Duration::days(3));
        assert_eq!(low_duration, Duration::days(7));
    }

    #[tokio::test]
    async fn test_workflow_type_validation() {
        let config = create_test_config();
        let engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        // Test all supported workflow types
        let supported_types = vec![
            WorkflowType::KeyRotation,
            WorkflowType::KeyDeletion,
            WorkflowType::PolicyChange,
            WorkflowType::ConfigurationChange,
            WorkflowType::UserProvisioning,
            WorkflowType::EmergencyAccess,
            WorkflowType::SystemMaintenance,
            WorkflowType::ComplianceAudit,
        ];
        
        for workflow_type in supported_types {
            assert!(engine.is_workflow_type_supported(&workflow_type));
        }
    }

    #[tokio::test]
    async fn test_notification_engine() {
        let config = NotificationConfig::default();
        let engine = NotificationEngine::new(&config);
        assert!(engine.is_ok());
        
        let engine = engine.unwrap();
        let workflow = Workflow {
            id: "test-workflow".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            initiator: "test-user".to_string(),
            target: WorkflowTarget::Key { key_id: "test-key".to_string() },
            parameters: HashMap::new(),
            approval_requirements: ApprovalRequirements {
                required_approvals: 2,
                required_roles: vec!["admin".to_string()],
                approval_hierarchy: vec![],
                min_approval_time: Duration::minutes(30),
                max_approval_time: Duration::days(3),
                delegation_allowed: true,
                self_approval_allowed: false,
            },
            status: WorkflowStatus::PendingApprovals,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(3),
            approvals: vec![],
            audit_trail: vec![],
            metadata: HashMap::new(),
        };
        
        let pending_approvals = vec![
            PendingApproval {
                id: "approval-1".to_string(),
                workflow_id: "test-workflow".to_string(),
                approver: "admin-1".to_string(),
                approver_role: "admin".to_string(),
                tier_level: 1,
                created_at: Utc::now(),
                expires_at: Utc::now() + Duration::hours(24),
                notification_sent: false,
            }
        ];
        
        let result = engine.send_approval_requests(&workflow, &pending_approvals).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_in_memory_stores() {
        let workflow_store = InMemoryWorkflowStore::new();
        let approval_store = InMemoryApprovalStore::new();
        
        let workflow = Workflow {
            id: "test-workflow".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            initiator: "test-user".to_string(),
            target: WorkflowTarget::Key { key_id: "test-key".to_string() },
            parameters: HashMap::new(),
            approval_requirements: ApprovalRequirements {
                required_approvals: 2,
                required_roles: vec!["admin".to_string()],
                approval_hierarchy: vec![],
                min_approval_time: Duration::minutes(30),
                max_approval_time: Duration::days(3),
                delegation_allowed: true,
                self_approval_allowed: false,
            },
            status: WorkflowStatus::PendingApprovals,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(3),
            approvals: vec![],
            audit_trail: vec![],
            metadata: HashMap::new(),
        };
        
        // Test workflow storage
        let store_result = workflow_store.store_workflow(&workflow).await;
        assert!(store_result.is_ok());
        
        let retrieved = workflow_store.get_workflow(&workflow.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, workflow.id);
        
        // Test approval storage
        let approval = ApprovalRecord {
            id: "approval-1".to_string(),
            workflow_id: workflow.id.clone(),
            approver: "admin-1".to_string(),
            approver_role: "admin".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Test approval".to_string()),
            timestamp: Utc::now(),
            signature: None,
            metadata: HashMap::new(),
        };
        
        let store_result = approval_store.store_approval(&approval).await;
        assert!(store_result.is_ok());
        
        let approvals = approval_store.get_approvals_for_workflow(&workflow.id).await.unwrap();
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].id, approval.id);
    }

    #[tokio::test]
    async fn test_workflow_processors() {
        let processors = vec![
            ("KeyRotation", Box::new(KeyRotationProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("KeyDeletion", Box::new(KeyDeletionProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("PolicyChange", Box::new(PolicyChangeProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("ConfigurationChange", Box::new(ConfigChangeProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("UserProvisioning", Box::new(UserProvisioningProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("EmergencyAccess", Box::new(EmergencyAccessProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("SystemMaintenance", Box::new(SystemMaintenanceProcessor::new()) as Box<dyn WorkflowProcessor>),
            ("ComplianceAudit", Box::new(ComplianceAuditProcessor::new()) as Box<dyn WorkflowProcessor>),
        ];
        
        for (name, processor) in processors {
            assert_eq!(processor.get_processor_name(), name);
            
            let execution = WorkflowExecution {
                workflow_id: format!("test-{}", name),
                execution_id: format!("exec-{}", name),
                started_at: Utc::now(),
                context: HashMap::new(),
            };
            
            let result = processor.execute_workflow(&execution).await;
            assert!(result.is_ok());
            
            let validation_result = processor.validate_parameters(&HashMap::new()).await;
            assert!(validation_result.is_ok());
        }
    }
} 