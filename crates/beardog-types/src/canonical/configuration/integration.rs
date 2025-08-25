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


/// # Integration Configuration
///
/// **CANONICAL INTEGRATION CONFIGURATION TYPES**
/// This module contains all integration-related configuration structures.
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL WORKFLOW CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowConfig {
    /// Workflow notification settings
    pub notification: WorkflowNotificationConfig,
    /// Workflow policy settings
    pub policy: WorkflowPolicyConfig,
}
/// **CANONICAL WORKFLOW NOTIFICATION CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowNotificationConfig {
    /// Email configuration
    pub email: EmailConfig,
    /// Slack notification settings
    pub slack: SlackNotificationConfig,
    /// Webhook configuration
    pub webhook: WebhookConfig,
}

/// **CANONICAL WORKFLOW POLICY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowPolicyConfig {
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Timeout for workflow execution
    pub timeout: Duration,
}

/// **CANONICAL EMAIL CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailConfig {
    /// SMTP server
    pub smtp_server: String,
    /// SMTP port
    pub smtp_port: u16,
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// From address
    pub from_address: String,
}

/// **CANONICAL SLACK NOTIFICATION CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlackNotificationConfig {
    /// Webhook URL
    pub webhook_url: String,
    /// Channel to post to
    pub channel: String,
    /// Bot username
    pub bot_username: String,
}

/// **CANONICAL WEBHOOK CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookConfig {
    pub url: String,
    /// HTTP method
    pub method: String,
    /// Headers to include
    pub headers: std::collections::HashMap<String, String>,
    /// Timeout for webhook calls
    pub timeout: Duration,
}

/// **CANONICAL WORKFLOW RETRY CONFIGURATION** - Defined in performance module
/// **WORKFLOWS MODULE** - Workflow-specific configurations
pub mod workflows {
    pub use super::{WorkflowConfig, WorkflowNotificationConfig, WorkflowPolicyConfig};
}
