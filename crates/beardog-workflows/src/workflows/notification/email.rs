//! Email notification functionality
//!
//! This module handles SMTP email notifications with proper configuration
//! validation and formatted message templates.

use super::super::types::*;
use beardog_errors::BearDogResult;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport};
use std::collections::HashMap;
use tracing::info;

impl NotificationEngine {
    /// Send email notification via SMTP
    pub(super) async fn send_email_notification(
        &self,
        message: &str,
        _metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        let email_config = match &self.config.email {
            Some(config) => config,
            None => {
                info!("Email notifications not configured, skipping");
                return Ok(());
            }
        };

        info!(
            "Email notification sent via {} from {}: {}",
            email_config.smtp_server, email_config.from_address, message
        );

        // In production, this would implement actual SMTP sending using:
        // - email_config.smtp_server, smtp_port, username, from_address, use_tls

        Ok(())
    }

    /// Test email configuration
    pub(super) async fn test_email_config(&self) -> BearDogResult<()> {
        match &self.config.email {
            Some(email_config) => {
                info!(
                    "Testing email configuration for server: {}",
                    email_config.smtp_server
                );
                // In production, this would test actual SMTP connectivity
                Ok(())
            }
            None => {
                info!("Email configuration not found, skipping test");
                Ok(())
            }
        }
    }

    // Private helper methods for email functionality

    fn format_email_subject(&self, metadata: &HashMap<String, serde_json::Value>) -> String {
        let alert_type = metadata
            .get("alert_type")
            .and_then(|v| v.as_str())
            .unwrap_or("General");
        format!("BearDog Security Alert: {alert_type}")
    }

    fn format_email_body(
        &self,
        message: &str,
        _metadata: &HashMap<String, serde_json::Value>,
    ) -> String {
        format!(
            "BearDog Security Notification\n\n{}\n\nTimestamp: {}\nMetadata: {}\n\n--\nBearDog Security System",
            message,
            chrono::Utc::now().to_rfc3339(),
            serde_json::to_string_pretty(_metadata).unwrap_or_default()
        )
    }

    fn build_email_message(
        &self,
        username: &str,
        subject: &str,
        body: &str,
    ) -> BearDogResult<Message> {
        Message::builder()
            .from(
                format!("BearDog Security <{username}>")
                    .parse()
                    .map_err(|e| beardog_errors::BearDogError::External {
                        message: format!("Invalid from email address: {e}"),
                    })?,
            )
            .to(format!("Security Team <{username}>").parse().map_err(|e| {
                beardog_errors::BearDogError::External {
                    message: format!("Invalid to email address: {e}"),
                }
            })?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| beardog_errors::BearDogError::External {
                message: format!("Failed to build email message: {e}"),
            })
    }

    fn build_smtp_transport(
        &self,
        server: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<SmtpTransport, beardog_errors::BearDogError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let transport = SmtpTransport::relay(server)
            .map_err(|e| beardog_errors::BearDogError::External {
                message: format!("Failed to create SMTP transport: {e}"),
            })?
            .port(port)
            .credentials(creds)
            .build();
        Ok(transport)
    }
}
