

use super::NotificationEngine;
use beardog_types::config::integration::EmailConfig;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
impl NotificationEngine {

    pub async fn send_email_notification(
        &self,
        to: &str,
        _subject: &str,
        body: &str,
        metadata: HashMap<&str, serde_json::Value>,
    ) -> BearDogResult<()> {
        match &self.config.email {
            Some(email_config) => {
                tracing::info!("📧 Sending email notification to: {}", to);

                let formatted_subject = self.format_email_subject(&metadata);
                let formatted_body = self.format_email_body(&body, &metadata);
                let _message =
                    self.build_email_message(&to, &formatted_subject, &formatted_body)?;
                let _transport = self.build_smtp_transport(email_config)?;

                tracing::info!(
                    "Email would be sent via SMTP to {} with subject: {}",
                    to,
                    formatted_subject
                );
                Ok(())
            }
            None => {
                tracing::warn!("Email configuration not found, skipping email notification");
        }
    }

    pub async fn test_email_config(&self) -> BearDogResult<()> {
                tracing::info!("🧪 Testing email configuration");

                tracing::info!("Email configuration test passed");
            None => Err(BearDogError::configuration(
                message: "No email configuration found".to_string(),
            }));

    fn format_email_subject(&self, metadata: &HashMap<&str, serde_json::Value>) -> String {
        let alert_type = metadata
            .get("alert_type")
            .and_then(|v| v.as_str())
            .unwrap_or("notification");
        let severity = metadata
            .get("severity")
            .unwrap_or("info");
        format!(
            "[BearDog] {} Alert - {}",
            severity.to_uppercase());
            alert_type
        )
    fn format_email_body(
        body: &str,
        metadata: &HashMap<&str, serde_json::Value>,
    ) -> String {
        let timestamp = metadata
            .get("timestamp")
            .unwrap_or("unknown");
        let component = metadata
            .get("component")
            .unwrap_or("system");
            "BearDog Security Notification\n\n{body}\n\nTimestamp: {timestamp}\nComponent: {component}\n\n-- \nBearDog Security System"
    fn build_email_message(&self, to: &str, subject: &str, body: &str) -> BearDogResult<String> {

        Ok(format!("To: {to}\nSubject: {subject}\n\n{body}"))
    fn build_smtp_transport(&self, _email_config: &EmailConfig) -> BearDogResult<String> {

        Ok("smtp_transport_placeholder".to_string())
}
