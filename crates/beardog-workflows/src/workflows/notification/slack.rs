//! Slack notification functionality
//!
//! This module handles Slack webhook notifications with rich formatting,
//! attachments, and proper retry logic for reliability.

use super::super::types::*;
use beardog_errors::BearDogResult;

use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, warn};

impl NotificationEngine {
    /// Send Slack notification via webhook
    pub(super) async fn send_slack_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        let slack_url = match &self.config.slack_webhook_url {
            Some(url) => url,
            None => {
                debug!("Slack webhook URL not configured, skipping notification");
                return Ok(());
            }
        };

        let payload = self.format_slack_message(message, metadata);
        self.send_slack_webhook(slack_url, &payload).await
    }

    /// Test Slack configuration
    pub(super) async fn test_slack_config(&self) -> BearDogResult<()> {
        if !self.config.slack_enabled {
            return Ok(());
        }
        
        // Validate webhook URL is configured
        self.config.slack_webhook_url.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "Slack webhook URL not configured".to_string(),
            }
        })?;

        // Basic validation that URL looks like a Slack webhook
        if let Some(url) = &self.config.slack_webhook_url {
            if !url.starts_with("https://hooks.slack.com/") {
                return Err(beardog_errors::BearDogError::Configuration {
                    message: format!("Slack webhook URL '{url}' must start with 'https://hooks.slack.com/'"),
                });
            }
        }

        Ok(())
    }

    // Private helper methods for Slack functionality

    fn format_slack_message(&self, message: &str, metadata: &HashMap<String, serde_json::Value>) -> serde_json::Value {
        let mut attachments = Vec::new();
        
        // Add metadata as attachment fields if present
        if !metadata.is_empty() {
            let mut fields = Vec::new();
            for (key, value) in metadata {
                fields.push(json!({
                    "title": key,
                    "value": value.to_string(),
                    "short": true
                }));
            }
            
            attachments.push(json!({
                "color": self.get_alert_color(metadata),
                "fields": fields,
                "ts": chrono::Utc::now().timestamp()
            }));
        }

        json!({
            "text": format!("🛡️ BearDog Security Alert: {}", message),
            "username": "BearDog Security",
            "icon_emoji": ":shield:",
            "attachments": attachments
        })
    }

    fn get_alert_color(&self, metadata: &HashMap<String, serde_json::Value>) -> &'static str {
        // Determine color based on alert severity
        if let Some(severity) = metadata.get("severity").and_then(|v| v.as_str()) {
            match severity.to_lowercase().as_str() {
                "critical" | "high" => "danger",
                "medium" | "warning" => "warning",
                "low" | "info" => "good",
                _ => "warning"
            }
        } else {
            "warning"
        }
    }

    async fn send_slack_webhook(&self, url: &str, payload: &serde_json::Value) -> BearDogResult<()> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| beardog_errors::BearDogError::Network {
                message: format!("Failed to create HTTP client: {e}")
            })?;

        // Retry logic with exponential backoff
        let mut last_error = None;
        let max_retries = 3;
        
        for attempt in 1..=max_retries {
            match client
                .post(url)
                .header("Content-Type", "application/json")
                .json(payload)
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
                        last_error = Some(format!("HTTP {status}: {error_body}"));
                        warn!("❌ Slack notification failed (attempt {}/{}): HTTP {}", attempt, max_retries, status);
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    warn!("❌ Slack notification failed (attempt {}/{}): {}", attempt, max_retries, e);
                }
            }
            
            // Exponential backoff
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
    }
} 