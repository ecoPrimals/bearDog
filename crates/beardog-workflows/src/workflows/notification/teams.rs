

use super::super::types::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{debug, info};
impl NotificationEngine {

    pub(super) async fn send_teams_notification(
        &self,
        message: &str,
        _metadata: &HashMap<&str, serde_json::Value>,
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
        }
    }

    pub(super) async fn test_teams_config(&self) -> BearDogResult<()> {
                    "Testing Teams configuration for webhook: {}",
                    config.webhook_url
                info!("Teams configuration not found, skipping test");
}
