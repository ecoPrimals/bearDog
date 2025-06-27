# BearDog Multi-Party Approval Workflows Specification

**Version:** 1.0  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** HIGH  

## 🎯 **Overview**

BearDog's Multi-Party Approval Workflows provide enterprise-grade governance for sensitive operations:
- **Cryptographic approval processes** for key operations
- **Role-based approval hierarchies**
- **Time-bound approval windows**
- **Audit-compliant workflow tracking**
- **Automated workflow orchestration**
- **Integration with external approval systems**

## 🔄 **Workflow Architecture**

### **Core Workflow Engine**
```rust
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

pub struct MultiPartyWorkflowEngine {
    config: Arc<MultiPartyConfig>,
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

impl MultiPartyWorkflowEngine {
    pub async fn new(config: MultiPartyConfig) -> Result<Self> {
        let workflow_store = Arc::new(DatabaseWorkflowStore::new(&config.database).await?);
        let approval_store = Arc::new(DatabaseApprovalStore::new(&config.database).await?);
        let notification_engine = Arc::new(NotificationEngine::new(&config.notifications).await?);
        let policy_engine = Arc::new(WorkflowPolicyEngine::new(&config.policies).await?);
        let scheduler = Arc::new(WorkflowScheduler::new());
        
        let mut workflow_processors: HashMap<WorkflowType, Box<dyn WorkflowProcessor>> = HashMap::new();
        workflow_processors.insert(WorkflowType::KeyRotation, Box::new(KeyRotationProcessor::new()));
        workflow_processors.insert(WorkflowType::KeyDeletion, Box::new(KeyDeletionProcessor::new()));
        workflow_processors.insert(WorkflowType::PolicyChange, Box::new(PolicyChangeProcessor::new()));
        workflow_processors.insert(WorkflowType::ConfigurationChange, Box::new(ConfigChangeProcessor::new()));
        workflow_processors.insert(WorkflowType::UserProvisioning, Box::new(UserProvisioningProcessor::new()));
        workflow_processors.insert(WorkflowType::EmergencyAccess, Box::new(EmergencyAccessProcessor::new()));
        
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
    
    pub async fn initiate_workflow(&self, request: WorkflowRequest) -> Result<WorkflowResponse> {
        // Validate the workflow request
        self.validate_workflow_request(&request).await?;
        
        // Determine approval requirements
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
            approval_requirements,
            status: WorkflowStatus::PendingApprovals,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(self.config.default_approval_timeout_hours as i64),
            approvals: Vec::new(),
            audit_trail: vec![WorkflowAuditEntry {
                timestamp: Utc::now(),
                action: WorkflowAction::Initiated,
                actor: request.initiator.clone(),
                details: "Workflow initiated".to_string(),
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
        
        Ok(WorkflowResponse {
            workflow_id: workflow.id,
            status: WorkflowStatus::PendingApprovals,
            required_approvals: workflow.approval_requirements.clone(),
            pending_approvers: pending_approvals.iter().map(|pa| pa.approver.clone()).collect(),
            estimated_completion: workflow.expires_at,
        })
    }
    
    pub async fn submit_approval(&self, approval: ApprovalSubmission) -> Result<ApprovalResponse> {
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
        
        // Create approval record
        let approval_record = ApprovalRecord {
            id: Uuid::new_v4().to_string(),
            workflow_id: approval.workflow_id.clone(),
            approver: approval.approver.clone(),
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
        
        // Send notifications
        self.notification_engine.send_approval_status_update(&workflow, &approval_record).await?;
        
        Ok(ApprovalResponse {
            approval_id: approval_record.id,
            workflow_status: workflow.status,
            remaining_approvals: workflow_result.remaining_approvals,
            estimated_completion: workflow.expires_at,
        })
    }
    
    async fn execute_approved_workflow(&self, workflow: &Workflow) -> Result<()> {
        // Get appropriate processor
        let processors = self.workflow_processors.read().await;
        let processor = processors.get(&workflow.workflow_type)
            .ok_or_else(|| BearDogError::UnsupportedWorkflowType(workflow.workflow_type.clone()))?;
        
        // Create execution context
        let execution_context = WorkflowExecutionContext {
            workflow: workflow.clone(),
            config: self.config.clone(),
            audit_logger: self.create_audit_logger(),
        };
        
        // Execute workflow
        let execution_result = processor.execute(&execution_context).await?;
        
        // Update workflow with execution result
        let mut updated_workflow = workflow.clone();
        updated_workflow.status = WorkflowStatus::Completed;
        updated_workflow.audit_trail.push(WorkflowAuditEntry {
            timestamp: Utc::now(),
            action: WorkflowAction::Executed,
            actor: "system".to_string(),
            details: format!("Workflow executed successfully: {}", execution_result.summary),
        });
        
        // Store final workflow state
        self.workflow_store.store_workflow(&updated_workflow).await?;
        
        // Remove from active workflows
        self.active_workflows.write().await.remove(&workflow.id);
        self.pending_approvals.write().await.remove(&workflow.id);
        
        // Send completion notifications
        self.notification_engine.send_workflow_completion(&updated_workflow, &execution_result).await?;
        
        Ok(())
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum WorkflowType {
    KeyRotation,
    KeyDeletion,
    PolicyChange,
    ConfigurationChange,
    UserProvisioning,
    UserDeprovisioning,
    RoleAssignment,
    EmergencyAccess,
    SystemMaintenance,
    ComplianceException,
    DataExport,
    SecurityIncidentResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTarget {
    Key { key_id: String },
    User { user_id: String },
    Policy { policy_id: String },
    Configuration { config_section: String },
    System { component: String },
    Data { resource_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirements {
    pub min_approvers: u32,
    pub required_roles: Vec<String>,
    pub required_users: Vec<String>,
    pub approval_levels: Vec<ApprovalLevel>,
    pub unanimous_required: bool,
    pub allow_self_approval: bool,
    pub timeout_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalLevel {
    pub level: u32,
    pub required_approvers: u32,
    pub eligible_roles: Vec<String>,
    pub eligible_users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    PendingApprovals,
    Approved,
    Rejected,
    Expired,
    Cancelled,
    Executing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String,
    pub workflow_id: String,
    pub approver: String,
    pub decision: ApprovalDecision,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>, // Digital signature for non-repudiation
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: WorkflowAction,
    pub actor: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowAction {
    Initiated,
    Approved,
    Rejected,
    Delegated,
    Expired,
    Cancelled,
    Executed,
    Failed,
}
```

