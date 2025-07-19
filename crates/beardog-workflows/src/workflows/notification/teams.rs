//! Microsoft Teams notification functionality
//!
//! This module handles Teams webhook notifications using Adaptive Cards
//! with proper formatting and retry logic for reliability.

use super::super::types::*;
use beardog_errors::BearDogResult;

use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, warn};

impl NotificationEngine {
    /// Send Microsoft Teams notification via webhook
    pub(super) async fn send_teams_notification(
        &self,
        message: &str,
        metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        let teams_url = match &self.config.teams_webhook_url {
            Some(url) => url,
            None => {
                debug!("Teams webhook URL not configured, skipping notification");
                return Ok(());
            }
        };

        let payload = self.format_teams_message(message, metadata);
        self.send_teams_webhook(teams_url, &payload).await
    }

    /// Test Microsoft Teams configuration
    pub(super) async fn test_teams_config(&self) -> BearDogResult<()> {
        if !self.config.teams_enabled {
            return Ok(());
        }
        
        // Validate webhook URL is configured
        self.config.teams_webhook_url.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "Teams webhook URL not configured".to_string(),
            }
        })?;

        // Basic validation that URL looks like a Teams webhook
        if let Some(url) = &self.config.teams_webhook_url {
            if !url.contains("office.com") && !url.contains("outlook.office365.com") {
                return Err(beardog_errors::BearDogError::Configuration {
                    message: format!("Teams webhook URL '{url}' does not appear to be a valid Teams webhook"),
                });
            }
        }

        Ok(())
    }

    // Private helper methods for Teams functionality

    fn format_teams_message(&self, message: &str, metadata: &HashMap<String, serde_json::Value>) -> serde_json::Value {
        // Convert metadata to Teams "facts" format
        let mut facts = Vec::new();
        for (key, value) in metadata {
            facts.push(json!({
                "name": key,
                "value": value.to_string()
            }));
        }

        // Create Adaptive Card payload for Teams
        json!({
            "@type": "MessageCard",
            "@context": "http://schema.org/extensions",
            "themeColor": self.get_teams_theme_color(metadata),
            "summary": "BearDog Security Notification",
            "sections": [{
                "activityTitle": "🛡️ BearDog Security System",
                "activitySubtitle": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                "activityImage": "https://raw.githubusercontent.com/microsoft/fluentui-system-icons/master/assets/Shield/SVG/ic_fluent_shield_20_filled.svg",
                "facts": facts,
                "markdown": true,
                "text": message
            }]
        })
    }

    fn get_teams_theme_color(&self, metadata: &HashMap<String, serde_json::Value>) -> &'static str {
        // Determine theme color based on alert severity
        if let Some(severity) = metadata.get("severity").and_then(|v| v.as_str()) {
            match severity.to_lowercase().as_str() {
                "critical" | "high" => "FF0000",     // Red
                "medium" | "warning" => "FF6B35",    // Orange  
                "low" | "info" => "00AA00",          // Green
                _ => "FF6B35"                        // Default orange
            }
        } else {
            "FF6B35" // Default orange
        }
    }

    async fn send_teams_webhook(&self, url: &str, payload: &serde_json::Value) -> BearDogResult<()> {
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
                        info!("✅ Teams notification sent successfully");
                        return Ok(());
                    } else {
                        let status = response.status();
                        let error_body = response.text().await.unwrap_or_default();
                        last_error = Some(format!("HTTP {status}: {error_body}"));
                        warn!("❌ Teams notification failed (attempt {}/{}): HTTP {}", attempt, max_retries, status);
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    warn!("❌ Teams notification failed (attempt {}/{}): {}", attempt, max_retries, e);
                }
            }
            
            // Exponential backoff
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
    }
} 