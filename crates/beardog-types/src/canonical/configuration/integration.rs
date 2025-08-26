

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowConfig {

    pub notification: WorkflowNotificationConfig,

    pub policy: WorkflowPolicyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowNotificationConfig {

    pub email: EmailConfig,

    pub slack: SlackNotificationConfig,

    pub webhook: WebhookConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowPolicyConfig {

    pub max_retries: u32,

    pub retry_delay: Duration,

    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailConfig {

    pub smtp_server: String,

    pub smtp_port: u16,

    pub username: String,

    pub password: String,

    pub from_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlackNotificationConfig {

    pub webhook_url: String,

    pub channel: String,

    pub bot_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookConfig {
    pub url: String,

    pub method: String,

    pub headers: std::collections::HashMap<String, String>,

    pub timeout: Duration,
}

pub mod workflows {
    pub use super::{WorkflowConfig, WorkflowNotificationConfig, WorkflowPolicyConfig};
}