## 🔐 **Workflow Processors**

### **Key Rotation Workflow Processor**
```rust
pub struct KeyRotationProcessor {
    key_manager: Arc<dyn KeyManager>,
    audit_logger: Arc<dyn AuditLogger>,
}

impl KeyRotationProcessor {
    pub fn new() -> Self {
        Self {
            key_manager: Arc::new(BearDogKeyManager::new()),
            audit_logger: Arc::new(AuditLogger::new()),
        }
    }
}

#[async_trait]
impl WorkflowProcessor for KeyRotationProcessor {
    async fn execute(&self, context: &WorkflowExecutionContext) -> Result<WorkflowExecutionResult> {
        let workflow = &context.workflow;
        
        // Extract key ID from workflow parameters
        let key_id = workflow.parameters.get("key_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidWorkflowParameters("Missing key_id".to_string()))?;
        
        // Perform key rotation
        let rotation_request = KeyRotationRequest {
            key_id: key_id.to_string(),
            reason: "Multi-party approved rotation".to_string(),
            rotate_derived_keys: workflow.parameters.get("rotate_derived_keys")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            metadata: workflow.metadata.clone(),
        };
        
        let rotation_result = self.key_manager.rotate_key(rotation_request).await?;
        
        // Audit the key rotation
        self.audit_logger.log_key_rotation(&rotation_result, &workflow.audit_trail).await?;
        
        Ok(WorkflowExecutionResult {
            success: true,
            summary: format!("Key {} rotated successfully", key_id),
            details: serde_json::to_value(&rotation_result)?,
            artifacts: vec![WorkflowArtifact {
                artifact_type: "key_rotation_result".to_string(),
                data: serde_json::to_value(&rotation_result)?,
                metadata: HashMap::new(),
            }],
        })
    }
    
    async fn validate_parameters(&self, parameters: &HashMap<String, serde_json::Value>) -> Result<()> {
        // Validate required parameters for key rotation
        if !parameters.contains_key("key_id") {
            return Err(BearDogError::InvalidWorkflowParameters("Missing key_id".to_string()));
        }
        
        if let Some(key_id) = parameters.get("key_id").and_then(|v| v.as_str()) {
            // Verify key exists and is eligible for rotation
            if !self.key_manager.key_exists(key_id).await? {
                return Err(BearDogError::InvalidWorkflowParameters(
                    format!("Key {} does not exist", key_id)
                ));
            }
        }
        
        Ok(())
    }
    
    fn workflow_type(&self) -> WorkflowType {
        WorkflowType::KeyRotation
    }
}
```

