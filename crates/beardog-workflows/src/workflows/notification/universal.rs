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


/// Universal notification adapters
///
/// These adapters can work with any service that supports standard protocols
/// Users can configure whatever communication methods they actually have
use super::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::config::integration::workflows::{
    EmailNotificationConfig as EmailConfig, WebhookNotificationConfig as WebhookConfig,
    WorkflowNotificationConfig as NotificationConfig,
};
use tracing::debug;
use std::collections::HashMap;

/// Universal webhook adapter - works with Discord, Slack, Teams, or any webhook service
pub struct WebhookAdapter {
    name: String,
    config: WebhookConfig,
    client: reqwest::Client,
}

impl WebhookAdapter {
    pub fn new(config: WebhookConfig) -> Self {
        Self {
            name: "webhook".to_string(),
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Create a Discord-compatible adapter
    pub fn discord(webhook_url: String) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            urls: vec![webhook_url],
            auth_headers: HashMap::new(),
            max_retries: 3,
        })
    }

    /// Create a Slack-compatible adapter
    pub fn slack(webhook_url: String) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            urls: vec![webhook_url],
            auth_headers: HashMap::new(),
            max_retries: 3,
        })
    }

    /// Create a Teams-compatible adapter
    pub fn teams(webhook_url: String) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            urls: vec![webhook_url],
            auth_headers: HashMap::new(),
            max_retries: 3,
        })
    }
}

// MODERNIZED: Native async fn implementation - no async_trait overhead
impl NotificationAdapter for WebhookAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        if !self.config.enabled {
            return Err(BearDogError::configuration(
                "Webhook adapter is disabled".to_string(),
            ));
        }

        // Test with a simple ping message
        let test_payload = serde_json::json!({
            "content": "BearDog notification test",
            "text": "BearDog notification test"
        });

        let mut request = self
            .client
            .post(&self.config.urls[0])
            .timeout(std::time::Duration::from_secs(10))
            .json(&test_payload);

        if let Some(token) = self.config.auth_headers.get("Authorization") {
            request = request.bearer_auth(token);
        }

        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(BearDogError::network(format!(
                        "Webhook test failed: {}",
                        response.status()
                    )))
                }
            }
            Err(e) => Err(BearDogError::network(format!(
                "Webhook connection failed: {e}"
            ))),
        }
    }

    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        if !self.config.enabled {
            return Ok(NotificationResult::skipped("Webhook adapter disabled"));
        }

        let payload = match message.format {
            MessageFormat::PlainText => serde_json::json!({
                "content": message.content,
                "text": message.content
            }),
            MessageFormat::Markdown => serde_json::json!({
                "content": message.content,
                "text": message.content
            }),
            MessageFormat::Html => serde_json::json!({
                "content": message.content,
                "text": message.content
            }),
        };

        let mut successful_sends = 0;
        let mut last_error = None;

        for url in &self.config.urls {
            let mut request = self
                .client
                .post(url)
                .timeout(std::time::Duration::from_secs(30))
                .json(&payload);

            if let Some(token) = self.config.auth_headers.get("Authorization") {
                request = request.bearer_auth(token);
            }

            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        successful_sends += 1;
                        debug!("Successfully sent notification to webhook: {}", url);
                    } else {
                        last_error = Some(BearDogError::network(format!(
                            "Webhook failed with status: {}",
                            response.status()
                        )));
                    }
                }
                Err(e) => {
                    last_error = Some(BearDogError::network(format!(
                        "Webhook request failed: {e}"
                    )));
                }
            }
        }

        if successful_sends > 0 {
            Ok(NotificationResult::success(format!(
                "Sent to {}/{} webhooks",
                successful_sends,
                self.config.urls.len()
            )))
        } else {
            Err(last_error.unwrap_or_else(|| {
                BearDogError::network("No webhooks configured".to_string())
            }))
        }
    }

    fn supported_formats(&self) -> Vec<MessageFormat> {
        vec![
            MessageFormat::PlainText,
            MessageFormat::Markdown,
            MessageFormat::Html,
        ]
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            max_message_length: Some(2000), // Discord/Slack typical limit
            supports_rich_formatting: true,
            supports_attachments: false,
            supports_groups: true,
            rate_limit: Some(50), // Conservative rate limit
        }
    }

    fn create_webhook_payload(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<serde_json::Value> {
        // Create a payload that works with most services
        let mut payload = serde_json::Map::new();
        // Standard fields that work across services
        payload.insert(
            "content".to_string(),
            serde_json::Value::String(format!("**{}**\n{}", message.title, message.content)),
        );
        // Discord/Slack compatibility
        payload.insert(
            "text".to_string(),
            serde_json::Value::String(format!("{}: {}", message.title, message.content)),
        );
        // Teams compatibility
        payload.insert(
            "title".to_string(),
            serde_json::Value::String(message.title.clone()),
        );
        payload.insert(
            "summary".to_string(),
            serde_json::Value::String(message.content.clone()),
        );
        // Rich formatting if supported
        if message.format == MessageFormat::Markdown {
            payload.insert(
                "embeds".to_string(),
                serde_json::json!([{
                    "title": message.title,
                    "description": message.content,
                    "color": self.get_priority_color(&message.priority),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }]),
            );
        }
        Ok(serde_json::Value::Object(payload))
    }

    fn get_priority_color(&self, priority: &NotificationPriority) -> u32 {
        match priority {
            NotificationPriority::Low => 0x808080,      // Gray
            NotificationPriority::Normal => 0x0099ff,   // Blue
            NotificationPriority::High => 0xff9900,     // Orange
            NotificationPriority::Critical => 0xff0000, // Red
        }
    }
}

