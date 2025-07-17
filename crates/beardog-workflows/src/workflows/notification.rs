//! Notification engine for workflow events
//!
//! Handles sending notifications for workflow state changes and approvals.

use super::types::*;
use beardog_errors::BearDogResult;

use std::collections::HashMap;
use std::time::Duration;
use serde_json::json;
use tracing::{debug, error, info, warn};
use reqwest::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};

impl NotificationEngine {
    /// Create a new notification engine
    pub fn new(config: NotificationConfig) -> Self {
        Self { config }
    }

    /// Notify that a workflow has been initiated
    pub async fn notify_workflow_initiated(&self, workflow: &Workflow) -> BearDogResult<()> {
        let message = format!(
            "Workflow {} initiated by {}: {}",
            workflow.workflow_type, workflow.initiator, workflow.id
        );

        self.send_notification_legacy(&message, &workflow.metadata).await
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

        self.send_notification_legacy(&message, &approval.metadata).await
    }

    /// Notify that a workflow has been completed
    pub async fn notify_workflow_completed(&self, workflow: &Workflow) -> BearDogResult<()> {
        let message = format!(
            "Workflow {} completed with status: {}",
            workflow.id, workflow.status
        );

        self.send_notification_legacy(&message, &workflow.metadata).await
    }

    /// Send notification through configured channels
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

        // Email notifications
        if self.config.email_enabled {
            match self.send_email_notification(&message.content, &message.metadata).await {
                Ok(_) => result.email_sent = true,
                Err(e) => result.errors.push(format!("Email: {}", e)),
            }
        }

        // SMS notifications
        if self.config.sms_enabled {
            match self.send_sms_notification(&message.content, &message.metadata).await {
                Ok(_) => result.sms_sent = true,
                Err(e) => result.errors.push(format!("SMS: {}", e)),
            }
        }

        // Webhook notifications
        if self.config.webhook_enabled {
            match self.send_webhook_notification(&message.content, &message.metadata).await {
                Ok(_) => result.webhook_sent = true,
                Err(e) => result.errors.push(format!("Webhook: {}", e)),
            }
        }

        // Slack notifications
        if self.config.slack_enabled {
            match self.send_slack_notification(&message.content, &message.metadata).await {
                Ok(_) => result.slack_sent = true,
                Err(e) => result.errors.push(format!("Slack: {}", e)),
            }
        }

        // Teams notifications
        if self.config.teams_enabled {
            match self.send_teams_notification(&message.content, &message.metadata).await {
                Ok(_) => result.teams_sent = true,
                Err(e) => result.errors.push(format!("Teams: {}", e)),
            }
        }

