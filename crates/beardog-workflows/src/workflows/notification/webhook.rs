//! Webhook notification functionality - simplified placeholder implementation

use super::super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{debug, info};

impl NotificationEngine {
    /// Send webhook notification
    pub(super) async fn send_webhook_notification(
        &self,
        message: &str,
        _metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        match &self.config.webhook {
            Some(config) => {
                info!(
                    "Sending webhook notification to {} via {}: {}",
                    config.url, config.method, message
                );
                Ok(())
            }
            None => {
                debug!("Webhook not configured, skipping notification");
                Ok(())
            }
        }
    }

    /// Test webhook configuration
    pub(super) async fn test_webhook_config(&self) -> BearDogResult<()> {
        match &self.config.webhook {
            Some(config) => {
                info!("Testing webhook configuration for URL: {}", config.url);
                Ok(())
            }
            None => {
                info!("Webhook configuration not found, skipping test");
                Ok(())
            }
        }
    }
}