### **Policy Change Workflow Processor**
```rust
pub struct PolicyChangeProcessor {
    policy_manager: Arc<dyn PolicyManager>,
    security_validator: Arc<SecurityValidator>,
    backup_manager: Arc<PolicyBackupManager>,
}

#[async_trait]
impl WorkflowProcessor for PolicyChangeProcessor {
    async fn execute(&self, context: &WorkflowExecutionContext) -> Result<WorkflowExecutionResult> {
        let workflow = &context.workflow;
        
        // Extract policy change details
        let policy_id = workflow.parameters.get("policy_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::InvalidWorkflowParameters("Missing policy_id".to_string()))?;
        
        let new_policy_content = workflow.parameters.get("new_policy")
            .ok_or_else(|| BearDogError::InvalidWorkflowParameters("Missing new_policy".to_string()))?;
        
        // Backup current policy
        let backup_result = self.backup_manager.backup_policy(policy_id).await?;
        
        // Validate new policy
        let new_policy: SecurityPolicy = serde_json::from_value(new_policy_content.clone())?;
        self.security_validator.validate_policy(&new_policy).await?;
        
        // Apply policy change
        let change_result = self.policy_manager.update_policy(policy_id, new_policy).await?;
        
        Ok(WorkflowExecutionResult {
            success: true,
            summary: format!("Policy {} updated successfully", policy_id),
            details: serde_json::to_value(&change_result)?,
            artifacts: vec![
                WorkflowArtifact {
                    artifact_type: "policy_backup".to_string(),
                    data: serde_json::to_value(&backup_result)?,
                    metadata: HashMap::new(),
                },
                WorkflowArtifact {
                    artifact_type: "policy_change_result".to_string(),
                    data: serde_json::to_value(&change_result)?,
                    metadata: HashMap::new(),
                },
            ],
        })
    }
    
    async fn validate_parameters(&self, parameters: &HashMap<String, serde_json::Value>) -> Result<()> {
        // Validate policy change parameters
        if !parameters.contains_key("policy_id") || !parameters.contains_key("new_policy") {
            return Err(BearDogError::InvalidWorkflowParameters(
                "Missing required parameters: policy_id, new_policy".to_string()
            ));
        }
        
        // Validate new policy syntax
        if let Some(new_policy_value) = parameters.get("new_policy") {
            let new_policy: SecurityPolicy = serde_json::from_value(new_policy_value.clone())
                .map_err(|e| BearDogError::InvalidWorkflowParameters(
                    format!("Invalid policy format: {}", e)
                ))?;
            
            self.security_validator.validate_policy(&new_policy).await?;
        }
        
        Ok(())
    }
    
    fn workflow_type(&self) -> WorkflowType {
        WorkflowType::PolicyChange
    }
}
```

## 📱 **Notification Engine**

