

use super::super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{debug, info};
impl NotificationEngine {

    pub(super) async fn send_sms_notification(
        &self,
        message: &str,
        _metadata: &HashMap<&str, serde_json::Value>,
    ) -> BearDogResult<()> {
        match &self.config.sms {
            Some(config) => {
                info!(
                    "Sending SMS notification via {} from {}: {}",
                    config.provider, config.from_number, message
                );
                Ok(())
            }
            None => {
                debug!("SMS not configured, skipping notification");
        }
    }

    pub(super) async fn test_sms_config(&self) -> BearDogResult<()> {
                    "Testing SMS configuration for provider: {}",
                    config.provider
                info!("SMS configuration not found, skipping test");
}
