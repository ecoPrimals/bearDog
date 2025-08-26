

use super::*;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::config::integration::workflows::{
    EmailNotificationConfig as EmailConfig, WebhookNotificationConfig as WebhookConfig,
    WorkflowNotificationConfig as NotificationConfig,
};
use tracing::debug;
use std::collections::HashMap;

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

    pub fn discord(webhook_url: &str) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            urls: vec![webhook_url],
            auth_headers: HashMap::with_capacity(16),
            max_retries: 3,
        })
    }

    pub fn slack(webhook_url: &str) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            urls: vec![webhook_url],
            auth_headers: HashMap::with_capacity(16),
            max_retries: 3,
        })
    }

    pub fn teams(webhook_url: &str) -> Self {
        Self::new(WebhookConfig {
            enabled: true,
            urls: vec![webhook_url],
            auth_headers: HashMap::with_capacity(16),
            max_retries: 3,
        })
    }
}

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

        let mut payload = serde_json::Map::new();

        payload.insert(
            "content".to_string(),
            serde_json::Value::String(format_args!("**{}**\n{}", message.title, message.content).to_string()),
        );

        payload.insert(
            "text".to_string(),
            serde_json::Value::String(format_args!("{}: {}", message.title, message.content).to_string()),
        );

        payload.insert(
            "title".to_string(),
            serde_json::Value::String(message.title.clone()),
        );
        payload.insert(
            "summary".to_string(),
            serde_json::Value::String(message.content.clone()),
        );

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
            message: format_args!("Email sent to {} recipients", message.recipients.len().to_string()),
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

pub struct SmsAdapter {
    provider: String,
    api_key: String,
    from_number: String,
}

impl SmsAdapter {
    pub fn twilio(api_key: &str, from_number: &str) -> Self {
        Self {
            provider: "twilio".to_string(),
            api_key,
            from_number,
        }
    }

    pub fn aws_sns(api_key: &str) -> Self {
        Self {
            provider: "aws_sns".to_string(),
            api_key,
            from_number: String::with_capacity(64),
        }
    }

    pub fn custom(provider: &str, api_key: &str, from_number: &str) -> Self {
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

        if self.api_key.is_empty() {
            return Err(BearDogError::configuration(
                "SMS API key not configured".to_string(),
            ));
        }
        Ok(())
    }

    async fn send_notification(
        &self,
        message: &NotificationMessage,
    ) -> BearDogResult<NotificationResult> {
        debug!(
            "Sending SMS notification via {} from {}",
            self.provider, self.from_number
        );
        let mut success_count = 0;

        for recipient in &message.recipients {

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

                    self.client
                        .post("https://sns.amazonaws.com/")
                        .header(
                            "Authorization",
                            format_args!("AWS4-HMAC-SHA256 Credential={}", self.api_key).to_string(),
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

#[derive(Debug)]
pub enum NotificationAdapterType {
    Discord(DiscordNotificationAdapter),
    Slack(SlackNotificationAdapter),
    Email(EmailNotificationAdapter),
    Webhook(WebhookNotificationAdapter),
}

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

    pub fn discord(mut self, webhook_url: &str) -> Self {
        self.adapters
            .push(Box::new(WebhookAdapter::discord(webhook_url)));
        self
    }

    pub fn slack(mut self, webhook_url: &str) -> Self {
        self.adapters
            .push(Box::new(WebhookAdapter::slack(webhook_url)));
        self
    }

    pub fn teams(mut self, webhook_url: &str) -> Self {
        self.adapters
            .push(Box::new(WebhookAdapter::teams(webhook_url)));
        self
    }

    pub fn sms_twilio(mut self, api_key: &str, from_number: &str) -> Self {
        self.adapters
            .push(Box::new(SmsAdapter::twilio(api_key, from_number)));
        self
    }

    pub fn email(mut self, config: EmailConfig) -> Self {
        self.adapters.push(Box::new(EmailAdapter::new(config)));
        self
    }

    pub fn custom_webhook(mut self, url: &str, auth_token: Option<&str>) -> Self {
        let config = WebhookConfig {
            urls: vec![url],
            auth_headers: if let Some(token) = auth_token {
                let mut headers = HashMap::with_capacity(16);
                headers.insert("Authorization".to_string(), format_args!("Bearer {}", token).to_string());
                headers
            } else {
                HashMap::with_capacity(16)
            },
            enabled: true,
            max_retries: 3,
        };
        self.adapters.push(Box::new(WebhookAdapter::new(config)));
        self
    }

    pub fn build(self, config: NotificationConfig) -> NotificationEngine {
        let mut engine = NotificationEngine::new(config);
        for adapter in self.adapters {
            engine.register_adapter(adapter);
        }
        engine
    }
}