### **Multi-Channel Notifications**
```rust
pub struct NotificationEngine {
    config: NotificationConfig,
    channels: HashMap<NotificationChannel, Box<dyn NotificationProvider>>,
    template_engine: Arc<NotificationTemplateEngine>,
}

impl NotificationEngine {
    pub async fn new(config: &NotificationConfig) -> Result<Self> {
        let mut channels: HashMap<NotificationChannel, Box<dyn NotificationProvider>> = HashMap::new();
        
        if config.email.enabled {
            channels.insert(
                NotificationChannel::Email,
                Box::new(EmailNotificationProvider::new(&config.email).await?)
            );
        }
        
        if config.slack.enabled {
            channels.insert(
                NotificationChannel::Slack,
                Box::new(SlackNotificationProvider::new(&config.slack).await?)
            );
        }
        
        if config.sms.enabled {
            channels.insert(
                NotificationChannel::SMS,
                Box::new(SmsNotificationProvider::new(&config.sms).await?)
            );
        }
        
        if config.webhook.enabled {
            channels.insert(
                NotificationChannel::Webhook,
                Box::new(WebhookNotificationProvider::new(&config.webhook).await?)
            );
        }
        
        Ok(Self {
            config: config.clone(),
            channels,
            template_engine: Arc::new(NotificationTemplateEngine::new()),
        })
    }
    
    pub async fn send_approval_requests(
        &self,
        workflow: &Workflow,
        pending_approvals: &[PendingApproval],
    ) -> Result<()> {
        for pending_approval in pending_approvals {
            let notification = self.create_approval_request_notification(workflow, pending_approval).await?;
            
            for channel in &self.config.default_channels {
                if let Some(provider) = self.channels.get(channel) {
                    if let Err(e) = provider.send_notification(&notification).await {
                        tracing::error!("Failed to send notification via {:?}: {}", channel, e);
                        // Continue with other channels
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn send_workflow_completion(
        &self,
        workflow: &Workflow,
        execution_result: &WorkflowExecutionResult,
    ) -> Result<()> {
        let notification = self.create_completion_notification(workflow, execution_result).await?;
        
        // Send to workflow initiator
        for channel in &self.config.default_channels {
            if let Some(provider) = self.channels.get(channel) {
                if let Err(e) = provider.send_notification(&notification).await {
                    tracing::error!("Failed to send completion notification via {:?}: {}", channel, e);
                }
            }
        }
        
        // Send to all approvers
        for approval in &workflow.approvals {
            let approver_notification = self.create_approver_completion_notification(
                workflow,
                execution_result,
                &approval.approver,
            ).await?;
            
            for channel in &self.config.default_channels {
                if let Some(provider) = self.channels.get(channel) {
                    if let Err(e) = provider.send_notification(&approver_notification).await {
                        tracing::error!("Failed to send approver notification via {:?}: {}", channel, e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn create_approval_request_notification(
        &self,
        workflow: &Workflow,
        pending_approval: &PendingApproval,
    ) -> Result<Notification> {
        let template_data = json!({
            "workflow_id": workflow.id,
            "workflow_type": workflow.workflow_type,
            "initiator": workflow.initiator,
            "target": workflow.target,
            "created_at": workflow.created_at,
            "expires_at": workflow.expires_at,
            "approver": pending_approval.approver,
            "approval_url": format!("{}/workflows/{}/approve", self.config.base_url, workflow.id),
        });
        
        let subject = self.template_engine.render_template(
            "approval_request_subject",
            &template_data,
        ).await?;
        
        let body = self.template_engine.render_template(
            "approval_request_body",
            &template_data,
        ).await?;
        
        Ok(Notification {
            id: Uuid::new_v4().to_string(),
            recipient: pending_approval.approver.clone(),
            subject,
            body,
            priority: NotificationPriority::High,
            notification_type: NotificationType::ApprovalRequest,
            metadata: template_data,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Slack,
    SMS,
    Webhook,
    PushNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub recipient: String,
    pub subject: String,
    pub body: String,
    pub priority: NotificationPriority,
    pub notification_type: NotificationType,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    ApprovalRequest,
    ApprovalReminder,
    WorkflowCompleted,
    WorkflowRejected,
    WorkflowExpired,
    SystemAlert,
}

#[async_trait]
pub trait NotificationProvider: Send + Sync {
    async fn send_notification(&self, notification: &Notification) -> Result<()>;
    async fn validate_configuration(&self) -> Result<()>;
    fn provider_name(&self) -> &str;
}
```

## ⚙️ **Configuration**