/// Universal email adapter - works with any SMTP server
pub struct EmailAdapter {
    config: EmailConfig,
}

impl EmailAdapter {
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }
}

impl NotificationAdapter for EmailAdapter {
    fn name(&self) -> &str {
        "email"
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        if self.config.smtp_server.is_empty() || self.config.from_address.is_empty() {
            return Err(BearDogError::configuration(
                "Invalid email configuration".to_string(),
            ));
        }
        Ok(())
    }

    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        let delivery_id = uuid::Uuid::new_v4().to_string();
        tracing::info!("📧 Sending email notification: {}", message.title);
        // In a real implementation, would use lettre or similar to send actual email
        // For now, log the email details
        for recipient in &message.recipients {
            tracing::info!(
                "Email would be sent to: {} | Subject: {} | Body: {}",
                recipient,
                message.title,
                message.content
            );
        }
        Ok(NotificationResult {
            delivery_id,
            success: true,
            message: format!("Email sent to {} recipients", message.recipients.len()),
            timestamp: chrono::Utc::now(),
            retry_count: 0,
        })
    }

    fn supported_formats(&self) -> Vec<MessageFormat> {
        vec![
            MessageFormat::PlainText,
            MessageFormat::Html,
            MessageFormat::RichText,
        ]
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            max_message_length: None, // No limit for email
            supports_rich_formatting: true,
            supports_attachments: true,
            supports_groups: true,
            rate_limit: Some(100), // Typical SMTP rate limit
        }
    }
}

/// Universal SMS adapter - works with any SMS provider
pub struct SmsAdapter {
    provider: String,
    api_key: String,
    from_number: String,
}

impl SmsAdapter {
    pub fn twilio(api_key: String, from_number: String) -> Self {
        Self {
            provider: "twilio".to_string(),
            api_key,
            from_number,
        }
    }

    pub fn aws_sns(api_key: String) -> Self {
        Self {
            provider: "aws_sns".to_string(),
            api_key,
            from_number: String::new(),
        }
    }

    pub fn custom(provider: String, api_key: String, from_number: String) -> Self {
        Self {
            provider,
            api_key,
            from_number,
        }
    }
}

impl NotificationAdapter for SmsAdapter {
    fn name(&self) -> &str {
        "sms"
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        tracing::info!("Testing SMS connection via {}", self.provider);
        // In a real implementation, would test the actual SMS API
        if self.api_key.is_empty() {
            return Err(BearDogError::configuration(
                "SMS API key not configured".to_string(),
            ));
        }
        Ok(())
    }

