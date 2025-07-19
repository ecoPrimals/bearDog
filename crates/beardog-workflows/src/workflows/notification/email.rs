//! Email notification functionality
//!
//! This module handles SMTP email notifications with proper configuration
//! validation and formatted message templates.

use super::super::types::*;
use beardog_errors::BearDogResult;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::collections::HashMap;
use tracing::{error, info};

impl NotificationEngine {
    /// Send email notification via SMTP
    pub(super) async fn send_email_notification(
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

        let subject = self.format_email_subject(metadata);
        let body = self.format_email_body(message, metadata);

        let email = self.build_email_message(username, &subject, &body)?;
        let mailer = self.build_smtp_transport(smtp_server, smtp_port, username, password)?;

        match mailer.send(&email) {
            Ok(_) => {
                info!("✅ Email notification sent successfully");
                Ok(())
            }
            Err(e) => {
                error!("❌ Failed to send email notification: {}", e);
                Err(beardog_errors::BearDogError::External {
                    message: format!("Failed to send email notification: {e}")
                })
            }
        }
    }

    /// Test email configuration
    pub(super) async fn test_email_config(&self) -> BearDogResult<()> {
        if !self.config.email_enabled {
            return Ok(());
        }
        
        // Validate required configuration
        self.config.smtp_server.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMTP server not configured".to_string(),
            }
        })?;
        
        self.config.smtp_username.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMTP username not configured".to_string(),
            }
        })?;
        
        self.config.smtp_password.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "SMTP password not configured".to_string(),
            }
        })?;

        Ok(())
    }

    // Private helper methods for email functionality

    fn format_email_subject(&self, metadata: &HashMap<String, serde_json::Value>) -> String {
        let alert_type = metadata
            .get("alert_type")
            .and_then(|v| v.as_str())
            .unwrap_or("General");
        format!("BearDog Security Alert: {alert_type}")
    }

    fn format_email_body(&self, message: &str, metadata: &HashMap<String, serde_json::Value>) -> String {
        format!(
            "BearDog Security Notification\n\n{}\n\nTimestamp: {}\nMetadata: {}\n\n--\nBearDog Security System",
            message,
            chrono::Utc::now().to_rfc3339(),
            serde_json::to_string_pretty(metadata).unwrap_or_default()
        )
    }

    fn build_email_message(&self, username: &str, subject: &str, body: &str) -> BearDogResult<Message> {
        Message::builder()
            .from(format!("BearDog Security <{username}>").parse()
                .map_err(|e| beardog_errors::BearDogError::External {
                    message: format!("Invalid from email address: {e}")
                })?)
            .to(format!("Security Team <{username}>").parse()
                .map_err(|e| beardog_errors::BearDogError::External {
                    message: format!("Invalid to email address: {e}")
                })?)
            .subject(subject)
            .body(body.to_string())
            .map_err(|e| beardog_errors::BearDogError::External {
                message: format!("Failed to build email message: {e}")
            })
    }

    fn build_smtp_transport(&self, server: &str, port: u16, username: &str, password: &str) -> Result<SmtpTransport, beardog_errors::BearDogError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let transport = SmtpTransport::relay(server)
            .map_err(|e| beardog_errors::BearDogError::External {
                message: format!("Failed to create SMTP transport: {e}")
            })?
            .port(port)
            .credentials(creds)
            .build();
        Ok(transport)
    }
} 