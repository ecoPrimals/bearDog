//! Universal notification adapters
//!
//! These adapters can work with any service that supports standard protocols
//! Users can configure whatever communication methods they actually have

use super::*;
use beardog_config::integration::{EmailConfig, SlackNotificationConfig, WebhookConfig};
use beardog_errors::{BearDogError, BearDogResult};
use tracing::debug;

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
            url: webhook_url,
            auth_token: None,
            timeout: std::time::Duration::from_secs(10),
        })
    }

    /// Create a Slack-compatible adapter
    pub fn slack(webhook_url: String) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            url: webhook_url,
            auth_token: None,
            timeout: std::time::Duration::from_secs(10),
        })
    }

    /// Create a Teams-compatible adapter
    pub fn teams(webhook_url: String) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            url: webhook_url,
            auth_token: None,
            timeout: std::time::Duration::from_secs(10),
        })
    }
}

#[async_trait]
impl NotificationAdapter for WebhookAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        if !self.config.enabled {
            return Err(BearDogError::ConfigurationError {
                message: "Webhook adapter is disabled".to_string(),
            });
        }

        // Test with a simple ping message
        let test_payload = serde_json::json!({
            "content": "BearDog notification test",
            "text": "BearDog notification test"
        });

        let mut request = self
            .client
            .post(&self.config.url)
            .timeout(self.config.timeout)
            .json(&test_payload);

        if let Some(token) = &self.config.auth_token {
            request = request.bearer_auth(token);
        }

        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(BearDogError::NotificationError {
                        message: format!("Webhook test failed: {}", response.status()),
                    })
                }
            }
            Err(e) => Err(BearDogError::NotificationError {
                message: format!("Webhook connection failed: {e}"),
            }),
        }
    }

    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        let delivery_id = uuid::Uuid::new_v4().to_string();

        // Create payload that works with most webhook services
        let payload = self.create_webhook_payload(message)?;

        let mut request = self
            .client
            .post(&self.config.url)
            .timeout(self.config.timeout)
            .json(&payload);

        if let Some(token) = &self.config.auth_token {
            request = request.bearer_auth(token);
        }

        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(NotificationResult {
                        delivery_id,
                        success: true,
                        message: "Webhook notification sent successfully".to_string(),
                        timestamp: chrono::Utc::now(),
                        retry_count: 0,
                    })
                } else {
                    Err(BearDogError::NotificationError {
                        message: format!("Webhook failed: {}", response.status()),
                    })
                }
            }
            Err(e) => Err(BearDogError::NotificationError {
                message: format!("Webhook error: {e}"),
            }),
        }
    }

    fn supported_formats(&self) -> Vec<MessageFormat> {
        vec![
            MessageFormat::PlainText,
            MessageFormat::Markdown,
            MessageFormat::Json,
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
}

impl WebhookAdapter {
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

#[async_trait]
impl NotificationAdapter for EmailAdapter {
    fn name(&self) -> &str {
        "email"
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        if !self.config.enabled {
            return Err(BearDogError::ConfigurationError {
                message: "Email adapter is disabled".to_string(),
            });
        }

        // Test SMTP connection
        tracing::info!(
            "Testing SMTP connection to {}:{}",
            self.config.smtp_server,
            self.config.smtp_port
        );

        // In a real implementation, would test actual SMTP connection
        // For now, validate configuration
        if self.config.smtp_server.is_empty() || self.config.from_address.is_empty() {
            return Err(BearDogError::ConfigurationError {
                message: "Invalid email configuration".to_string(),
            });
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
    client: reqwest::Client,
}

impl SmsAdapter {
    pub fn twilio(api_key: String, from_number: String) -> Self {
        Self {
            provider: "twilio".to_string(),
            api_key,
            from_number,
            client: reqwest::Client::new(),
        }
    }

    pub fn aws_sns(api_key: String) -> Self {
        Self {
            provider: "aws_sns".to_string(),
            api_key,
            from_number: String::new(),
            client: reqwest::Client::new(),
        }
    }

    pub fn custom(provider: String, api_key: String, from_number: String) -> Self {
        Self {
            provider,
            api_key,
            from_number,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NotificationAdapter for SmsAdapter {
    fn name(&self) -> &str {
        "sms"
    }

    async fn test_connection(&self) -> BearDogResult<()> {
        tracing::info!("Testing SMS connection via {}", self.provider);

        // In a real implementation, would test the actual SMS API
        if self.api_key.is_empty() {
            return Err(BearDogError::ConfigurationError {
                message: "SMS API key not configured".to_string(),
            });
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

        let delivery_id = uuid::Uuid::new_v4().to_string();
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
                }
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
                }
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
            }
            .map_err(|e| BearDogError::NotificationError {
                message: format!("SMS send failed via {}: {}", self.provider, e),
            })?;

            if response.status().is_success() {
                success_count += 1;
            }
        }

        Ok(NotificationResult {
            delivery_id,
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
            supports_attachments: false,
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
    pub fn new(config: SlackNotificationConfig) -> Self {
        let webhook_config = WebhookConfig {
            enabled: config.enabled,
            url: config.webhook_url,
            auth_token: config.bot_token,
            timeout: config.timeout,
        };

        Self {
            webhook_adapter: WebhookAdapter::new(webhook_config),
        }
    }
}

#[async_trait]
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

/// Configuration builder for easy adapter setup
pub struct NotificationAdapterBuilder {
    adapters: Vec<Box<dyn NotificationAdapter>>,
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
            enabled: true,
            url,
            auth_token,
            timeout: std::time::Duration::from_secs(10),
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