        Ok(result)
    }

    /// Send notification through configured channels (legacy method)
    async fn send_notification_legacy(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        let notification_message = NotificationMessage {
            content: message.to_string(),
            metadata: metadata.clone(),
        };
        
        let result = self.send_notification(&notification_message).await?;
        
        if !result.errors.is_empty() {
            return Err(beardog_errors::BearDogError::External {
                message: format!("Notification failures: {}", result.errors.join(", "))
            });
        }
        
        Ok(())
    }

    /// Send webhook notification
    async fn send_webhook_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        if !self.config.webhook_enabled {
            return Ok(());
        }

        let webhook_url = self.config.webhook_url.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "Webhook URL not configured".to_string()
            }
        })?;

        let payload = json!({
            "message": message,
            "metadata": metadata,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "source": "beardog-security"
        });

        // Create HTTP client with timeout
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        // Generate signature if webhook secret is configured
        let signature = if let Some(secret) = &self.config.webhook_secret {
            let payload_str = serde_json::to_string(&payload)?;
            let signature = self.generate_webhook_signature(secret, &payload_str)?;
            Some(signature)
        } else {
            None
        };

        // Build request
        let mut request_builder = client
            .post(webhook_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "BearDog-Security/1.0")
            .json(&payload);

        // Add signature header if available
        if let Some(sig) = signature {
            request_builder = request_builder.header("X-Webhook-Signature", format!("sha256={}", sig));
        }

        // Send webhook with retry logic
        let mut last_error = None;
        let max_retries = 3;
        
        for attempt in 1..=max_retries {
            match request_builder.try_clone().unwrap().send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("✅ Webhook notification sent successfully to {}", webhook_url);
                        return Ok(());
                    } else {
                        let status = response.status();
                        let error_body = response.text().await.unwrap_or_default();
                        last_error = Some(format!("HTTP {}: {}", status, error_body));
                        warn!("❌ Webhook notification failed (attempt {}/{}): HTTP {}", attempt, max_retries, status);
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    warn!("❌ Webhook notification failed (attempt {}/{}): {}", attempt, max_retries, e);
                }
            }
            
            // Wait before retry (exponential backoff)
            if attempt < max_retries {
                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt - 1))).await;
            }
        }

        Err(beardog_errors::BearDogError::External {
            message: format!(
                "Failed to send webhook notification after {} attempts. Last error: {}",
                max_retries,
                last_error.unwrap_or_default()
            )
        })
    }

    /// Generate webhook signature
    fn generate_webhook_signature(&self, secret: &str, payload: &str) -> BearDogResult<String> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| beardog_errors::BearDogError::Crypto {
                message: format!("HMAC key error: {}", e)
            })?;
        
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        
        Ok(hex::encode(result.into_bytes()))
    }

    /// Send email notification
    async fn send_email_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        if !self.config.email_enabled {
            return Ok(());
        }

        let smtp_server = self.config.smtp_server.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMTP server not configured".to_string()
            }
        })?;

        let smtp_port = self.config.smtp_port.unwrap_or(587);
        let username = self.config.smtp_username.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMTP username not configured".to_string()
            }
        })?;
        let password = self.config.smtp_password.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMTP password not configured".to_string()
            }
        })?;

        let subject = format!("BearDog Security Alert: {}", 
            metadata.get("alert_type").and_then(|v| v.as_str()).unwrap_or("General"));

        let body = format!(
            "BearDog Security Notification\n\n{}\n\nTimestamp: {}\nMetadata: {}\n\n--\nBearDog Security System",
            message,
            chrono::Utc::now().to_rfc3339(),
            serde_json::to_string_pretty(metadata).unwrap_or_default()
        );

        // Create SMTP client and send email
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport};

        let email = Message::builder()
            .from(format!("BearDog Security <{}>", username).parse()
                .map_err(|e| beardog_errors::BearDogError::External {
                    message: format!("Invalid from email address: {}", e)
                })?)
            .to(format!("Security Team <{}>", username).parse()
                .map_err(|e| beardog_errors::BearDogError::External {
                    message: format!("Invalid to email address: {}", e)
                })?)
            .subject(subject)
            .body(body)
            .map_err(|e| beardog_errors::BearDogError::External {
                message: format!("Failed to build email message: {}", e)
            })?;

        let creds = Credentials::new(username.clone(), password.clone());

        let mailer = SmtpTransport::relay(smtp_server)
            .map_err(|e| beardog_errors::BearDogError::External {
                message: format!("Failed to create SMTP transport: {}", e)
            })?
            .port(smtp_port)
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => {
                info!("✅ Email notification sent successfully");
                Ok(())
            }
            Err(e) => {
                error!("❌ Failed to send email notification: {}", e);
                Err(beardog_errors::BearDogError::External {
                    message: format!("Failed to send email notification: {}", e)
                })
            }
        }
    }

    /// Send SMS notification
    async fn send_sms_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        if !self.config.sms_enabled {
            return Ok(());
        }

        let provider = self.config.sms_provider.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS provider not configured".to_string()
            }
        })?;

        let api_key = self.config.sms_api_key.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS API key not configured".to_string()
            }
        })?;

        let phone_number = self.config.sms_phone_number.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMS phone number not configured".to_string()
            }
        })?;

        let sms_body = format!(
            "BearDog Security: {}\nTime: {}\nRef: {}",
            message,
            chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"),
            metadata.get("incident_id").and_then(|v| v.as_str()).unwrap_or("N/A")
        );

        match provider.as_str() {
            "twilio" => self.send_twilio_sms(api_key, phone_number, &sms_body).await,
            "aws_sns" => self.send_aws_sns_sms(api_key, phone_number, &sms_body).await,
            _ => {
                error!("❌ Unsupported SMS provider: {}", provider);
                Err(beardog_errors::BearDogError::Configuration {
                    message: format!("Unsupported SMS provider: {}", provider)
                })
            }
        }
    }

    /// Send SMS via Twilio
    async fn send_twilio_sms(&self, api_key: &str, phone_number: &str, message: &str) -> BearDogResult<()> {
        let client = Client::new();
        let auth = general_purpose::STANDARD.encode(format!("{}:{}", api_key, api_key)); // Twilio uses API key as both username and password
        
        let payload = json!({
            "To": phone_number,
            "From": self.config.sms_from_number.as_ref().unwrap_or(&"+1234567890".to_string()),
            "Body": message
        });

        let response = client
            .post("https://api.twilio.com/2010-04-01/Accounts/{account_sid}/Messages.json")
            .header("Authorization", format!("Basic {}", auth))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            info!("✅ SMS sent successfully via Twilio");
            Ok(())
        } else {
            let error_body = response.text().await.unwrap_or_default();
            error!("❌ Failed to send SMS via Twilio: {}", error_body);
            Err(beardog_errors::BearDogError::External {
                message: format!("Failed to send SMS via Twilio: {}", error_body)
            })
        }
    }

    /// Send SMS via AWS SNS
    async fn send_aws_sns_sms(&self, _api_key: &str, phone_number: &str, message: &str) -> BearDogResult<()> {
        // For AWS SNS, we would use the AWS SDK
        // This is a simplified implementation
        info!("📱 SMS notification prepared for AWS SNS");
        info!("  Phone: {}", phone_number);
        info!("  Message: {}", message);
        
        // In a real implementation, this would use the AWS SDK
        // let sns_client = aws_sdk_sns::Client::new(&aws_config);
        // let result = sns_client.publish()
        //     .phone_number(phone_number)
        //     .message(message)
        //     .send()
        //     .await?;
        
        Ok(())
    }

    /// Send Slack notification
    async fn send_slack_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        if let Some(slack_url) = &self.config.slack_webhook_url {
            // Create Slack message format
            let mut attachments = Vec::new();
            if !metadata.is_empty() {
                let mut fields = Vec::new();
                for (key, value) in metadata {
                    fields.push(serde_json::json!({
                        "title": key,
                        "value": value.to_string(),
                        "short": true
                    }));
                }
                attachments.push(serde_json::json!({
                    "color": "warning",
                    "fields": fields,
                    "ts": chrono::Utc::now().timestamp()
                }));
            }

            let payload = serde_json::json!({
                "text": format!("🛡️ BearDog Security Alert: {}", message),
                "username": "BearDog Security",
                "icon_emoji": ":shield:",
                "attachments": attachments
            });

            // Create HTTP client with timeout
            let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .build()?;

            // Send Slack webhook with retry logic
            let mut last_error = None;
            let max_retries = 3;
            
            for attempt in 1..=max_retries {
                match client
                    .post(slack_url)
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("✅ Slack notification sent successfully");
                            return Ok(());
                        } else {
                            let status = response.status();
                            let error_body = response.text().await.unwrap_or_default();
                            last_error = Some(format!("HTTP {}: {}", status, error_body));
                            warn!("❌ Slack notification failed (attempt {}/{}): HTTP {}", attempt, max_retries, status);
                        }
                    }
                    Err(e) => {
                        last_error = Some(e.to_string());
                        warn!("❌ Slack notification failed (attempt {}/{}): {}", attempt, max_retries, e);
                    }
                }
                
                // Wait before retry (exponential backoff)
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt - 1))).await;
                }
            }

            Err(beardog_errors::BearDogError::External {
                message: format!(
                    "Failed to send Slack notification after {} attempts. Last error: {}",
                    max_retries,
                    last_error.unwrap_or_default()
                )
            })
        } else {
            debug!("Slack webhook URL not configured, skipping notification");
            Ok(())
        }
    }

    /// Send Teams notification
    async fn send_teams_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        if let Some(teams_url) = &self.config.teams_webhook_url {
            // Create Teams message format (Adaptive Card)
            let mut facts = Vec::new();
            for (key, value) in metadata {
                facts.push(serde_json::json!({
                    "name": key,
                    "value": value.to_string()
                }));
            }

            let payload = serde_json::json!({
                "@type": "MessageCard",
                "@context": "http://schema.org/extensions",
                "themeColor": "FF6B35",
                "summary": "BearDog Security Notification",
                "sections": [{
                    "activityTitle": "🛡️ BearDog Security System",
                    "activitySubtitle": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    "activityImage": "https://raw.githubusercontent.com/microsoft/fluentui-system-icons/master/assets/Shield/SVG/ic_fluent_shield_20_filled.svg",
                    "facts": facts,
                    "markdown": true,
                    "text": message
                }]
            });

            // Create HTTP client with timeout
            let client = Client::builder()
                .timeout(Duration::from_secs(30))
                .build()?;

            // Send Teams webhook with retry logic
            let mut last_error = None;
            let max_retries = 3;
            
            for attempt in 1..=max_retries {
                match client
                    .post(teams_url)
                    .header("Content-Type", "application/json")
                    .json(&payload)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("✅ Teams notification sent successfully");
                            return Ok(());
                        } else {
                            let status = response.status();
                            let error_body = response.text().await.unwrap_or_default();
                            last_error = Some(format!("HTTP {}: {}", status, error_body));
                            warn!("❌ Teams notification failed (attempt {}/{}): HTTP {}", attempt, max_retries, status);
                        }
                    }
                    Err(e) => {
                        last_error = Some(e.to_string());
                        warn!("❌ Teams notification failed (attempt {}/{}): {}", attempt, max_retries, e);
                    }
                }
                
                // Wait before retry (exponential backoff)
                if attempt < max_retries {
                    tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt - 1))).await;
                }
            }

            Err(beardog_errors::BearDogError::External {
                message: format!(
                    "Failed to send Teams notification after {} attempts. Last error: {}",
                    max_retries,
                    last_error.unwrap_or_default()
                )
            })
        } else {
            debug!("Teams webhook URL not configured, skipping notification");
            Ok(())
        }
    }

    /// Test notification configuration
    pub async fn test_notifications(&self) -> BearDogResult<HashMap<String, bool>> {
        let mut results = HashMap::new();

        // Test email
        if self.config.email_enabled {
            let email_result = self.test_email_config().await;
            results.insert("email".to_string(), email_result.is_ok());
        }

        // Test SMS
        if self.config.sms_enabled {
            let sms_result = self.test_sms_config().await;
            results.insert("sms".to_string(), sms_result.is_ok());
        }

        // Test webhook
        if self.config.webhook_enabled {
            let webhook_result = self.test_webhook_config().await;
            results.insert("webhook".to_string(), webhook_result.is_ok());
        }

        // Test Slack
        if self.config.slack_enabled {
            let slack_result = self.test_slack_config().await;
            results.insert("slack".to_string(), slack_result.is_ok());
        }

        // Test Teams
        if self.config.teams_enabled {
            let teams_result = self.test_teams_config().await;
            results.insert("teams".to_string(), teams_result.is_ok());
        }

        Ok(results)
    }

    async fn test_email_config(&self) -> BearDogResult<()> {
        // TODO: Implement email configuration test
        Ok(())
    }

    async fn test_sms_config(&self) -> BearDogResult<()> {
        // TODO: Implement SMS configuration test
        Ok(())
    }

    async fn test_webhook_config(&self) -> BearDogResult<()> {
        // TODO: Implement webhook configuration test
        Ok(())
    }

    async fn test_slack_config(&self) -> BearDogResult<()> {
        // TODO: Implement Slack configuration test
        Ok(())
    }

    async fn test_teams_config(&self) -> BearDogResult<()> {
        // TODO: Implement Teams configuration test
        Ok(())
    }
}
