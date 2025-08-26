

use super::NotificationEngine;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{debug, info};
impl NotificationEngine {

    #[allow(dead_code)] // Will be used when webhook notifications are fully implemented
    pub(super) async fn send_webhook_notification(
        &self,
        message: &str,
        _metadata: &HashMap<&str, serde_json::Value>,
    ) -> BearDogResult<()> {
        match &self.config.webhook {
            Some(config) => {
                info!(
                    "Sending webhook notification to {}: {}",
                    config.url, message
                );
                Ok(())
            }
            None => {
                debug!("Webhook not configured, skipping notification");
        }
    }

    pub(super) async fn test_webhook_config(&self) -> BearDogResult<()> {
                info!("Testing webhook configuration for URL: {}", config.url);
                info!("Webhook configuration not found, skipping test");
}
