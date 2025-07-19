//! Core notification engine
//!
//! This module provides the main NotificationEngine that orchestrates
//! notifications across multiple channels (webhook, email, SMS, Slack, Teams).
//! Each channel is implemented in a focused sub-module.

pub mod webhook;
pub mod email;
pub mod sms;
pub mod slack;
pub mod teams;

use super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::debug;

impl NotificationEngine {
    /// Create a new notification engine with the given configuration
    pub fn new(config: NotificationConfig) -> Self {
        Self { config }
    }

    /// Notify that a workflow has been initiated
    pub async fn notify_workflow_initiated(&self, workflow: &Workflow) -> BearDogResult<()> {
        let message = format!(
            "Workflow {} initiated by {}: {}",
            workflow.workflow_type, workflow.initiator, workflow.id
        );

        let notification_message = NotificationMessage {
            content: message,
            metadata: workflow.metadata.clone(),
        };
        self.send_notification(&notification_message).await.map(|_| ())
    }

    /// Notify that an approval has been submitted
    pub async fn notify_approval_submitted(
        &self,
        workflow: &Workflow,
        approval: &ApprovalRecord,
    ) -> BearDogResult<()> {
        let message = format!(
            "Approval submitted for workflow {}: {} by {}",
            workflow.id, approval.decision, approval.approver
        );

        let notification_message = NotificationMessage {
            content: message,
            metadata: approval.metadata.clone(),
        };
        self.send_notification(&notification_message).await.map(|_| ())
    }

    /// Notify that a workflow has been completed
    pub async fn notify_workflow_completed(&self, workflow: &Workflow) -> BearDogResult<()> {
        let message = format!(
            "Workflow {} completed with status: {}",
            workflow.id, workflow.status
        );

        let notification_message = NotificationMessage {
            content: message,
            metadata: workflow.metadata.clone(),
        };
        self.send_notification(&notification_message).await.map(|_| ())
    }

    /// Send notification through all configured channels
    pub async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        let mut result = NotificationResult {
            email_sent: false,
            sms_sent: false,
            webhook_sent: false,
            slack_sent: false,
            teams_sent: false,
            errors: Vec::new(),
        };

        // Send email notifications
        if self.config.email_enabled {
            match self.send_email_notification(&message.content, &message.metadata).await {
                Ok(_) => result.email_sent = true,
                Err(e) => result.errors.push(format!("Email: {e}")),
            }
        }

        // Send SMS notifications  
        if self.config.sms_enabled {
            match self.send_sms_notification(&message.content, &message.metadata).await {
                Ok(_) => result.sms_sent = true,
                Err(e) => result.errors.push(format!("SMS: {e}")),
            }
        }

        // Send webhook notifications
        if self.config.webhook_enabled {
            match self.send_webhook_notification(&message.content, &message.metadata).await {
                Ok(_) => result.webhook_sent = true,
                Err(e) => result.errors.push(format!("Webhook: {e}")),
            }
        }

        // Send Slack notifications
        if self.config.slack_enabled {
            match self.send_slack_notification(&message.content, &message.metadata).await {
                Ok(_) => result.slack_sent = true,
                Err(e) => result.errors.push(format!("Slack: {e}")),
            }
        }

        // Send Teams notifications
        if self.config.teams_enabled {
            match self.send_teams_notification(&message.content, &message.metadata).await {
                Ok(_) => result.teams_sent = true,
                Err(e) => result.errors.push(format!("Teams: {e}")),
            }
        }

        Ok(result)
    }

    /// Test all configured notification channels
    pub async fn test_notifications(&self) -> BearDogResult<HashMap<String, bool>> {
        let mut results = HashMap::new();

        // Test email configuration
        if self.config.email_enabled {
            let email_result = self.test_email_config().await;
            results.insert("email".to_string(), email_result.is_ok());
            if let Err(e) = email_result {
                debug!("Email test failed: {}", e);
            }
        }

        // Test SMS configuration
        if self.config.sms_enabled {
            let sms_result = self.test_sms_config().await;
            results.insert("sms".to_string(), sms_result.is_ok());
            if let Err(e) = sms_result {
                debug!("SMS test failed: {}", e);
            }
        }

        // Test webhook configuration
        if self.config.webhook_enabled {
            let webhook_result = self.test_webhook_config().await;
            results.insert("webhook".to_string(), webhook_result.is_ok());
            if let Err(e) = webhook_result {
                debug!("Webhook test failed: {}", e);
            }
        }

        // Test Slack configuration
        if self.config.slack_enabled {
            let slack_result = self.test_slack_config().await;
            results.insert("slack".to_string(), slack_result.is_ok());
            if let Err(e) = slack_result {
                debug!("Slack test failed: {}", e);
            }
        }

        // Test Teams configuration
        if self.config.teams_enabled {
            let teams_result = self.test_teams_config().await;
            results.insert("teams".to_string(), teams_result.is_ok());
            if let Err(e) = teams_result {
                debug!("Teams test failed: {}", e);
            }
        }

        Ok(results)
    }
} 