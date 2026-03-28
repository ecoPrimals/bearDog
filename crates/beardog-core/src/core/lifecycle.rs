// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use chrono::Utc;
use tracing::info;

/// Health check result for a component
#[derive(Debug, Clone)]
pub struct HealthCheck {
    /// Name of the component that was checked
    /// Name of the component
    pub component_name: String,
    /// Current operational status of the component
    /// Current status of the component
    pub status: ComponentStatus,
    /// The last check value
    pub last_check: chrono::DateTime<Utc>,
    /// Optional additional details about the health check result
    /// Optional details
    pub details: Option<String>,
    /// Duration the component has been running
    pub uptime: std::time::Duration,
}

impl BearDogCore {
    /// Startup operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Starts serviceup
    pub async fn startup(&self) -> Result<(), BearDogError> {
        info!("🚀 BearDog Core startup initiated");

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("security".to_string(), ComponentStatus::Running);
            state
                .components
                .insert("monitor".to_string(), ComponentStatus::Running);
            state
                .components
                .insert("genetic_optimizer".to_string(), ComponentStatus::Running);
            state.overall_health = HealthStatus::Healthy;
        }

        info!("✅ BearDog Core startup completed successfully");
        Ok(())
    }

    /// Shutdown operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn shutdown(&self) -> Result<(), BearDogError> {
        info!("🛑 BearDog Core shutdown initiated");

        self.unregister_from_ecosystem()?;

        {
            let mut state = self.state.write().await;
            for status in state.components.values_mut() {
                *status = ComponentStatus::Inactive;
            }

            for status in state.components.values_mut() {
                *status = ComponentStatus::Inactive;
            }
            state.overall_health = HealthStatus::Unhealthy;
        }

        info!("✅ BearDog Core shutdown completed");
        Ok(())
    }

    /// Health Check operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn health_check(&self) -> Result<HealthCheck, BearDogError> {
        let state = self.state.read().await;
        let overall_healthy = state
            .components
            .values()
            .all(|status| matches!(status, ComponentStatus::Running));
        let health_check = HealthCheck {
            component_name: "core".to_string(),
            status: if overall_healthy {
                ComponentStatus::Running
            } else {
                ComponentStatus::Inactive
            },
            last_check: Utc::now(),
            details: if overall_healthy {
                None
            } else {
                Some("Some components unhealthy".to_string())
            },
            uptime: state.start_time.elapsed(),
        };
        Ok(health_check)
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod lifecycle_tests;
