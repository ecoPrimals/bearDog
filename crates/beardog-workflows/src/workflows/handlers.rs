

use super::canonical::{
    ApprovalDecision, ApprovalRecord, ApprovalResponse, ApprovalStore,
    AuditAction, InMemoryApprovalStore, InMemoryWorkflowStore, Workflow, WorkflowAuditEntry,
    WorkflowEngineConfig,
    WorkflowRequest, WorkflowResponse, WorkflowStatus,
    WorkflowStore,
};

use crate::workflows::canonical::core::WorkflowContext;
use crate::workflows::canonical::execution::WorkflowExecutionCommand;
use beardog_types::configuration::WorkflowConfig;

pub struct WorkflowHandler {
    pub policy_engine: crate::workflows::canonical::WorkflowPolicyConfig,
    pub scheduler: WorkflowScheduler,
    pub metrics: crate::workflows::canonical::execution::metrics::WorkflowMetrics,
    pub workflow_processors: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<beardog_types::canonical::workflow::WorkflowType, String>>>,
    pub approval_store: crate::workflows::canonical::InMemoryApprovalStore,
    pub execution_tx: tokio::sync::mpsc::UnboundedSender<crate::workflows::canonical::execution::WorkflowExecutionCommand>,

    pub config: crate::workflows::canonical::WorkflowEngineConfig,
    pub workflow_store: std::sync::Arc<crate::workflows::canonical::InMemoryWorkflowStore>,
    pub notification_engine: std::sync::Arc<crate::workflows::notification::NotificationEngine>,
    pub active_workflows: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, crate::workflows::canonical::processing::WorkflowExecution>>>,
    pub shutdown_tx: std::sync::Arc<tokio::sync::broadcast::Sender<()>>,
}

#[derive(Debug)]
pub struct WorkflowScheduler {
    pub config: crate::workflows::canonical::WorkflowEngineConfig,
}

impl WorkflowScheduler {

    pub async fn start(&self) -> BearDogResult<()> {
        tracing::info!("Starting workflow scheduler");

        Ok(())
    }
}
use beardog_errors::{BearDogError, BearDogResult};
use crate::workflows::canonical::execution::traits::WorkflowNotificationEngine;
use crate::workflows::types::enums::ApprovalSubmission;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;
use tracing::error;

use beardog_types::canonical::configuration::workflows::WorkflowNotificationConfig as NotificationConfig;
impl Default for WorkflowHandler {
    fn default() -> WorkflowHandler {
        let config = WorkflowEngineConfig::default();
        let workflow_store = Arc::new(InMemoryWorkflowStore::new());
        let _approval_store = Arc::new(InMemoryApprovalStore::new());
        let notification_engine = Arc::new(
            crate::workflows::notification::NotificationEngine::new(NotificationConfig::default()),
        );

        let (execution_tx, __execution_rx) = tokio::sync::mpsc::unbounded_channel::<crate::workflows::canonical::execution::WorkflowExecutionCommand>();
        let (shutdown_tx, _shutdown_rx) = tokio::sync::broadcast::channel(1);
        WorkflowHandler {
            policy_engine: crate::workflows::canonical::WorkflowPolicyConfig::default(),
            scheduler: WorkflowScheduler {
                config: config.clone(),
            },
            metrics: crate::workflows::canonical::execution::metrics::WorkflowMetrics {
                processing_time_ms: 0,
                memory_usage_bytes: 0,
                cpu_usage_percent: 0.0,
                steps_executed: 0,
                error_count: 0,
                custom_metrics: std::collections::HashMap::with_capacity(16),
            },
            workflow_processors: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            approval_store: crate::workflows::canonical::InMemoryApprovalStore::new(),
            execution_tx,
            config,
            workflow_store,
            notification_engine,
            active_workflows: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            shutdown_tx: Arc::new(shutdown_tx),
        }
    }
}
impl WorkflowHandler {

    pub fn from_config(
        _config: WorkflowConfig,
        _approval_store: Arc<InMemoryApprovalStore>,
        _workflow_store: Arc<InMemoryWorkflowStore>,
        _notification_engine: Arc<crate::workflows::notification::NotificationEngine>,
    ) -> BearDogResult<Self> {

        Ok(Self::default())
    }

    pub async fn get_approver_role(&self, approver: &str) -> BearDogResult<String> {
        tracing::debug!("Looking up role for approver: {}", approver);

        let role = match approver {
            admin if admin.contains("admin") => "administrator",
            manager if manager.contains("manager") => "manager",
            security if security.contains("security") => "security_officer",
            _ => "approver",
        };
        Ok(role.to_string())
    }

    #[allow(dead_code)] // Will be used when workflow processing is fully implemented
    async fn register_default_processors(&self) -> BearDogResult<()> {
        let _processors = self.workflow_processors.write().await;

        Ok(())
    }

