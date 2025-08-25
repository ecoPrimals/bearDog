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


/// BearDog Core Lifecycle Management
///
/// Manages the startup, shutdown, and health monitoring of BearDog core.

use super::BearDogCore;
use crate::types::{ComponentStatus, HealthCheck, HealthStatus};
use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SystemResult;
use chrono::Utc;
use std::collections::HashMap;
use tracing::info;
impl BearDogCore {
    /// Start all core components
    pub async fn startup(&self) -> BearDogResult<()> {
        info!("🚀 BearDog Core startup initiated");
        // Initialize components in order
        self.security.initialize().await?;
        self.monitor.start().await?;
        self.genetic_optimizer.initialize().await?;
        // Update state
        {
            let mut state = self.state.write().await;
            self.register_component(&mut *state, "security", true, None)
                .await;
            self.register_component(&mut *state, "monitor", true, None)
            self.register_component(&mut *state, "genetic_optimizer", true, None)
            state.overall_health = HealthStatus::healthy();
        }
        info!("✅ BearDog Core startup completed successfully");
        Ok(())
    }
    /// Shutdown all components gracefully
    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("🛑 BearDog Core shutdown initiated");
        // Update state to stopping
            for (name, status) in state.components.iter_mut() {
                *status = ComponentStatus::Stopping;
            }
        // Graceful shutdown would happen here
        // For now, just mark as stopped
                *status = ComponentStatus::Stopped;
            state.overall_health = HealthStatus::unhealthy("System shutting down".to_string());
        info!("✅ BearDog Core shutdown completed");
    /// Perform comprehensive health check
    pub async fn health_check(&self) -> Result<HealthCheck, SystemError> {
        let state = self.state.read().await;
        let overall_healthy = state.components.values().all(|status| status.healthy());
        let health_check = HealthCheck {
            component_name: "core".to_string(),
            status: if overall_healthy {
                ComponentStatus::Running
            } else {
                ComponentStatus::Error("Some components unhealthy".to_string())
            },
            last_check: Utc::now(),
            details: HashMap::new(),
            uptime: None, // Add the missing uptime field
        };
        Ok(health_check)
}
