//! Implementation logic and handlers for the workflow engine
//!
//! Contains the main business logic and implementation details for MultiPartyWorkflowEngine.

use super::processors::*;
use super::types::*;
use crate::{BearDogError, BearDogResult};
use tracing::error;

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

impl MultiPartyWorkflowEngine {
    /// Create a new multi-party workflow engine
    pub async fn new(
        config: Arc<crate::config::WorkflowConfig>,
        workflow_store: Arc<dyn WorkflowStore>,
        approval_store: Arc<dyn ApprovalStore>,
    ) -> BearDogResult<Self> {
        let notification_engine = Arc::new(NotificationEngine::new(NotificationConfig::default()));
        let policy_engine = Arc::new(WorkflowPolicyEngine::new(PolicyConfig::default()));
        let scheduler = Arc::new(WorkflowScheduler::default());

        let engine = Self {
            config,
            workflow_store,
            approval_store,
            notification_engine,
            policy_engine,
            scheduler,
            active_workflows: Arc::new(RwLock::new(HashMap::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
            workflow_processors: Arc::new(RwLock::new(HashMap::new())),
            execution_queue: Arc::new(tokio::sync::Mutex::new(std::collections::VecDeque::new())),
            node_registry: Arc::new(RwLock::new(HashMap::new())),
            consensus_engine: Arc::new(
                RwLock::new(crate::workflows::types::ConsensusEngine::new()),
            ),
            security_provider: Arc::new(RwLock::new(
                crate::workflows::types::WorkflowSecurityProvider::new(),
            )),
            metrics: Arc::new(RwLock::new(crate::workflows::types::WorkflowMetrics::new())),
        };

        // Register default workflow processors
        engine.register_default_processors().await?;

        Ok(engine)
    }

    /// Register default workflow processors
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
        // Generate workflow ID
        let workflow_id = Uuid::new_v4().to_string();

        // Determine approval requirements
        let approval_requirements = self
            .policy_engine
            .determine_approval_requirements(&request.workflow_type, &request.priority)
            .await?;

        // Create workflow
        let workflow = Workflow {
            id: workflow_id.clone(),
            workflow_type: request.workflow_type.clone(),
            initiator: request.initiator.clone(),
            target: request.target,
            parameters: request.parameters,
            approval_requirements: approval_requirements.clone(),
            status: WorkflowStatus::PendingApprovals,
            created_at: Utc::now(),
            expires_at: Utc::now() + approval_requirements.max_approval_time,
            approvals: Vec::new(),
            audit_trail: vec![WorkflowAuditEntry {
                timestamp: Utc::now(),
                action: WorkflowAction::Initiated,
                actor: request.initiator.clone(),
                details: request.reason,
                metadata: request.metadata,
            }],
            metadata: HashMap::new(),
        };

        // Store workflow
        self.workflow_store.store_workflow(&workflow).await?;

        // Add to active workflows
        {
            let mut active = self.active_workflows.write().await;
            active.insert(workflow_id.clone(), workflow.clone());
        }

        // Create pending approvals
        self.create_pending_approvals(&workflow).await?;

        // Send notifications
        self.notification_engine
            .notify_workflow_initiated(&workflow)
            .await?;

        // Create response
        let pending_approvers = self.get_pending_approvers(&workflow_id).await?;

        Ok(WorkflowResponse {
            workflow_id,
            status: WorkflowStatus::PendingApprovals,
            required_approvals: approval_requirements,
            pending_approvers,
            estimated_completion: workflow.expires_at,
            tracking_url: None,
        })
    }

    /// Submit an approval decision
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
                resource_type: "Workflow".to_string(),
                id: submission.workflow_id.clone(),
            })?;

        // Validate workflow is still pending
        if workflow.status != WorkflowStatus::PendingApprovals {
            return Err(BearDogError::InvalidState {
                message: "Workflow is not pending approvals".to_string(),
            });
        }

        // Validate approver is authorized
        self.validate_approver(&workflow, &submission.approver)
            .await?;

        // Create approval record
        let approval = ApprovalRecord {
            id: Uuid::new_v4().to_string(),
            workflow_id: submission.workflow_id.clone(),
            approver: submission.approver.clone(),
            approver_role: self.get_approver_role(&submission.approver).await?,
            decision: submission.decision.clone(),
            reason: submission.reason,
            timestamp: Utc::now(),
            signature: submission.signature,
            metadata: submission.metadata,
        };

        // Store approval
        self.approval_store.store_approval(&approval).await?;

        // Update workflow
        workflow.approvals.push(approval.clone());
        workflow.audit_trail.push(WorkflowAuditEntry {
            timestamp: Utc::now(),
            action: match submission.decision {
                ApprovalDecision::Approved => WorkflowAction::Approved,
                ApprovalDecision::Rejected => WorkflowAction::Rejected,
                ApprovalDecision::Delegated => WorkflowAction::Delegated,
            },
            actor: submission.approver,
            details: format!("Approval decision: {:?}", submission.decision),
            metadata: HashMap::new(),
        });

        // Check if workflow is complete
        let (new_status, remaining_approvals) = self.evaluate_workflow_status(&workflow).await?;
        workflow.status = new_status.clone();

        // Update workflow
        self.workflow_store.update_workflow(&workflow).await?;

        // Update active workflows
        {
            let mut active = self.active_workflows.write().await;
            active.insert(workflow.id.clone(), workflow.clone());
        }

        // Handle workflow completion
        if new_status == WorkflowStatus::Approved {
            self.queue_workflow_execution(&workflow).await?;
        }

        // Send notifications
        self.notification_engine
            .notify_approval_submitted(&workflow, &approval)
            .await?;

        Ok(ApprovalResponse {
            success: true,
            workflow_status: new_status,
            remaining_approvals,
            message: "Approval submitted successfully".to_string(),
        })
    }

    /// Get workflow status
    pub async fn get_workflow_status(&self, workflow_id: &str) -> BearDogResult<Option<Workflow>> {
        self.workflow_store.get_workflow(workflow_id).await
    }

    /// List pending approvals for an approver
    pub async fn list_pending_approvals(
        &self,
        approver: &str,
    ) -> BearDogResult<Vec<PendingApproval>> {
        self.approval_store.list_pending_approvals(approver).await
    }

    /// List workflows with optional status filter
    pub async fn list_workflows(
        &self,
        status: Option<WorkflowStatus>,
    ) -> BearDogResult<Vec<Workflow>> {
        self.workflow_store.list_workflows(status).await
    }

    // Private helper methods

    async fn create_pending_approvals(&self, workflow: &Workflow) -> BearDogResult<()> {
        let mut pending_approvals = Vec::new();

        for tier in &workflow.approval_requirements.approval_hierarchy {
            for user in &tier.eligible_users {
                let pending = PendingApproval {
                    id: Uuid::new_v4().to_string(),
                    workflow_id: workflow.id.clone(),
                    approver: user.clone(),
                    approver_role: self.get_approver_role(user).await.unwrap_or_default(),
                    tier_level: tier.tier_level,
                    created_at: Utc::now(),
                    expires_at: workflow.expires_at,
                    notification_sent: false,
                };
                pending_approvals.push(pending);
            }
        }

        // Store pending approvals
        {
            let mut pending = self.pending_approvals.write().await;
            pending.insert(workflow.id.clone(), pending_approvals);
        }

        Ok(())
    }

    async fn get_pending_approvers(&self, workflow_id: &str) -> BearDogResult<Vec<String>> {
        let pending = self.pending_approvals.read().await;
        if let Some(approvals) = pending.get(workflow_id) {
            Ok(approvals.iter().map(|a| a.approver.clone()).collect())
        } else {
            Ok(Vec::new())
        }
    }

    async fn validate_approver(&self, workflow: &Workflow, approver: &str) -> BearDogResult<()> {
        // Check if approver is in the eligible list
        for tier in &workflow.approval_requirements.approval_hierarchy {
            if tier.eligible_users.contains(&approver.to_string()) {
                return Ok(());
            }
        }

        Err(BearDogError::Unauthorized {
            message: "Approver not authorized for this workflow".to_string(),
        })
    }

    async fn get_approver_role(&self, _approver: &str) -> BearDogResult<String> {
        // TODO: Implement role lookup
        Ok("admin".to_string())
    }

    async fn evaluate_workflow_status(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<(WorkflowStatus, u32)> {
        let approved_count = workflow
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count() as u32;

        let rejected_count = workflow
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Rejected)
            .count() as u32;

        // Check for rejection
        if rejected_count > 0 {
            return Ok((WorkflowStatus::Rejected, 0));
        }

        // Check for approval
        if approved_count >= workflow.approval_requirements.required_approvals {
            return Ok((WorkflowStatus::Approved, 0));
        }

        // Still pending
        let remaining = workflow.approval_requirements.required_approvals - approved_count;
        Ok((WorkflowStatus::PendingApprovals, remaining))
    }

    async fn queue_workflow_execution(&self, workflow: &Workflow) -> BearDogResult<()> {
        let execution = WorkflowExecution {
            workflow_id: workflow.id.clone(),
            workflow_type: workflow.workflow_type.clone(),
            parameters: workflow.parameters.clone(),
            created_at: Utc::now(),
        };

        let mut queue = self.execution_queue.lock().await;
        queue.push_back(execution);

        Ok(())
    }

    /// Process workflow execution queue
    pub async fn process_execution_queue(&self) -> BearDogResult<()> {
        let mut queue = self.execution_queue.lock().await;

        while let Some(execution) = queue.pop_front() {
            // Drop the lock while processing
            drop(queue);

            // Process the workflow
            if let Err(e) = self.execute_workflow(&execution).await {
                error!(
                    "Failed to execute workflow {}: {}",
                    execution.workflow_id, e
                );
            }

            // Reacquire the lock
            queue = self.execution_queue.lock().await;
        }

        Ok(())
    }

    async fn execute_workflow(&self, execution: &WorkflowExecution) -> BearDogResult<()> {
        // Get processor for workflow type
        let processors = self.workflow_processors.read().await;
        let processor =
            processors
                .get(&execution.workflow_type)
                .ok_or_else(|| BearDogError::NotFound {
                    resource_type: "Workflow processor".to_string(),
                    id: execution.workflow_type.to_string(),
                })?;

        // Get workflow
        let mut workflow = self
            .workflow_store
            .get_workflow(&execution.workflow_id)
            .await?
            .ok_or_else(|| BearDogError::NotFound {
                resource_type: "Workflow".to_string(),
                id: execution.workflow_id.to_string(),
            })?;

        // Update status to in progress
        workflow.status = WorkflowStatus::InProgress;
        workflow.audit_trail.push(WorkflowAuditEntry {
            timestamp: Utc::now(),
            action: WorkflowAction::Executed,
            actor: "system".to_string(),
            details: "Workflow execution started".to_string(),
            metadata: HashMap::new(),
        });

        self.workflow_store.update_workflow(&workflow).await?;

        // Execute workflow
        let result = processor.process_workflow(&workflow).await?;

        // Update final status
        workflow.status = if result.success {
            WorkflowStatus::Completed
        } else {
            WorkflowStatus::Failed
        };

        workflow.audit_trail.push(WorkflowAuditEntry {
            timestamp: Utc::now(),
            action: if result.success {
                WorkflowAction::Completed
            } else {
                WorkflowAction::Failed
            },
            actor: "system".to_string(),
            details: result
                .error
                .unwrap_or_else(|| "Workflow completed successfully".to_string()),
            metadata: HashMap::new(),
        });

        self.workflow_store.update_workflow(&workflow).await?;

        // Send completion notification
        self.notification_engine
            .notify_workflow_completed(&workflow)
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

    /// Stop the workflow engine
    pub async fn stop(&self) -> BearDogResult<()> {
        // TODO: Implement graceful shutdown
        Ok(())
    }
}