### **Multi-Party Workflow Configuration**
```toml
[multi_party]
# General settings
default_approval_timeout_hours = 24
max_concurrent_workflows = 100
enable_workflow_scheduling = true
auto_cleanup_completed_workflows_days = 90

[multi_party.approval_policies]
# Key management workflows
key_rotation_min_approvers = 2
key_rotation_required_roles = ["key-admin", "security-officer"]
key_deletion_min_approvers = 3
key_deletion_required_roles = ["key-admin", "security-officer", "compliance-officer"]

# Policy workflows
policy_change_min_approvers = 2
policy_change_required_roles = ["security-admin", "compliance-officer"]
policy_change_unanimous_required = true

# Configuration workflows
config_change_min_approvers = 2
config_change_required_roles = ["system-admin", "security-officer"]

# Emergency workflows
emergency_access_min_approvers = 2
emergency_access_required_roles = ["emergency-coordinator", "security-officer"]
emergency_access_timeout_hours = 4

[multi_party.notifications]
# Notification settings
base_url = "https://beardog.internal:8443"
default_channels = ["email", "slack"]
reminder_intervals_hours = [4, 8, 16]
escalation_enabled = true

[multi_party.notifications.email]
enabled = true
smtp_server = "smtp.internal.com"
smtp_port = 587
smtp_username_env_var = "SMTP_USERNAME"
smtp_password_env_var = "SMTP_PASSWORD"
from_address = "beardog-workflows@company.com"
use_tls = true

[multi_party.notifications.slack]
enabled = true
webhook_url_env_var = "SLACK_WEBHOOK_URL"
channel = "#beardog-approvals"
mention_users = true

[multi_party.notifications.sms]
enabled = false
provider = "twilio"
account_sid_env_var = "TWILIO_ACCOUNT_SID"
auth_token_env_var = "TWILIO_AUTH_TOKEN"
from_number = "+1234567890"

[multi_party.database]
# Workflow storage
connection_string_env_var = "WORKFLOW_DATABASE_URL"
connection_pool_size = 10
enable_encryption = true
backup_enabled = true
backup_interval_hours = 6

[multi_party.security]
# Security settings
require_digital_signatures = true
signature_algorithm = "ed25519"
enable_approval_delegation = true
max_delegation_depth = 2
audit_all_operations = true

[multi_party.performance]
# Performance settings
workflow_processing_threads = 4
notification_batch_size = 50
database_connection_timeout_seconds = 30
cache_size = 1000
cache_ttl_minutes = 30
```

## 🔍 **Workflow Templates**

### **Predefined Workflow Templates**
```rust
pub struct WorkflowTemplateManager {
    templates: HashMap<WorkflowType, WorkflowTemplate>,
}

impl WorkflowTemplateManager {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Key rotation template
        templates.insert(WorkflowType::KeyRotation, WorkflowTemplate {
            workflow_type: WorkflowType::KeyRotation,
            name: "Key Rotation".to_string(),
            description: "Rotate encryption keys with multi-party approval".to_string(),
            required_parameters: vec![
                "key_id".to_string(),
                "rotation_reason".to_string(),
            ],
            optional_parameters: vec![
                "rotate_derived_keys".to_string(),
                "notification_groups".to_string(),
            ],
            default_approval_requirements: ApprovalRequirements {
                min_approvers: 2,
                required_roles: vec!["key-admin".to_string(), "security-officer".to_string()],
                required_users: Vec::new(),
                approval_levels: vec![
                    ApprovalLevel {
                        level: 1,
                        required_approvers: 1,
                        eligible_roles: vec!["key-admin".to_string()],
                        eligible_users: Vec::new(),
                    },
                    ApprovalLevel {
                        level: 2,
                        required_approvers: 1,
                        eligible_roles: vec!["security-officer".to_string()],
                        eligible_users: Vec::new(),
                    },
                ],
                unanimous_required: false,
                allow_self_approval: false,
                timeout_hours: 24,
            },
            risk_level: WorkflowRiskLevel::High,
            compliance_requirements: vec![
                ComplianceRequirement::GDPR,
                ComplianceRequirement::SOX,
            ],
        });
        
        // Emergency access template
        templates.insert(WorkflowType::EmergencyAccess, WorkflowTemplate {
            workflow_type: WorkflowType::EmergencyAccess,
            name: "Emergency Access".to_string(),
            description: "Grant emergency access to critical systems".to_string(),
            required_parameters: vec![
                "user_id".to_string(),
                "resource_id".to_string(),
                "emergency_reason".to_string(),
                "access_duration_hours".to_string(),
            ],
            optional_parameters: vec![
                "incident_id".to_string(),
                "supervisor_contact".to_string(),
            ],
            default_approval_requirements: ApprovalRequirements {
                min_approvers: 2,
                required_roles: vec!["emergency-coordinator".to_string(), "security-officer".to_string()],
                required_users: Vec::new(),
                approval_levels: vec![
                    ApprovalLevel {
                        level: 1,
                        required_approvers: 1,
                        eligible_roles: vec!["emergency-coordinator".to_string()],
                        eligible_users: Vec::new(),
                    },
                    ApprovalLevel {
                        level: 2,
                        required_approvers: 1,
                        eligible_roles: vec!["security-officer".to_string()],
                        eligible_users: Vec::new(),
                    },
                ],
                unanimous_required: true,
                allow_self_approval: false,
                timeout_hours: 4, // Shorter timeout for emergencies
            },
            risk_level: WorkflowRiskLevel::Critical,
            compliance_requirements: vec![
                ComplianceRequirement::SOX,
                ComplianceRequirement::HIPAA,
            ],
        });
        
        Self { templates }
    }
    
    pub fn get_template(&self, workflow_type: &WorkflowType) -> Option<&WorkflowTemplate> {
        self.templates.get(workflow_type)
    }
    
    pub fn create_workflow_from_template(
        &self,
        template: &WorkflowTemplate,
        parameters: HashMap<String, serde_json::Value>,
        initiator: String,
    ) -> Result<WorkflowRequest> {
        // Validate required parameters
        for required_param in &template.required_parameters {
            if !parameters.contains_key(required_param) {
                return Err(BearDogError::InvalidWorkflowParameters(
                    format!("Missing required parameter: {}", required_param)
                ));
            }
        }
        
        Ok(WorkflowRequest {
            workflow_type: template.workflow_type.clone(),
            initiator,
            target: self.extract_target_from_parameters(&template.workflow_type, &parameters)?,
            parameters,
            approval_requirements: Some(template.default_approval_requirements.clone()),
            metadata: HashMap::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub workflow_type: WorkflowType,
    pub name: String,
    pub description: String,
    pub required_parameters: Vec<String>,
    pub optional_parameters: Vec<String>,
    pub default_approval_requirements: ApprovalRequirements,
    pub risk_level: WorkflowRiskLevel,
    pub compliance_requirements: Vec<ComplianceRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceRequirement {
    GDPR,
    HIPAA,
    SOX,
    PCI,
    FedRAMP,
}
```

