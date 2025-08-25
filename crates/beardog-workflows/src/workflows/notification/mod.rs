// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Workflow Notification System
///
/// **MODERNIZED NOTIFICATION ARCHITECTURE** ✅
/// This module provides unified notification capabilities for workflow events
/// with support for multiple channels and formats.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::configuration::workflows::WorkflowNotificationConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;
use tracing::{debug, info, warn};

/// Notification message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationMessage {
    /// Unique message identifier
    pub id: String,
    /// Message title
    pub title: String,
    /// Message subject (for email-like notifications)
    pub subject: String,
    /// Message content/body
    pub content: String,
    /// Recipients list
    pub recipients: Vec<String>,
    /// Message priority
    pub priority: NotificationPriority,
    /// Message format
    pub format: MessageFormat,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Supported message formats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum MessageFormat {
    /// Plain text format
    #[default]
    Text,
    /// HTML format
    Html,
    /// Markdown format
    Markdown,
    /// JSON format
    Json,
}

/// Notification priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum NotificationPriority {
    /// Low priority notifications
    Low,
    /// Normal priority notifications
    #[default]
    Normal,
    /// High priority notifications
    High,
    /// Critical priority notifications
    Critical,
}

/// Notification delivery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    /// Whether the notification was sent successfully
    pub success: bool,
    /// Delivery status message
    pub message: String,
    /// Delivery timestamp
    pub timestamp: u64,
    /// Delivery channel used
    pub channel: String,
}

/// Adapter capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterCapabilities {
    /// Supported message formats
    pub formats: Vec<MessageFormat>,
    /// Maximum message size
    pub max_size: usize,
    /// Whether attachments are supported
    pub supports_attachments: bool,
    /// Rate limiting information
    pub rate_limits: HashMap<String, u64>,
}

/// Main notification engine
pub struct NotificationEngine {
    /// Engine configuration
    config: WorkflowNotificationConfig,
    /// Active adapters
    adapters: HashMap<String, Box<dyn NotificationAdapter>>,
    /// Message queue for async processing
    message_queue: tokio::sync::mpsc::UnboundedSender<NotificationMessage>,
}

/// Notification adapter trait
pub trait NotificationAdapter: Send + Sync {
    /// Send a notification message
    fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> Pin<Box<dyn std::future::Future<Output = BearDogResult<NotificationResult>> + Send>>;
    
    /// Get adapter capabilities
    fn get_capabilities(&self) -> AdapterCapabilities;
    
    /// Get adapter name
    fn name(&self) -> &str;
}

impl NotificationEngine {
    /// Create a new notification engine
    pub fn new(config: WorkflowNotificationConfig) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<NotificationMessage>();
        
        // Spawn background task for message processing
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                debug!("Processing notification message: {}", message.id);
                // Process message in background
            }
        });
        
        Self {
            config,
            adapters: HashMap::new(),
            message_queue: tx,
        }
    }
    
    /// Add a notification adapter
    pub fn add_adapter(&mut self, name: String, adapter: Box<dyn NotificationAdapter>) {
        info!("Adding notification adapter: {}", name);
        self.adapters.insert(name, adapter);
    }
    
    /// Send a notification
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
    
    /// Queue a notification for async processing
    pub fn queue_notification(&self, message: NotificationMessage) -> BearDogResult<()> {
        self.message_queue.send(message)
            .map_err(|e| BearDogError::system(format!("Failed to queue notification: {e}")))?;
        Ok(())
    }
}



/// Convenience functions for common notification scenarios
impl NotificationEngine {
    /// Notify about workflow completion
    pub async fn notify_workflow_completed(
        &self,
        workflow: &crate::workflows::canonical::Workflow,
    ) -> BearDogResult<Vec<NotificationResult>> {
        let message = NotificationMessage {
            id: format!("workflow_completed_{}", workflow.id),
            title: "Workflow Completed".to_string(),
            subject: "Workflow Completed".to_string(),
            content: format!(
                "Workflow {} has been completed with status: {}",
                workflow.id, workflow.status
            ),
            recipients: vec![],
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: HashMap::new(),
        };
        
        self.send_notification(message).await
    }
    
    /// Notify about approval submission
    pub async fn notify_approval_submitted(
        &self,
        approval: &crate::workflows::canonical::ApprovalRecord,
        workflow: &crate::workflows::canonical::Workflow,
    ) -> BearDogResult<Vec<NotificationResult>> {
        let message = NotificationMessage {
            id: format!("approval_submitted_{}", approval.id),
            title: "Approval Submitted".to_string(),
            subject: "Approval Submitted".to_string(),
            content: format!(
                "Approval {:?} submitted for workflow {}",
                approval.decision, workflow.id
            ),
            recipients: vec![],
            priority: NotificationPriority::High,
            format: MessageFormat::Text,
            metadata: HashMap::new(),
        };
        
        self.send_notification(message).await
    }
}

// Implement WorkflowNotificationEngine for NotificationEngine
impl crate::workflows::canonical::execution::traits::WorkflowNotificationEngine for NotificationEngine {
    async fn notify_workflow_created(&self, workflow: &crate::workflows::canonical::CanonicalWorkflow) -> beardog_errors::BearDogResult<()> {
        let message = NotificationMessage {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Workflow Created".to_string(),
            subject: format!("Workflow {} Created", workflow.id),
            content: format!("Workflow {} has been created successfully", workflow.id),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::new(),
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
            subject: format!("Workflow {} Completed", workflow.id),
            content: format!("Workflow {} has been completed successfully", workflow.id),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::new(),
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
            subject: format!("Workflow {} Failed", workflow.id),
            content: format!("Workflow {} has failed: {}", workflow.id, error),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::High,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::new(),
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
            subject: format!("Approval Required for Workflow {}", workflow.id),
            content: format!("Workflow {} requires approval to proceed", workflow.id),
            recipients: vec![], // Would be populated based on approval requirements
            priority: NotificationPriority::High,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::new(),
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
            subject: format!("Workflow {} Approved", workflow.id),
            content: format!("Workflow {} has been approved by {}", workflow.id, approver),
            recipients: vec![], // Would be populated based on workflow configuration
            priority: NotificationPriority::Normal,
            format: MessageFormat::Text,
            metadata: std::collections::HashMap::new(),
        };
        
        self.message_queue.send(message).map_err(|e| {
            beardog_errors::BearDogError::internal(format!("Failed to queue approval granted notification: {e}"))
        })?;
        
        Ok(())
    }
}

// Implement WorkflowNotificationEngine for Arc<NotificationEngine> to enable shared ownership
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
