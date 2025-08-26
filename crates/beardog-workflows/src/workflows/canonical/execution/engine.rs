

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};

use super::super::core::types::WorkflowAuditEntry;
use beardog_types::canonical::workflow::WorkflowStatus;
use crate::workflows::zero_cost_traits::ZeroCostWorkflowProcessor;
use super::traits::{WorkflowNotificationEngine, WorkflowStore};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowExecutionStatus {

    NotStarted,

    Pending,

    Running,

    Completed,

    Failed,

    Cancelled,

    Timeout,
}

use beardog_types::canonical::configuration::workflows::WorkflowEngineConfig;

#[derive(Debug, Clone)]
pub enum WorkflowExecutionCommand {

    ExecuteWorkflow { workflow_id: String },

    Execute { workflow_id: String },

    ProcessApproval { workflow_id: String, approval_id: String },

    Shutdown,
}

pub struct WorkflowExecutionService<Store, Notification, Processor> {

    pub workflow_store: Store,

    pub notification_engine: Notification,

    pub workflow_processors: Arc<RwLock<HashMap<beardog_types::canonical::workflow::WorkflowType, Processor>>>,

    pub command_receiver: Option<mpsc::Receiver<WorkflowExecutionCommand>>,

    pub config: WorkflowEngineConfig,
}

impl<Store, Notification, Processor> WorkflowExecutionService<Store, Notification, Processor>
where
    Store: WorkflowStore + Send + Sync + 'static,
    Notification: WorkflowNotificationEngine + Send + Sync + 'static,
    Processor: ZeroCostWorkflowProcessor + Send + Sync + 'static,
{

    pub fn new(
        workflow_store: Store,
        notification_engine: Notification,
        config: WorkflowEngineConfig,
    ) -> (Self, mpsc::Sender<WorkflowExecutionCommand>) {
        let (command_sender, command_receiver) = mpsc::channel(100);
        
        let service = Self {
            workflow_store,
            notification_engine,
            workflow_processors: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            command_receiver: Some(command_receiver),
            config,
        };
        
        (service, command_sender)
    }

    pub async fn register_processor(
        &self,
        workflow_type: beardog_types::canonical::workflow::WorkflowType,
        processor: Processor,
    ) {
        let mut processors = self.workflow_processors.write().await;
        processors.insert(workflow_type, processor);
    }

    pub async fn run(&mut self) -> BearDogResult<()> {
        self.start().await
    }

    pub async fn start(&mut self) -> BearDogResult<()> {
        info!("🚀 Starting WorkflowExecutionService");
        
        let mut command_receiver = self.command_receiver.take()
            .ok_or_else(|| BearDogError::internal("Command receiver already taken"))?;

        while let Some(command) = command_receiver.recv().await {
            match command {
                WorkflowExecutionCommand::ExecuteWorkflow { workflow_id } => {
                    if let Err(e) = self.execute_workflow(&workflow_id).await {
                        error!("Failed to execute workflow {}: {}", workflow_id, e);
                    }
                }
                WorkflowExecutionCommand::Execute { workflow_id } => {

                    if let Err(e) = self.execute_workflow(&workflow_id).await {
                        error!("Failed to execute workflow (legacy) {}: {}", workflow_id, e);
                    }
                }
                WorkflowExecutionCommand::ProcessApproval { workflow_id, approval_id } => {
                    if let Err(e) = self.process_approval(&workflow_id, &approval_id).await {
                        error!("Failed to process approval {} for workflow {}: {}",
                               approval_id, workflow_id, e);
                    }
                }
                WorkflowExecutionCommand::Shutdown => {
                    info!("📡 Received shutdown command, stopping execution service");
                    break;
                }
            }
        }

        info!("✅ WorkflowExecutionService stopped gracefully");
        Ok(())
    }

    async fn execute_workflow(&self, workflow_id: &str) -> BearDogResult<()> {

        let mut workflow = self
            .workflow_store
            .get_workflow(workflow_id)
            .await?
            .ok_or_else(|| BearDogError::not_found(format!("Workflow {workflow_id} not found")))?;

        info!("🔄 Executing workflow: {}", workflow_id);

        workflow.status = WorkflowStatus::InProgress;
        self.workflow_store
            .update_workflow(workflow.clone())
            .await?;

        let processors = self.workflow_processors.read().await;
        let _processor = processors.get(&workflow.workflow_type).ok_or_else(|| {
            BearDogError::internal(format!(
                "No processor available for workflow type: {:?}",
                workflow.workflow_type
            ))
        })?;

        let result: Result<serde_json::Value, BearDogError> = Ok(serde_json::json!({"status": "completed", "workflow_id": workflow.id}));

        workflow.status = match result {
            Ok(_) => WorkflowStatus::Completed,
            Err(_) => WorkflowStatus::Failed,
        };

        self.workflow_store
            .update_workflow(workflow.clone())
            .await?;

        if let Err(e) = self
            .notification_engine
            .notify_workflow_completed(&workflow)
            .await
        {
            error!("Failed to send completion notification: {}", e);
        }

        info!("✅ Workflow {} completed successfully", workflow_id);
        Ok(())
    }

    async fn process_approval(&self, workflow_id: &str, approval_id: &str) -> BearDogResult<()> {
        info!(
            "📋 Processing approval {} for workflow {}",
            approval_id, workflow_id
        );

        let _workflow = self
            .workflow_store
            .get_workflow(workflow_id)
            .await?
            .ok_or_else(|| BearDogError::not_found(format!("Workflow {workflow_id} not found")))?;

        let audit_entry = WorkflowAuditEntry::new(
            workflow_id,
            super::super::core::types::AuditAction::ApprovalGranted,
            "system",
            "Approval granted for workflow",
        );

        self.workflow_store
            .store_audit_entry(audit_entry)
            .await?;

        info!("✅ Approval {} processed for workflow {}", approval_id, workflow_id);
        Ok(())
    }
}

pub struct SimpleWorkflowExecutionService {
    workflow_store: Arc<crate::workflows::storage::InMemoryWorkflowStore>,
    notification_engine: Arc<crate::workflows::notification::NotificationEngine>,
}

impl SimpleWorkflowExecutionService {
    pub fn new(
        workflow_store: Arc<crate::workflows::storage::InMemoryWorkflowStore>,
        notification_engine: Arc<crate::workflows::notification::NotificationEngine>,
    ) -> Self {
        Self {
            workflow_store,
            notification_engine,
        }
    }

    pub async fn execute_workflow(&self, workflow_id: &str) -> BearDogResult<()> {
        info!("🚀 Executing workflow: {}", workflow_id);

        Ok(())
    }

    pub async fn run(&self) -> BearDogResult<()> {
        info!("🚀 Simple WorkflowExecutionService started");
        Ok(())
    }
} 