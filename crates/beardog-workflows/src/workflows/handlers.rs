//! Implementation logic and handlers for the workflow engine
//!
//! Contains the main business logic and implementation details for MultiPartyWorkflowEngine.

use super::notification::{NotificationEngine, NotificationMessage, NotificationPriority, MessageFormat};
use super::processors::*;
use super::types::*;
use beardog_config::integration::WorkflowConfig;
use beardog_errors::{BearDogError, BearDogResult};
use tracing::error;

use chrono::Utc;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use uuid::Uuid;

impl MultiPartyWorkflowEngine {
    /// Create a new multi-party workflow engine
    pub fn new(
        config: WorkflowConfig,
        approval_store: Arc<dyn ApprovalStore>,
        workflow_store: Arc<dyn WorkflowStore>,
        notification_engine: Arc<NotificationEngine>,
    ) -> Self {
        Self {
            config: Arc::new(config),
            approval_store,
            workflow_store,
            notification_engine,
            policy_engine: Arc::new(WorkflowPolicyEngine {
                config: PolicyConfig::default(),
            }),
            scheduler: Arc::new(WorkflowScheduler {
                cleanup_enabled: true,
                workflows: Arc::new(RwLock::new(HashMap::new())),
                policy_config: Arc::new(PolicyConfig::default()),
                cleanup_task_handle: Arc::new(RwLock::new(None)),
            }),
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
            workflow_processors: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(Mutex::new(VecDeque::new())),
            node_registry: Arc::new(RwLock::new(HashMap::new())),
            consensus_engine: Arc::new(RwLock::new(ConsensusEngine::default())),
            security_provider: Arc::new(RwLock::new(WorkflowSecurityProvider::default())),
            metrics: Arc::new(RwLock::new(WorkflowMetrics::default())),
        }
    }

    /// Get approver role - implemented for workflow approval system
    pub async fn get_approver_role(&self, approver: &str) -> BearDogResult<String> {
        tracing::debug!("Looking up role for approver: {}", approver);

        // Default role mapping - could be extended with database lookup
        let role = match approver {
            admin if admin.contains("admin") => "administrator",
            manager if manager.contains("manager") => "manager",
            security if security.contains("security") => "security_officer",
            _ => "approver",
        };

        Ok(role.to_string())
    }

    /// Register default workflow processors
    #[allow(dead_code)] // Will be used when workflow processing is fully implemented
    async fn register_default_processors(&self) -> BearDogResult<()> {
        let mut processors = self.workflow_processors.write().await;

        processors.insert(WorkflowType::KeyRotation, Box::new(KeyRotationProcessor));
        processors.insert(WorkflowType::KeyDeletion, Box::new(KeyDeletionProcessor));
        processors.insert(WorkflowType::PolicyChange, Box::new(PolicyChangeProcessor));
        processors.insert(
            WorkflowType::ConfigurationChange,
            Box::new(ConfigChangeProcessor),
        );
        processors.insert(
            WorkflowType::UserProvisioning,
            Box::new(UserProvisioningProcessor),
        );
        processors.insert(
            WorkflowType::EmergencyAccess,
            Box::new(EmergencyAccessProcessor),
        );
        processors.insert(
            WorkflowType::SystemMaintenance,
            Box::new(SystemMaintenanceProcessor),
        );
        processors.insert(
            WorkflowType::ComplianceAudit,
            Box::new(ComplianceAuditProcessor),
        );

        Ok(())
    }

