// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// SMS notification functionality - simplified placeholder implementation

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
        }
    }
    /// Test SMS configuration
    pub(super) async fn test_sms_config(&self) -> BearDogResult<()> {
                    "Testing SMS configuration for provider: {}",
                    config.provider
                info!("SMS configuration not found, skipping test");
}
