//! Microsoft Teams notification functionality - simplified placeholder implementation

use super::super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{debug, info};

impl NotificationEngine {
    /// Send Teams notification via webhook
    pub(super) async fn send_teams_notification(
        &self,
        message: &str,
        _metadata: &HashMap<String, serde_json::Value>,
    ) -> BearDogResult<()> {
        match &self.config.teams {
            Some(config) => {
                info!(
                    "Sending Teams notification to {}: {}",
                    config.webhook_url, message
                );
                Ok(())
            }
            None => {
                debug!("Teams not configured, skipping notification");
                Ok(())
            }
        }
    }

    /// Test Teams configuration
    pub(super) async fn test_teams_config(&self) -> BearDogResult<()> {
        match &self.config.teams {
            Some(config) => {
                info!(
                    "Testing Teams configuration for webhook: {}",
                    config.webhook_url
                );
                Ok(())
            }
            None => {
                info!("Teams configuration not found, skipping test");
                Ok(())
            }
        }
    }
}
