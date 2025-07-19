//! Webhook notification functionality
//!
//! This module handles webhook notifications with proper retry logic,
//! HMAC signature generation, and configuration validation.

use super::super::types::*;
use beardog_errors::BearDogResult;

use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::json;
use sha2::Sha256;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn};

impl NotificationEngine {
    /// Send webhook notification with retry logic and signature
    pub(super) async fn send_webhook_notification(
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
            "source": "beardog-workflows"
        });

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        // Generate signature if webhook secret is configured
        let signature = if let Some(secret) = &self.config.webhook_secret {
            let payload_str = serde_json::to_string(&payload)?;
            Some(self.generate_webhook_signature(secret, &payload_str)?)
        } else {
            None
        };

        let mut request_builder = client
            .post(webhook_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "BearDog-Workflows/1.0")
            .json(&payload);

        if let Some(sig) = signature {
            request_builder = request_builder.header("X-Webhook-Signature", format!("sha256={sig}"));
        }

        // Retry logic with exponential backoff
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
                        last_error = Some(format!("HTTP {status}: {error_body}"));
                        warn!("❌ Webhook notification failed (attempt {}/{}): HTTP {}", attempt, max_retries, status);
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    warn!("❌ Webhook notification failed (attempt {}/{}): {}", attempt, max_retries, e);
                }
            }
            
            // Exponential backoff
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

    /// Generate HMAC-SHA256 signature for webhook payload
    pub(super) fn generate_webhook_signature(&self, secret: &str, payload: &str) -> BearDogResult<String> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| beardog_errors::BearDogError::Crypto {
                message: format!("HMAC key error: {e}")
            })?;
        
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        
        Ok(hex::encode(result.into_bytes()))
    }

    /// Test webhook configuration
    pub(super) async fn test_webhook_config(&self) -> BearDogResult<()> {
        if !self.config.webhook_enabled {
            return Ok(());
        }

        // Validate webhook URL is configured
        self.config.webhook_url.as_ref().ok_or_else(|| {
            beardog_errors::BearDogError::Configuration {
                message: "Webhook URL not configured".to_string(),
            }
        })?;

        // Basic validation that URL looks like a URL
        if let Some(url) = &self.config.webhook_url {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(beardog_errors::BearDogError::Configuration {
                    message: format!("Webhook URL '{url}' must start with http:// or https://"),
                });
            }
        }

        Ok(())
    }
} 