## 🧪 **Testing Strategy**

### **Workflow Testing Framework**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_key_rotation_workflow() {
        let config = create_test_config();
        let workflow_engine = MultiPartyWorkflowEngine::new(config).await.unwrap();
        
        // Create workflow request
        let workflow_request = WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            initiator: "alice@company.com".to_string(),
            target: WorkflowTarget::Key { key_id: "test-key-123".to_string() },
            parameters: {
                let mut params = HashMap::new();
                params.insert("key_id".to_string(), json!("test-key-123"));
                params.insert("rotation_reason".to_string(), json!("Scheduled rotation"));
                params
            },
            approval_requirements: None,
            metadata: HashMap::new(),
        };
        
        // Initiate workflow
        let response = workflow_engine.initiate_workflow(workflow_request).await.unwrap();
        assert_eq!(response.status, WorkflowStatus::PendingApprovals);
        
        // Submit first approval
        let approval1 = ApprovalSubmission {
            workflow_id: response.workflow_id.clone(),
            approver: "bob@company.com".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Approved for scheduled rotation".to_string()),
            signature: None,
            metadata: HashMap::new(),
        };
        
        let approval_response1 = workflow_engine.submit_approval(approval1).await.unwrap();
        assert_eq!(approval_response1.workflow_status, WorkflowStatus::PendingApprovals);
        
        // Submit second approval
        let approval2 = ApprovalSubmission {
            workflow_id: response.workflow_id.clone(),
            approver: "charlie@company.com".to_string(),
            decision: ApprovalDecision::Approved,
            reason: Some("Security approved".to_string()),
            signature: None,
            metadata: HashMap::new(),
        };
        
        let approval_response2 = workflow_engine.submit_approval(approval2).await.unwrap();
        assert_eq!(approval_response2.workflow_status, WorkflowStatus::Approved);
        
        // Verify workflow completion
        // (Additional verification logic would be implemented)
    }
    
    #[tokio::test]
    async fn test_workflow_rejection() {
        // Test workflow rejection scenario
        // (Implementation details...)
    }
    
    #[tokio::test]
    async fn test_workflow_timeout() {
        // Test workflow timeout handling
        // (Implementation details...)
    }
}
```

---

**Next Steps**: Implement workflow scheduling, approval delegation, and integration with external approval systems. 