    pub async fn initiate_workflow(
        &self,
        request: WorkflowRequest,
    ) -> BearDogResult<WorkflowResponse> {
        let workflow_id = Uuid::new_v4().to_string();

        let approval_requirements = self
            .policy_engine
            .determine_approval_requirements(&request.workflow_type, &request.priority)?;
        let now = Utc::now();
        let timeout_duration_hours = chrono::Duration::hours(24); // Default 24 hours
        let _timeout_duration = Some(timeout_duration_hours);
        let requested_by = request.initiator.clone();
        let workflow_priority = request.priority.clone();

        let workflow = Workflow {
            id: workflow_id.clone(),
            workflow_type: request.workflow_type.clone(),
            status: beardog_types::canonical::workflow::WorkflowStatus::PendingApprovals,
            created_at: now,
            updated_at: now,
            approval_requirements: Some(beardog_types::canonical::configuration::workflows::ApprovalRequirements {
                tier: beardog_types::canonical::configuration::workflows::ApprovalTier::Basic,
                required_count: approval_requirements.len() as u32,
                required_roles: approval_requirements,
                parallel_approvals: false,
                timeout: std::time::Duration::from_secs(86400), // 24 hours
                auto_approval_rules: Vec::new(),
            }),
            audit_trail: Vec::new(),
            metadata: HashMap::with_capacity(16),
            target: request.parameters.get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string(),
            priority: request.priority,
            expires_at: Some(now + chrono::Duration::hours(24)),
            requested_by: requested_by.clone(),
            initiator: request.initiator.clone(),
            description: Some(request.description),
            timeout_duration: Some(Duration::from_secs(24 * 3600)),
            properties: request.properties,
            parameters: HashMap::with_capacity(16), // Initialize empty parameters - will be populated from properties
            context: WorkflowContext {
                initiated_by: request.initiator.clone(),
                target: request.parameters.get("target")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string(),
                data: request.parameters.clone(),
            },
            approvals: Vec::new(),
        };

        self.workflow_store.store_workflow(workflow.clone()).await?;

        {
            let mut active = self.active_workflows.write().await;

            let workflow_execution = crate::workflows::canonical::processing::WorkflowExecution {
                execution_id: workflow_id.clone(),
                workflow: workflow.clone(),
                processor: "default".to_string(),
                state: crate::workflows::canonical::processing::WorkflowExecutionStatus::Pending,
                result: None,
                started_at: chrono::Utc::now(),
                completed_at: None,
            };
            active.insert(workflow_id.clone(), workflow_execution);
        }

        self.notification_engine
            .notify_workflow_created(&workflow)
            .await?;

        let audit_entry = WorkflowAuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: now,
            workflow_id: workflow_id.clone(),
            user: requested_by.clone(),
            actor: requested_by.clone(),
            actor_ip: Some("127.0.0.1".to_string()),
            user_agent: Some("beardog-workflows".to_string()),
            event_type: "workflow_initiated".to_string(),
            action: AuditAction::WorkflowCreated,
            description: format!("Workflow {workflow_id} submitted for processing"),
            context: {
                let mut details = HashMap::with_capacity(16);
                details.insert(
                    "workflow_type".to_string(),
                    serde_json::json!(request.workflow_type),
                );
                details.insert("priority".to_string(), serde_json::json!(workflow_priority));
                details.insert(
                    "description".to_string(),
                    serde_json::json!(workflow.description),
                );
                details
            },
            metadata: HashMap::with_capacity(16),
            result: Some("workflow_created".to_string()),
        };

        let mut updated_workflow = workflow;
        updated_workflow.audit_trail.push(audit_entry);
        self.workflow_store
            .update_workflow(updated_workflow)
            .await?;

