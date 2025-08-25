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


/// BearDog Core Component Status Management
///
/// Manages the status and health of individual BearDog components.

use super::BearDogCore;
use crate::types::{ComponentStatus, CoreState, HealthStatus};
use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SystemResult;
use chrono::Utc;
impl BearDogCore {
    /// Register a component with the core
    pub(crate) async fn register_component(
        &self,
        state: &mut CoreState,
        name: &str,
        healthy: bool,
        error_message: Option<String>,
    ) {
        let now = Utc::now();
        let error_msg_clone = error_message.clone();
        let status = if healthy {
            ComponentStatus::Running
        } else {
            ComponentStatus::Error(error_message.unwrap_or_default())
        };
        state.components.insert(name.to_string(), status.clone());
        state.component_status.insert(name.to_string(), status);
        if healthy {
            state.health_status = HealthStatus::healthy();
            state.health_status = HealthStatus::unhealthy(
                error_msg_clone.unwrap_or_else(|| format!("Component {} unhealthy", name)),
            );
        }
    }
    /// Get component health summary
    pub async fn get_component_health(&self) -> BearDogResult<Vec<String>> {
        let state = self.state.read().await;
        let mut health_summary = Vec::new();
        for (name, status) in &state.components {
            let is_healthy = status.healthy();
            health_summary.push(format!(
                "{}: {}",
                name,
                if is_healthy { "Healthy" } else { "Unhealthy" }
            ));
        Ok(health_summary)
    /// Check if all components are healthy
    pub async fn all_components_healthy(&self) -> Result<bool, SystemError> {
        let all_healthy = state.components.values().all(|status| status.healthy());
        Ok(all_healthy)
}