    /// Initiate a new workflow
    pub async fn initiate_workflow(
        &self,
        request: WorkflowRequest,
    ) -> BearDogResult<WorkflowResponse> {
        let workflow_id = Uuid::new_v4().to_string();

        // Determine approval requirements based on workflow type and priority
        let approval_requirements = self
            .policy_engine
            .determine_approval_requirements(&request.workflow_type, &request.priority)
            .await?;

        let now = Utc::now();
        let timeout_duration_hours = chrono::Duration::hours(
            self.policy_engine.config.default_approval_timeout_hours as i64,
        );
        let timeout_duration = Some(timeout_duration_hours);

        let requested_by = request.requested_by.clone();
        let workflow_priority = request.priority.clone();

        // Create workflow with all required fields
        let workflow = Workflow {
            id: workflow_id.clone(),
            workflow_type: request.workflow_type.clone(),
            status: WorkflowStatus::PendingApprovals,
            target: request.target,
            approval_requirements,
            priority: request.priority,
            created_at: now,
            expires_at: now + timeout_duration_hours,
            requested_by: requested_by.clone(),
            initiator: request.initiator,
            description: request.description,
            metadata: HashMap::new(),
            timeout_duration,
            properties: request.properties,
            parameters: HashMap::new(), // Initialize empty parameters - will be populated from properties
            approvals: Vec::new(),
            audit_trail: Vec::new(),
        };

        // Store workflow
        self.workflow_store.store_workflow(&workflow).await?;

        // Add to active workflows
        {
            let mut active = self.active_workflows.write().await;
            active.insert(workflow_id.clone(), workflow.clone());
        }

        // Send initiation notification
        self.notification_engine
            .notify_workflow_initiated(&workflow)
            .await?;

        // Create audit entry
        let audit_entry = WorkflowAuditEntry {
            id: Uuid::new_v4().to_string(),
            workflow_id: workflow_id.clone(),
            event_type: "workflow_initiated".to_string(),
            user_id: requested_by,
            timestamp: now,
            details: serde_json::json!({
                "workflow_type": request.workflow_type,
                "priority": workflow_priority,
                "description": workflow.description
            }),
        };

        // Add audit entry to workflow
        let mut updated_workflow = workflow;
        updated_workflow.audit_trail.push(audit_entry);
        self.workflow_store
            .update_workflow(&updated_workflow)
            .await?;

        Ok(WorkflowResponse {
            workflow_id,
            success: true,
            message: "Workflow initiated successfully".to_string(),
            status: WorkflowStatus::PendingApprovals,
            data: None,
        })
    }

    /// Submit an approval decision for a workflow
    pub async fn submit_approval(
        &self,
        submission: ApprovalSubmission,
    ) -> BearDogResult<ApprovalResponse> {
        // Get workflow
        let mut workflow = self
            .workflow_store
            .get_workflow(&submission.workflow_id)
            .await?
            .ok_or_else(|| BearDogError::NotFound {
                message: format!("Workflow {} not found", submission.workflow_id),
            })?;

        // Validate workflow is in pending state
        if workflow.status != WorkflowStatus::PendingApprovals {
            return Err(BearDogError::WorkflowInvalidState(format!(
                "Workflow {} is not pending approvals (status: {:?})",
                submission.workflow_id, workflow.status
            )));
        }

        // Create approval record
        let approval = ApprovalRecord {
            id: Uuid::new_v4().to_string(),
            workflow_id: submission.workflow_id.clone(),
            approver_id: submission.approver_id.clone(),
            decision: submission.decision.clone(),
            reason: submission.reason.clone(),
            decided_at: Utc::now(),
            signature: submission.signature,
            approver_ip: None, // Could be populated from request context
        };

        // Store approval
        self.approval_store.store_approval(&approval).await?;

        // Add approval to workflow
        workflow.approvals.push(approval.clone());

        // Check if workflow should proceed to execution
        let should_execute = self.should_execute_workflow(&workflow).await?;

        if should_execute {
            workflow.status = WorkflowStatus::Approved;

            // Add to execution queue
            let execution = WorkflowExecution {
                workflow: workflow.clone(),
                state: WorkflowExecutionState::NotStarted,
                started_at: Utc::now(),
            };

            {
                let mut queue = self.execution_queue.lock().await;
                queue.push_back(execution);
            }
        }

        // Update workflow
        self.workflow_store.update_workflow(&workflow).await?;

        // Send notification
        self.notification_engine
            .notify_approval_submitted(&workflow, &approval)
            .await?;

        Ok(ApprovalResponse {
            success: true,
            message: format!("Approval {} submitted successfully", approval.decision),
            workflow_status: workflow.status,
        })
    }

    /// Check if workflow has enough approvals to proceed
    async fn should_execute_workflow(&self, workflow: &Workflow) -> BearDogResult<bool> {
        let approved_count = workflow
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count() as u32;

        Ok(approved_count >= workflow.approval_requirements.minimum_approvals)
    }

    /// Process execution queue
    async fn process_execution_queue(&self) -> BearDogResult<()> {
        let execution = {
            let mut queue = self.execution_queue.lock().await;
            queue.pop_front()
        };

        if let Some(mut exec) = execution {
            self.execute_workflow(&mut exec.workflow).await?;
        }

        Ok(())
    }

