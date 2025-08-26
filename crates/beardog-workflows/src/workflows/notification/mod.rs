

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::configuration::workflows::WorkflowNotificationConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMessage {

    pub id: String,

    pub title: String,

    pub subject: String,

    pub content: String,

    pub recipients: Vec<String>,

    pub priority: NotificationPriority,

    pub format: MessageFormat,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum MessageFormat {

    #[default]
    Text,

    Html,

    Markdown,

    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum NotificationPriority {

    Low,

    #[default]
    Normal,

    High,

    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {

    pub success: bool,

    pub message: String,

    pub timestamp: u64,

    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterCapabilities {

    pub formats: Vec<MessageFormat>,

    pub max_size: usize,

    pub supports_attachments: bool,

    pub rate_limits: HashMap<String, u64>,
}

pub struct NotificationEngine {

    config: WorkflowNotificationConfig,

    adapters: HashMap<String, Box<dyn NotificationAdapter>>,

    message_queue: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
}

pub trait NotificationAdapter: Send + Sync {

    fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> Pin<Box<dyn std::future::Future<Output = BearDogResult<NotificationResult>> + Send>>;

    fn get_capabilities(&self) -> AdapterCapabilities;

    fn name(&self) -> &str;
}

impl NotificationEngine {

    pub fn new(config: WorkflowNotificationConfig) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NotificationMessage>();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                debug!("Processing notification message: {}", message.id);

            }
        });
        
        Self {
            config,
            adapters: HashMap::with_capacity(16),
            message_queue: tx,
        }
    }

    pub fn add_adapter(&mut self, name: &str, adapter: Box<dyn NotificationAdapter>) {
        self.adapters.insert(name.to_string(), adapter);
    }

    pub async fn send_notification(&self, message: NotificationMessage) -> BearDogResult<Vec<NotificationResult>> {
        let mut results = Vec::new();
        
        for (name, adapter) in &self.adapters {
            match adapter.send_notification(&message).await {
                Ok(result) => {
                    info!("Notification sent via {}: {}", name, result.message);
                    results.push(result);
                }
                Err(e) => {
                    warn!("Failed to send notification via {}: {}", name, e);
                    results.push(NotificationResult {
                        success: false,
                        message: format!("Failed: {e}"),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        channel: name.clone(),
                    });
                }
            }
        }
        
        Ok(results)
    }

    pub fn queue_notification(&self, message: NotificationMessage) -> BearDogResult<()> {
        self.message_queue.send(message)
            .map_err(|e| BearDogError::system(format!("Failed to queue notification: {e}")))?;
        Ok(())
    }
}

impl NotificationEngine {

    pub async fn notify_workflow_completed(
        &self,
        workflow: &crate::workflows::canonical::Workflow,
    ) -> BearDogResult<Vec<NotificationResult>> {
        let message = NotificationMessage {
            id: format_args!("workflow_completed_{}", workflow.id).to_string(),
            title: "Workflow Completed".to_string(),
            subject: "Workflow Completed".to_string(),
            content: format!(
                "Workflow {} has been completed with status: {}",
                workflow.id, workflow.status
            ),
            recipients: vec![],
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: HashMap::with_capacity(16),
        };
        
        self.send_notification(message).await
    }

    pub async fn notify_approval_submitted(
        &self,
        approval: &crate::workflows::canonical::ApprovalRecord,
        workflow: &crate::workflows::canonical::Workflow,
    ) -> BearDogResult<Vec<NotificationResult>> {
        let message = NotificationMessage {
            id: format_args!("approval_submitted_{}", approval.id).to_string(),
            title: "Approval Submitted".to_string(),
            subject: "Approval Submitted".to_string(),
            content: format!(
                "Approval {:?} submitted for workflow {}",
                approval.decision, workflow.id
            ),
            recipients: vec![],
            priority: NotificationPriority::High,
            format: MessageFormat::Text,
            metadata: HashMap::with_capacity(16),
        };
        
        self.send_notification(message).await
    }
}

impl crate::workflows::canonical::execution::traits::WorkflowNotificationEngine for NotificationEngine {
    async fn notify_workflow_created(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        let message = NotificationMessage {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Workflow Created".to_string(),
            subject: format_args!("Workflow {} Created", workflow.id).to_string(),
            content: format_args!("Workflow {} has been created successfully", workflow.id).to_string(),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        self.message_queue.send(message).map_err(|e| {
            beardog_errors::BearDogError::internal(format!("Failed to queue workflow created notification: {e}"))
        })?;
        
        Ok(())
    }

    async fn notify_workflow_completed(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        let message = NotificationMessage {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Workflow Completed".to_string(),
            subject: format_args!("Workflow {} Completed", workflow.id).to_string(),
            content: format_args!("Workflow {} has been completed successfully", workflow.id).to_string(),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        self.message_queue.send(message).map_err(|e| {
            beardog_errors::BearDogError::internal(format!("Failed to queue workflow completed notification: {e}"))
        })?;
        
        Ok(())
    }

    async fn notify_workflow_failed(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow, error: &str) -> beardog_errors::BearDogResult<()> {
        let message = NotificationMessage {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Workflow Failed".to_string(),
            subject: format_args!("Workflow {} Failed", workflow.id).to_string(),
            content: format_args!("Workflow {} has failed: {}", workflow.id, error).to_string(),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::High,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        self.message_queue.send(message).map_err(|e| {
            beardog_errors::BearDogError::internal(format!("Failed to queue workflow failed notification: {e}"))
        })?;
        
        Ok(())
    }

    async fn notify_approval_required(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        let message = NotificationMessage {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Approval Required".to_string(),
            subject: format_args!("Approval Required for Workflow {}", workflow.id).to_string(),
            content: format_args!("Workflow {} requires approval to proceed", workflow.id).to_string(),
            recipients: vec![], // Would be populated based on approval requirements
            priority: NotificationPriority::High,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        self.message_queue.send(message).map_err(|e| {
            beardog_errors::BearDogError::internal(format!("Failed to queue approval required notification: {e}"))
        })?;
        
        Ok(())
    }

    async fn notify_approval_granted(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow, approver: &str) -> beardog_errors::BearDogResult<()> {
        let message = NotificationMessage {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Approval Granted".to_string(),
            subject: format_args!("Workflow {} Approved", workflow.id).to_string(),
            content: format_args!("Workflow {} has been approved by {}", workflow.id, approver).to_string(),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::with_capacity(16),
        };
        
        self.message_queue.send(message).map_err(|e| {
            beardog_errors::BearDogError::internal(format!("Failed to queue approval granted notification: {e}"))
        })?;
        
        Ok(())
    }
}

impl crate::workflows::canonical::execution::traits::WorkflowNotificationEngine for std::sync::Arc<NotificationEngine> {
    async fn notify_workflow_created(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        (**self).notify_workflow_created(workflow).await
    }

    async fn notify_workflow_completed(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        (**self).notify_workflow_completed(workflow).await?;
        Ok(())
    }

    async fn notify_workflow_failed(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow, error: &str) -> beardog_errors::BearDogResult<()> {
        (**self).notify_workflow_failed(workflow, error).await
    }

    async fn notify_approval_required(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        (**self).notify_approval_required(workflow).await
    }

    async fn notify_approval_granted(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow, approver: &str) -> beardog_errors::BearDogResult<()> {
        (**self).notify_approval_granted(workflow, approver).await
    }
}

impl std::fmt::Debug for NotificationEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationEngine")
            .field("config", &self.config)
            .field("message_queue", &"<message_queue>")
            .field("adapters", &"<adapters>")
            .finish()
    }
}
