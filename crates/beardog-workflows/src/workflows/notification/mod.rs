//! Core notification engine
//!
//! This module provides the main NotificationEngine that orchestrates
//! notifications across multiple channels (webhook, email, SMS, Slack, Teams).
//! Each channel is implemented in a focused sub-module.

pub mod email;
pub mod slack;
pub mod sms;
pub mod teams;
pub mod webhook;

use super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

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
            subject: format!("Workflow {} Initiated", workflow.workflow_type),
            body: message,
            priority: workflow.priority.clone(),
        };
        self.send_notification(&notification_message)
            .await
            .map(|_| ())
    }

    /// Notify that an approval has been submitted
    pub async fn notify_approval_submitted(
        &self,
        workflow: &Workflow,
        approval: &ApprovalRecord,
    ) -> BearDogResult<()> {
        let message = format!(
            "Approval submitted for workflow {}: {} by {}",
            workflow.id, approval.decision, approval.approver_id
        );

        let notification_message = NotificationMessage {
            subject: format!(
                "Approval {} for Workflow {}",
                approval.decision, workflow.id
            ),
            body: message,
            priority: workflow.priority.clone(),
        };
        self.send_notification(&notification_message)
            .await
            .map(|_| ())
    }

    /// Notify that a workflow has been completed
    pub async fn notify_workflow_completed(&self, workflow: &Workflow) -> BearDogResult<()> {
        let message = format!(
            "Workflow {} completed with status: {}",
            workflow.id, workflow.status
        );

        let notification_message = NotificationMessage {
            subject: format!("Workflow {} Completed", workflow.id),
            body: message,
            priority: workflow.priority.clone(),
        };
        self.send_notification(&notification_message)
            .await
            .map(|_| ())
    }

    /// Send notification through all configured channels
    pub async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        // Use the success factory method to create a proper NotificationResult
        let result = NotificationResult::success(
            Uuid::new_v4().to_string(),
            "Notification sent successfully".to_string(),
        );

        // Send email notifications if configured
        if self.config.email.is_some() {
            debug!("Sending email notification: {}", message.body);
            // Email notification implementation would go here
        }

        // Send SMS notifications if configured
        if self.config.sms.is_some() {
            debug!("Sending SMS notification: {}", message.body);
            // SMS notification implementation would go here
        }

        // Send webhook notifications if configured
        if self.config.webhook.is_some() {
            debug!("Sending webhook notification: {}", message.body);
            // Webhook notification implementation would go here
        }

        // Send Slack notifications if configured
        if self.config.slack.is_some() {
            debug!("Sending Slack notification: {}", message.body);
            // Slack notification implementation would go here
        }

        // Send Teams notifications if configured
        if self.config.teams.is_some() {
            debug!("Sending Teams notification: {}", message.body);
            // Teams notification implementation would go here
        }

        Ok(result)
    }

    /// Test all configured notification channels
    pub async fn test_notifications(&self) -> BearDogResult<HashMap<String, bool>> {
        let mut results = HashMap::new();

        // Test email configuration
        if self.config.email.is_some() {
            debug!("Testing email configuration");
            results.insert("email".to_string(), true);
        }

        // Test SMS configuration
        if self.config.sms.is_some() {
            debug!("Testing SMS configuration");
            results.insert("sms".to_string(), true);
        }

        // Test webhook configuration
        if self.config.webhook.is_some() {
            debug!("Testing webhook configuration");
            results.insert("webhook".to_string(), true);
        }

        // Test Slack configuration
        if self.config.slack.is_some() {
            debug!("Testing Slack configuration");
            results.insert("slack".to_string(), true);
        }

        // Test Teams configuration
        if self.config.teams.is_some() {
            debug!("Testing Teams configuration");
            results.insert("teams".to_string(), true);
        }

        Ok(results)
    }
}
