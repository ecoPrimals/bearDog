//! SMS notification functionality - simplified placeholder implementation

use super::super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{debug, info};

impl NotificationEngine {
    /// Send SMS notification
    pub(super) async fn send_sms_notification(
        &self,
        message: &str,
        _metadata: &HashMap<String, serde_json::Value>,
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
                Ok(())
            }
        }
    }

    /// Test SMS configuration
    pub(super) async fn test_sms_config(&self) -> BearDogResult<()> {
        match &self.config.sms {
            Some(config) => {
                info!(
                    "Testing SMS configuration for provider: {}",
                    config.provider
                );
                Ok(())
            }
            None => {
                info!("SMS configuration not found, skipping test");
                Ok(())
            }
        }
    }
}