    /// Execute a workflow using the appropriate processor
    async fn execute_workflow(&self, workflow: &mut Workflow) -> BearDogResult<()> {
        workflow.status = WorkflowStatus::InProgress;
        self.workflow_store.update_workflow(workflow).await?;

        // Get processor for workflow type
        let processors = self.workflow_processors.read().await;
        let processor = processors.get(&workflow.workflow_type).ok_or_else(|| {
            BearDogError::NotImplemented {
                message: format!(
                    "No processor available for workflow type: {:?}",
                    workflow.workflow_type
                ),
            }
        })?;

        self.workflow_store.update_workflow(workflow).await?;

        // Execute workflow
        let result = processor.process_workflow(workflow).await?;

        // Update final status
        workflow.status = if result.success {
            WorkflowStatus::Completed
        } else {
            WorkflowStatus::Failed
        };

        workflow.audit_trail.push(WorkflowAuditEntry {
            id: Uuid::new_v4().to_string(),
            workflow_id: workflow.id.clone(),
            event_type: if result.success {
                "workflow_completed".to_string()
            } else {
                "workflow_failed".to_string()
            },
            user_id: "system".to_string(),
            timestamp: Utc::now(),
            details: serde_json::json!({
                "success": result.success,
                "message": result.message,
                "execution_duration_ms": result.execution_duration_ms
            }),
        });

        self.workflow_store.update_workflow(workflow).await?;

        // Send completion notification
        self.notification_engine
            .notify_workflow_completed(workflow)
            .await?;

        Ok(())
    }

    /// Start the workflow engine background tasks
    pub async fn start(&self) -> BearDogResult<()> {
        // Start scheduler
        self.scheduler.start().await?;

        // Start execution queue processor
        tokio::spawn({
            let engine = self.clone();
            async move {
                loop {
                    if let Err(e) = engine.process_execution_queue().await {
                        error!("Error processing execution queue: {e}");
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        });

        Ok(())
    }

    /// Stop the workflow engine gracefully
    pub async fn stop(&self) -> BearDogResult<()> {
        tracing::info!("🛑 Initiating graceful shutdown of workflow engine");

        // 1. Stop the scheduler to prevent new workflows from being processed
        {
            let mut scheduler = self.scheduler.workflows.write().await;
            if let Some(handle) = self.scheduler.cleanup_task_handle.write().await.take() {
                handle.abort();
                tracing::debug!("📝 Scheduler cleanup task stopped");
            }
        }

        // 2. Wait for active workflows to complete (with timeout)
        let active_workflows = self.active_workflows.read().await;
        let active_count = active_workflows.len();
        
        if active_count > 0 {
            tracing::info!("⏳ Waiting for {} active workflows to complete", active_count);
            
            // Give workflows up to 30 seconds to complete gracefully
            let mut attempts = 0;
            const MAX_ATTEMPTS: u32 = 30;
            
            while attempts < MAX_ATTEMPTS {
                let current_count = self.active_workflows.read().await.len();
                if current_count == 0 {
                    break;
                }
                tracing::debug!("⏳ {} workflows still active, waiting...", current_count);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                attempts += 1;
            }
            
            let final_count = self.active_workflows.read().await.len();
            if final_count > 0 {
                tracing::warn!("⚠️ {} workflows did not complete within timeout", final_count);
            } else {
                tracing::info!("✅ All active workflows completed successfully");
            }
        }

        // 3. Cancel pending approvals and notify participants
        {
            let mut pending = self.pending_approvals.write().await;
            for (workflow_id, approvals) in pending.drain() {
                tracing::debug!("📝 Cancelling pending approvals for workflow {}", workflow_id);
                
                // Notify participants about shutdown
                for approval in approvals {
                    let notification = NotificationMessage {
                        title: "Workflow System Shutdown".to_string(),
                        content: format!("Workflow {} has been cancelled due to system shutdown", workflow_id),
                        priority: NotificationPriority::High,
                        recipients: vec![approval.approver_id.clone()],
                        metadata: std::collections::HashMap::new(),
                        format: MessageFormat::PlainText,
                    };
                    
                    if let Err(e) = self.notification_engine.send_universal_notification(notification).await {
                        tracing::warn!("Failed to send shutdown notification: {}", e);
                    }
                }
            }
        }

        // 4. Clear remaining resources
        {
            self.workflow_processors.write().await.clear();
            self.execution_queue.lock().await.clear();
            self.node_registry.write().await.clear();
        }

        // 5. Update metrics - increment total workflows as completed
        {
            let mut metrics = self.metrics.write().await;
            // Note: No specific shutdown fields available, just update general metrics
            tracing::debug!("📊 Workflow engine metrics at shutdown: {} total, {} pending", 
                          metrics.total_workflows, metrics.pending_workflows);
        }

        tracing::info!("✅ Workflow engine shutdown completed successfully");
        Ok(())
    }
}