        Ok(WorkflowResponse {
            data: {
                let mut data = HashMap::with_capacity(16);
                data.insert("workflow_id".to_string(), serde_json::json!(workflow_id.clone()));
                data
            },
            success: true,
            message: "Workflow initiated successfully".to_string(),
            workflow_id: Some(workflow_id),
            estimated_completion: Some(now + chrono::Duration::hours(24)),
            metadata: HashMap::with_capacity(16),
        })
    }

    pub async fn submit_workflow(
        &self,
        request: WorkflowRequest,
        requested_by: &str,
    ) -> BearDogResult<WorkflowResponse> {
        let workflow_id = uuid::Uuid::new_v4().to_string();
        let _timeout_duration_hours = chrono::Duration::hours(24);
        let _workflow_priority = request.priority.clone();

        let canonical_workflow = crate::workflows::canonical::CanonicalWorkflow {
            id: workflow_id.clone(),
            workflow_type: request.workflow_type,
            status: WorkflowStatus::PendingApprovals,
            priority: request.priority,
            description: Some(request.description),
            target: "default".to_string(),
            initiator: requested_by.to_string(),
            requested_by: requested_by.to_string(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
            timeout_duration: Some(std::time::Duration::from_secs(3600)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: std::collections::HashMap::with_capacity(16),
            parameters: std::collections::HashMap::with_capacity(16),
            properties: std::collections::HashMap::with_capacity(16),
            context: crate::workflows::canonical::core::types::WorkflowContext {
                initiated_by: requested_by.to_string(),
                target: "default".to_string(),
                data: std::collections::HashMap::with_capacity(16),
            },
            approval_requirements: None,
            approvals: Vec::new(),
            audit_trail: Vec::new(),
        };

        let workflow = crate::workflows::canonical::processing::WorkflowExecution {
            execution_id: workflow_id.clone(),
            workflow: canonical_workflow,
            processor: "default".to_string(),
            state: crate::workflows::canonical::processing::WorkflowExecutionStatus::Pending,
            result: None,
            started_at: Utc::now(),
            completed_at: None,
        };
        
        self.active_workflows
            .write()
            .await
            .insert(workflow_id.clone(), workflow);
            
        let audit_entry = WorkflowAuditEntry::new(
            &workflow_id.clone(),
            AuditAction::WorkflowSubmitted,
            requested_by,
            "Workflow submitted for processing",
        );

        self.workflow_store.store_audit_entry(audit_entry).await?;
        
        Ok(WorkflowResponse {
            data: HashMap::with_capacity(16),
            success: true,
            message: "Workflow submitted successfully".to_string(),
            workflow_id: Some(workflow_id),
            estimated_completion: Some(Utc::now() + chrono::Duration::hours(24)),
            metadata: HashMap::with_capacity(16),
        })
    }

    pub async fn submit_approval(
        &self,
        submission: ApprovalSubmission,
    ) -> BearDogResult<ApprovalResponse> {

        let mut workflow = self
            .workflow_store
            .get_workflow(&submission.workflow_id)
            .await?
            .ok_or_else(|| {
                BearDogError::not_found(format_args!("Workflow {} not found", submission.workflow_id).to_string())
            })?;

        if workflow.status != WorkflowStatus::PendingApprovals {
            return Err(BearDogError::validation(format!(
                "Workflow {} is not pending approvals (status: {:?})",
                submission.workflow_id, workflow.status
            )));
        }

        let mut approval = ApprovalRecord::new(
            &submission.workflow_id,
            &submission.approver_id,
            crate::workflows::canonical::configuration::ApprovalDecision::Approved,
            submission.comments.as_deref(),
        );

        approval.signature = Some(submission.signature);
        // Note: 'comments' and 'reason' fields don't exist in ApprovalRecord
        // The message field is used for comments
        if !submission.reason.is_empty() {
            // Use the reason directly since it's already a String
            tracing::info!("Workflow submission reason: {}", submission.reason);
            approval.response = Some(submission.reason);
        }

        self.approval_store.store_approval(approval.clone()).await?;

        workflow.approvals.push(approval.clone());

        let should_execute = self.should_execute_workflow(&workflow).await?;
        if should_execute {
            workflow.status = WorkflowStatus::Approved;

            if let Err(e) = self.execution_tx.send(WorkflowExecutionCommand::Execute {
                workflow_id: workflow.id.clone(),
            }) {
                error!("Failed to send execution command: {}", e);
                return Err(BearDogError::internal(format!(
                    "Failed to queue workflow execution: {e}"
                )));
            }
        }

        self.workflow_store.update_workflow(workflow.clone()).await?;
        Ok(ApprovalResponse {
            response_id: uuid::Uuid::new_v4().to_string(),
            approval_id: approval.id,
            status: crate::workflows::canonical::configuration::ApprovalDecision::Approved,
            workflow_status: workflow.status,
            timestamp: approval.requested_at,
        })
    }

    async fn should_execute_workflow(&self, workflow: &Workflow) -> BearDogResult<bool> {
        let approved_count = workflow
            .approvals
            .iter()
            .filter(|a| a.decision == ApprovalDecision::Approved)
            .count() as u32;
        Ok(approved_count
            >= workflow
                .approval_requirements
                .as_ref()
                .map_or(1, |req| req.required_count))
    }

    pub async fn start(&mut self) -> BearDogResult<tokio::task::JoinHandle<()>> {

        self.scheduler.start().await?;

        let (_execution_tx, _execution_rx) = tokio::sync::mpsc::unbounded_channel::<crate::workflows::canonical::execution::WorkflowExecutionCommand>();
        let _shutdown_rx = self.shutdown_tx.subscribe();

        let execution_service = crate::workflows::canonical::execution::engine::SimpleWorkflowExecutionService::new(
            Arc::clone(&self.workflow_store),
            Arc::clone(&self.notification_engine),
        );

        let service_handle = tokio::spawn(async move {
            let _ = execution_service.run().await;
        });
        Ok(service_handle)
    }

    pub async fn stop(&self) -> BearDogResult<()> {

        if let Err(e) = self.shutdown_tx.send(()) {
            error!("Failed to send shutdown signal: {}", e);
        }

        if let Err(e) = self.execution_tx.send(WorkflowExecutionCommand::Shutdown) {
            error!("Failed to send shutdown command: {}", e);
            return Err(BearDogError::internal(format!("Failed to send shutdown command: {e}")));
        }
        
        Ok(())
    }
}
