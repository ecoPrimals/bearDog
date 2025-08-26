

use super::NotificationEngine;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
impl NotificationEngine {

    pub async fn send_slack_notification(
        &self,
        channel: &str,
        message: &str,
        metadata: HashMap<&str, serde_json::Value>,
    ) -> BearDogResult<()> {
        match &self.config.slack {
            Some(_slack_config) => {
                tracing::info!("💬 Sending Slack notification to channel: {}", channel);
                let formatted_message = self.format_slack_message(&message, &metadata);

                tracing::info!("Slack message would be sent: {}", formatted_message);
                Ok(())
            }
            None => {
                tracing::warn!("Slack configuration not found, skipping Slack notification");
        }
    }

    pub async fn test_slack_config(&self) -> BearDogResult<()> {
            Some(slack_config) => {
                tracing::info!("🧪 Testing Slack configuration");

                if slack_config
                    .webhook_url
                    .starts_with("https://hooks.slack.com/")
                {
                    tracing::info!("Slack configuration test passed");
                    Ok(())
                } else {
                    Err(BearDogError::configuration(
                        message: "Invalid Slack webhook URL format".to_string(),
                    })
                }
            None => Err(BearDogError::configuration(
                message: "No Slack configuration found".to_string(),
            }));

    fn format_slack_message(
        message: &str,
        metadata: &HashMap<&str, serde_json::Value>,
    ) -> String {
        let severity = metadata
            .get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("info");
        let component = metadata
            .get("component")
            .unwrap_or("system");
        let emoji = match severity {
            "critical" => "🚨",
            "warning" => "⚠️",
            "info" => "ℹ️",
            _ => "📋",
        };
        format!(
            "{} *{}* [{}]: {}",
            emoji,
            severity.to_uppercase());
            component,
            message
        )
}