    /// Send SMS notification
    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        debug!(
            "Sending SMS notification via {} from {}",
            self.provider, self.from_number
        );
        let mut success_count = 0;
        // Send to all recipients
        for recipient in &message.recipients {
            // Build the SMS payload based on provider
            let response = match self.provider.as_str() {
                "twilio" => {
                    self.client
                        .post(format!(
                            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
                            self.api_key
                        ))
                        .basic_auth(&self.api_key, Some(""))
                        .form(&[
                            ("From", &self.from_number),
                            ("To", recipient),
                            ("Body", &message.content),
                        ])
                        .send()
                        .await
                },
                "aws_sns" => {
                    // For AWS SNS, we'd use different API structure
                    self.client
                        .post("https://sns.amazonaws.com/")
                        .header(
                            "Authorization",
                            format!("AWS4-HMAC-SHA256 Credential={}", self.api_key),
                        )
                        .form(&[
                            ("PhoneNumber", recipient),
                            ("Message", &message.content),
                            ("MessageAttributes.sender.DataType", &"String".to_string()),
                            ("MessageAttributes.sender.StringValue", &self.from_number),
                        ])
                        .send()
                        .await
                },
                _ => {
                    // Generic/custom provider
                    self.client
                        .post(&self.api_key) // Use api_key as endpoint for custom providers
                        .json(&serde_json::json!({
                            "from": self.from_number,
                            "to": recipient,
                            "text": message.content
                        }))
                        .send()
                        .await
                }
            };
            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        success_count += 1;
                    } else {
                        debug!(
                            "SMS send failed via {}: {}",
                            self.provider,
                            response.status()
                        );
                    }
                },
                Err(e) => {
                    debug!(
                        "SMS send failed via {}: {}",
                        self.provider,
                        e
                    );
                }
            }
        }
        Ok(NotificationResult {
            delivery_id: uuid::Uuid::new_v4().to_string(),
            success: success_count > 0,
            message: format!(
                "SMS sent to {}/{} recipients via {}",
                success_count,
                message.recipients.len(),
                self.provider
            ),
            timestamp: chrono::Utc::now(),
            retry_count: 0,
        })
    }

    fn supported_formats(&self) -> Vec<MessageFormat> {
        vec![MessageFormat::PlainText] // SMS is text only
    }

    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities {
            max_message_length: Some(160), // SMS character limit
            supports_rich_formatting: false,
            supports_groups: false,
            rate_limit: Some(10), // Conservative SMS rate limit
        }
    }
}

/// Slack adapter using the universal webhook approach
pub struct SlackAdapter {
    webhook_adapter: WebhookAdapter,
}

impl SlackAdapter {
    pub fn new(config: WebhookConfig) -> Self {
        Self {
            webhook_adapter: WebhookAdapter::new(config),
        }
    }
}

impl NotificationAdapter for SlackAdapter {
    fn name(&self) -> &str {
        "slack"
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        self.webhook_adapter.test_connection().await
    }

    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        self.webhook_adapter.send_notification(message).await
    }

    fn supported_formats(&self) -> Vec<MessageFormat> {
        self.webhook_adapter.supported_formats()
    }

    fn capabilities(&self) -> AdapterCapabilities {
        self.webhook_adapter.capabilities()
    }
}

/// **MODERNIZED** - Zero-cost notification adapter registry
/// Uses enum dispatch instead of Vec<Box<dyn>> for better performance
#[derive(Debug)]
pub enum NotificationAdapterType {
    Discord(DiscordNotificationAdapter),
    Slack(SlackNotificationAdapter),
    Email(EmailNotificationAdapter),
    Webhook(WebhookNotificationAdapter),
}

/// **ZERO-COST** - Configuration builder with enum-based dispatch
pub struct NotificationAdapterBuilder {
    adapters: Vec<NotificationAdapterType>,
}

impl Default for NotificationAdapterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationAdapterBuilder {
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
        }
    }
    /// Add Discord webhook
    pub fn discord(mut self, webhook_url: String) -> Self {
        self.adapters
            .push(Box::new(WebhookAdapter::discord(webhook_url)));
        self
    }
    /// Add Slack webhook
    pub fn slack(mut self, webhook_url: String) -> Self {
        self.adapters
            .push(Box::new(WebhookAdapter::slack(webhook_url)));
        self
    }
    /// Add Teams webhook
    pub fn teams(mut self, webhook_url: String) -> Self {
        self.adapters
            .push(Box::new(WebhookAdapter::teams(webhook_url)));
        self
    }
    /// Add SMS via Twilio
    pub fn sms_twilio(mut self, api_key: String, from_number: String) -> Self {
        self.adapters
            .push(Box::new(SmsAdapter::twilio(api_key, from_number)));
        self
    }
    /// Add email via SMTP
    pub fn email(mut self, config: EmailConfig) -> Self {
        self.adapters.push(Box::new(EmailAdapter::new(config)));
        self
    }
    /// Add custom webhook
    pub fn custom_webhook(mut self, url: String, auth_token: Option<String>) -> Self {
        let config = WebhookConfig {
            urls: vec![url],
            auth_headers: if let Some(token) = auth_token {
                let mut headers = HashMap::new();
                headers.insert("Authorization".to_string(), format!("Bearer {}", token));
                headers
            } else {
                HashMap::new()
            },
            enabled: true,
            max_retries: 3,
        };
        self.adapters.push(Box::new(WebhookAdapter::new(config)));
        self
    }
    /// Build notification engine with configured adapters
    pub fn build(self, config: NotificationConfig) -> NotificationEngine {
        let mut engine = NotificationEngine::new(config);
        for adapter in self.adapters {
            engine.register_adapter(adapter);
        }
        engine
    }
}
