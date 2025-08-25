# BearDog Multi-Party Approval Workflows Specification

**Version:** 1.1  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** CRITICAL  

## 🎯 **Overview**

BearDog's Multi-Party Approval Workflows provide enterprise-grade governance for sensitive operations AND **cross-node authorization** in the distributed ecosystem:
- **Cryptographic approval processes** for key operations
- **Cross-node permission workflows** with signed authorization tokens
- **Role-based approval hierarchies**
- **Time-bound approval windows**
- **Audit-compliant workflow tracking**
- **Peer-to-peer trust establishment**
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
    
    // Cross-node components
    cross_node_auth_store: Arc<dyn CrossNodeAuthStore>,
    proof_verifier: Arc<ProofVerifier>,
    node_registry: Arc<NodeRegistry>,
    
    // Active workflows
    active_workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    pending_approvals: Arc<RwLock<HashMap<String, Vec<PendingApproval>>>>,
    cross_node_authorizations: Arc<RwLock<HashMap<String, CrossNodeAuthorization>>>,
    
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
            cross_node_auth_store: Arc::new(DatabaseCrossNodeAuthStore::new(&config.database).await?),
            proof_verifier: Arc::new(ProofVerifier::new()),
            node_registry: Arc::new(DatabaseNodeRegistry::new(&config.database).await?),
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
            cross_node_authorizations: Arc::new(RwLock::new(HashMap::new())),
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

    /// Initiate a cross-node permission request
    pub async fn request_cross_node_permission(
        &self,
        target_node_id: &str,
        requested_permissions: Vec<ResourcePermission>,
        justification: &str,
    ) -> BearDogResult<String> {
        let workflow_id = Uuid::new_v4().to_string();
        
        let workflow = Workflow {
            id: workflow_id.clone(),
            workflow_type: WorkflowType::CrossNodePermissionRequest,
            initiator: self.config.node_id.clone(),
            target: WorkflowTarget::CrossNode {
                target_node: target_node_id.to_string(),
                requested_permissions: requested_permissions.clone(),
            },
            parameters: hashmap! {
                "justification".to_string() => serde_json::Value::String(justification.to_string()),
                "requested_permissions".to_string() => serde_json::to_value(requested_permissions)?,
            },
            status: WorkflowStatus::PendingRemoteApproval,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
            audit_trail: vec![WorkflowAuditEntry {
                timestamp: Utc::now(),
                actor: self.config.node_id.clone(),
                action: "workflow_initiated".to_string(),
                details: "Cross-node permission request initiated".to_string(),
            }],
            ..Default::default()
        };
        
        // Store locally
        self.workflow_store.store_workflow(&workflow).await?;
        
        // Send to target node via SongBird
        self.send_cross_node_workflow_request(target_node_id, &workflow).await?;
        
        Ok(workflow_id)
    }
    
    /// Handle incoming cross-node permission request
    pub async fn handle_cross_node_request(
        &self,
        request: CrossNodeWorkflowRequest,
    ) -> BearDogResult<()> {
        let approval_workflow = Workflow {
            id: Uuid::new_v4().to_string(),
            workflow_type: WorkflowType::CrossNodePermissionGrant,
            initiator: request.from_node_id.clone(),
            target: WorkflowTarget::CrossNode {
                target_node: request.from_node_id.clone(),
                requested_permissions: request.requested_permissions.clone(),
            },
            approval_requirements: self.policy_engine
                .determine_cross_node_approval_requirements(&request)
                .await?,
            status: WorkflowStatus::PendingApprovals,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
            metadata: hashmap! {
                "original_request_id".to_string() => serde_json::Value::String(request.workflow_id),
                "justification".to_string() => serde_json::Value::String(request.justification),
            },
            ..Default::default()
        };
        
        // Store the approval workflow
        self.workflow_store.store_workflow(&approval_workflow).await?;
        
        // Notify required approvers
        self.notification_engine
            .send_cross_node_approval_requests(&approval_workflow)
            .await?;
        
        Ok(())
    }
    
    /// Generate cryptographic proof of authorization
    pub async fn generate_authorization_proof(
        &self,
        workflow_id: &str,
        grantee_node_id: &str,
    ) -> BearDogResult<CrossNodeAuthorization> {
        let workflow = self.workflow_store.get_workflow(workflow_id).await?
            .ok_or(BearDogError::WorkflowNotFound)?;
        
        if workflow.status != WorkflowStatus::Approved {
            return Err(BearDogError::WorkflowNotApproved);
        }
        
        let permissions = workflow.extract_granted_permissions()?;
        
        let authorization = CrossNodeAuthorization {
            id: Uuid::new_v4().to_string(),
            grantor_node_id: self.config.node_id.clone(),
            grantee_node_id: grantee_node_id.to_string(),
            resource_permissions: permissions,
            granted_at: Utc::now(),
            expires_at: workflow.calculate_permission_expiry(),
            workflow_id: workflow_id.to_string(),
            approval_chain: workflow.approvals.clone(),
            conditions: workflow.extract_conditions()?,
            
            // Cryptographic proof
            grantor_signature: self.sign_authorization(&authorization_data).await?,
        };
        
        // Store locally for audit
        self.cross_node_auth_store.store_authorization(&authorization).await?;
        
        Ok(authorization)
    }
    
    /// Verify and store received authorization proof
    pub async fn receive_cross_node_authorization(
        &self,
        authorization: CrossNodeAuthorization,
    ) -> BearDogResult<()> {
        // Verify the cryptographic signature
        let grantor_public_key = self.node_registry
            .get_node_public_key(&authorization.grantor_node_id)
            .await?;
        
        self.proof_verifier.verify_authorization_signature(
            &authorization,
            &grantor_public_key,
        )?;
        
        // Verify the approval chain
        self.verify_approval_chain(&authorization.approval_chain).await?;
        
        // Store the authorization proof
        self.cross_node_auth_store.store_authorization(&authorization).await?;
        
        // Log successful receipt
        tracing::info!(
            "✅ Received valid cross-node authorization from {} for permissions: {:?}",
            authorization.grantor_node_id,
            authorization.resource_permissions
        );
        
        Ok(())
    }
    
    /// Present proof of authorization for cross-node operation
    pub async fn prove_authorization(
        &self,
        target_node_id: &str,
        operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof> {
        let authorization = self.cross_node_auth_store
            .get_authorization_for_node(target_node_id)
            .await?
            .ok_or(BearDogError::NoAuthorization)?;
        
        // Check if authorization is still valid
        if authorization.expires_at < Utc::now() {
            return Err(BearDogError::AuthorizationExpired);
        }
        
        // Check if operation is permitted
        if !authorization.permits_operation(operation) {
            return Err(BearDogError::OperationNotPermitted);
        }
        
        // Create proof that can be verified by target node
        let proof = AuthorizationProof {
            authorization: authorization.clone(),
            operation: operation.clone(),
            request_timestamp: Utc::now(),
            requester_signature: self.sign_operation_request(operation).await?,
        };
        
        Ok(proof)
    }
    
    /// Verify authorization proof from another node
    pub async fn verify_authorization_proof(
        &self,
        proof: &AuthorizationProof,
    ) -> BearDogResult<bool> {
        // Verify this is our authorization
        if proof.authorization.grantor_node_id != self.config.node_id {
            return Ok(false);
        }
        
        // Verify the requester's signature
        let grantee_public_key = self.node_registry
            .get_node_public_key(&proof.authorization.grantee_node_id)
            .await?;
        
        proof.verify_requester_signature(&grantee_public_key)?;
        
        // Verify our own authorization signature
        proof.authorization.verify_signature(&self.config.node_keypair.public_key())?;
        
        // Check if authorization is still valid
        if proof.authorization.expires_at < Utc::now() {
            return Ok(false);
        }
        
        // Check if operation is permitted
        Ok(proof.authorization.permits_operation(&proof.operation))
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
    CrossNodePermissionRequest,    // Request permission from another node
    CrossNodePermissionGrant,      // Grant permission to another node
    CrossNodeStorageAccess,        // Access storage on another node
    CrossNodeDataSharing,          // Share data with another node
    CrossNodeKeyRecovery,          // Recover keys with help from another node
    CrossNodeEmergencyAccess,      // Emergency access via another node
    CrossNodeTrustEstablishment,   // Establish trust relationship
    CrossNodePermissionRevocation, // Revoke previously granted permissions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTarget {
    Key { key_id: String },
    User { user_id: String },
    Policy { policy_id: String },
    Configuration { config_section: String },
    System { component: String },
    Data { resource_id: String },
    CrossNode {
        target_node: String,
        requested_permissions: Vec<ResourcePermission>,
    },
    CrossNodeData {
        target_node: String,
        data_resource: String,
        access_type: DataAccessType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAccessType {
    Read,
    Write,
    Delete,
    Share,
    Copy,
    Move,
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
    PendingRemoteApproval,
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

## 🔐 **Cross-Node Authorization Types**

### **Cross-Node Workflow Types**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum WorkflowType {
    // Local workflows
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
    
    // Cross-node workflows
    CrossNodePermissionRequest,    // Request permission from another node
    CrossNodePermissionGrant,      // Grant permission to another node
    CrossNodeStorageAccess,        // Access storage on another node
    CrossNodeDataSharing,          // Share data with another node
    CrossNodeKeyRecovery,          // Recover keys with help from another node
    CrossNodeEmergencyAccess,      // Emergency access via another node
    CrossNodeTrustEstablishment,   // Establish trust relationship
    CrossNodePermissionRevocation, // Revoke previously granted permissions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTarget {
    // Local targets
    Key { key_id: String },
    User { user_id: String },
    Policy { policy_id: String },
    Configuration { config_section: String },
    System { component: String },
    Data { resource_id: String },
    
    // Cross-node targets
    CrossNode {
        target_node: String,
        requested_permissions: Vec<ResourcePermission>,
    },
    CrossNodeData {
        target_node: String,
        data_resource: String,
        access_type: DataAccessType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAccessType {
    Read,
    Write,
    Delete,
    Share,
    Copy,
    Move,
}
```

### **Cross-Node Authorization Structure**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {
    pub id: String,
    pub grantor_node_id: String,        // Node granting permission
    pub grantee_node_id: String,        // Node receiving permission
    pub resource_permissions: Vec<ResourcePermission>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workflow_id: String,            // Original workflow that granted this
    pub approval_chain: Vec<ApprovalRecord>, // Who approved this
    pub conditions: Vec<AccessCondition>,
    
    // Cryptographic proof
    pub grantor_signature: Ed25519Signature,  // Grantor's BearDog signs this
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourcePermission {
    StoreData {
        max_gb: u64,
        data_types: Vec<DataType>,
        expires_at: DateTime<Utc>,
    },
    RetrieveData {
        own_data_only: bool,
        data_types: Vec<DataType>,
    },
    DeleteData {
        own_data_only: bool,
    },
    ShareData {
        max_recipients: u32,
        require_encryption: bool,
    },
    ComputeAccess {
        max_cpu_hours: u64,
        max_memory_gb: u64,
    },
    NetworkRelay {
        max_bandwidth_mbps: u64,
    },
    KeyRecoveryAssistance {
        key_types: Vec<KeyType>,
    },
    EmergencyAccess {
        escalation_required: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    TimeWindow {
        start_hour: u8,  // 0-23
        end_hour: u8,    // 0-23
        timezone: String,
    },
    GeographicRestriction {
        allowed_countries: Vec<String>,
        allowed_regions: Vec<String>,
    },
    NetworkRestriction {
        allowed_ip_ranges: Vec<String>,
    },
    RequireAdditionalAuth {
        auth_methods: Vec<AuthMethod>,
    },
    MaxConcurrentOperations(u32),
    RequireAuditLog(bool),
    RequireEncryption(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    PersonalData,
    FinancialData,
    HealthData,
    MediaFiles,
    Documents,
    BackupData,
    SystemLogs,
    ApplicationData,
}
```

### **Authorization Proof System**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    pub authorization: CrossNodeAuthorization,
    pub operation: CrossNodeOperation,
    pub request_timestamp: DateTime<Utc>,
    pub requester_signature: Ed25519Signature,  // Grantee signs the request
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeOperation {
    pub operation_type: OperationType,
    pub resource_id: String,
    pub data_size_bytes: Option<u64>,
    pub estimated_duration: Option<Duration>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    StoreData,
    RetrieveData,
    DeleteData,
    ShareData,
    ComputeTask,
    NetworkRelay,
    KeyRecovery,
    EmergencyAccess,
}

impl AuthorizationProof {
    pub fn verify_requester_signature(&self, public_key: &Ed25519PublicKey) -> BearDogResult<()> {
        let signature_data = self.create_signature_data()?;
        public_key.verify(&signature_data, &self.requester_signature)
            .map_err(|_| BearDogError::InvalidSignature)?;
        Ok(())
    }
    
    fn create_signature_data(&self) -> BearDogResult<Vec<u8>> {
        let mut data = Vec::new();
        data.extend_from_slice(self.authorization.id.as_bytes());
        data.extend_from_slice(&self.request_timestamp.timestamp().to_le_bytes());
        data.extend_from_slice(&serde_json::to_vec(&self.operation)?);
        Ok(data)
    }
}

impl CrossNodeAuthorization {
    pub fn permits_operation(&self, operation: &CrossNodeOperation) -> bool {
        for permission in &self.resource_permissions {
            if permission.permits(operation) {
                return true;
            }
        }
        false
    }
    
    pub fn verify_signature(&self, public_key: &Ed25519PublicKey) -> BearDogResult<()> {
        let signature_data = self.create_signature_data()?;
        public_key.verify(&signature_data, &self.grantor_signature)
            .map_err(|_| BearDogError::InvalidSignature)?;
        Ok(())
    }
    
    fn create_signature_data(&self) -> BearDogResult<Vec<u8>> {
        let mut data = Vec::new();
        data.extend_from_slice(self.grantor_node_id.as_bytes());
        data.extend_from_slice(self.grantee_node_id.as_bytes());
        data.extend_from_slice(&self.granted_at.timestamp().to_le_bytes());
        data.extend_from_slice(&self.expires_at.timestamp().to_le_bytes());
        data.extend_from_slice(&serde_json::to_vec(&self.resource_permissions)?);
        Ok(data)
    }
}
```

## 🌐 **Cross-Node Workflow Examples**

### **Example 1: Request Storage Permission**
```rust
// Your BearDog requests permission to store data on friend's NAS
let workflow_id = workflow_engine.request_cross_node_permission(
    "friend-node-id",
    vec![
        ResourcePermission::StoreData {
            max_gb: 100,
            data_types: vec![DataType::MediaFiles, DataType::BackupData],
            expires_at: Utc::now() + Duration::days(365),
        },
        ResourcePermission::RetrieveData {
            own_data_only: true,
            data_types: vec![DataType::MediaFiles, DataType::BackupData],
        },
        ResourcePermission::DeleteData {
            own_data_only: true,
        },
    ],
    "Family photo backup to trusted friend's NAS",
).await?;
```

### **Example 2: Grant Storage Permission**
```rust
// Friend's BearDog receives request and creates approval workflow
let approval = ApprovalRecord {
    approver: "friend-user-id".to_string(),
    decision: ApprovalDecision::Approved,
    justification: "Trust this person with family photos".to_string(),
    timestamp: Utc::now(),
    signature: friend_beardog.sign_approval(&approval_data).await?,
    additional_conditions: vec![
        AccessCondition::RequireEncryption(true),
        AccessCondition::MaxConcurrentOperations(5),
        AccessCondition::RequireAuditLog(true),
    ],
};

// Generate signed authorization proof
let authorization = workflow_engine.generate_authorization_proof(
    &workflow_id,
    "your-node-id",
).await?;

// Send authorization back to requesting node
songbird.send_authorization_proof("your-node-id", authorization).await?;
```

### **Example 3: Use Authorization for Storage**
```rust
// When SongBird wants to store data on friend's NAS
let operation = CrossNodeOperation {
    operation_type: OperationType::StoreData,
    resource_id: "family-photos-2024".to_string(),
    data_size_bytes: Some(2_000_000_000), // 2GB
    estimated_duration: Some(Duration::minutes(30)),
    metadata: hashmap! {
        "content_type".to_string() => json!("image/jpeg"),
        "encryption_algorithm".to_string() => json!("aes-256-gcm"),
    },
};

// Present proof of authorization
let proof = workflow_engine.prove_authorization(
    "friend-node-id",
    &operation,
).await?;

// Friend's BearDog verifies the proof
let is_authorized = friend_workflow_engine.verify_authorization_proof(&proof).await?;

if is_authorized {
    // Proceed with storage operation
    friend_nas.store_encrypted_data(&encrypted_data).await?;
    
    // Log the operation for audit
    friend_audit_engine.log_cross_node_operation(&proof, &operation).await?;
}
```

## ⚙️ **Configuration**

### **Cross-Node Workflow Configuration**
```toml
[multi_party.cross_node]
# Cross-node workflow settings
enable_cross_node_workflows = true
default_permission_duration_days = 365
max_permission_duration_days = 1095  # 3 years
require_mutual_approval = false

# Trust establishment
auto_trust_known_nodes = false
require_manual_trust_establishment = true
trust_verification_required = true

# Authorization proof settings
proof_signature_algorithm = "ed25519"
proof_expiry_buffer_hours = 1  # How long before expiry to warn
max_concurrent_authorizations = 100

[multi_party.cross_node.default_permissions]
# Default permission limits for cross-node requests
max_storage_gb = 1000
max_compute_hours = 100
max_network_bandwidth_mbps = 100
require_encryption = true
require_audit_log = true

[multi_party.cross_node.approval_requirements]
# Default approval requirements for different permission types
storage_permissions.min_approvers = 1
storage_permissions.required_roles = ["storage_admin"]
storage_permissions.timeout_hours = 24

compute_permissions.min_approvers = 2
compute_permissions.required_roles = ["compute_admin", "security_admin"]
compute_permissions.timeout_hours = 12

emergency_access.min_approvers = 3
emergency_access.required_roles = ["security_admin", "emergency_responder"]
emergency_access.timeout_hours = 4
```

---

**Summary**: Enhanced multi-party workflows to include cross-node authorization with cryptographic proof systems. BearDog nodes can now request permissions from each other, carry signed authorization tokens, and prove authorization for cross-node operations. This enables the distributed storage and compute ecosystem while maintaining security and audit compliance. 