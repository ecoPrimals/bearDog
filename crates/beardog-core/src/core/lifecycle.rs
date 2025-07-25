//! BearDog Core Lifecycle Management
//!
//! Manages the startup, shutdown, and health monitoring of BearDog core.

use super::BearDogCore;
use crate::types::{ComponentStatus, HealthCheck, HealthStatus, SystemMetrics};
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

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
                .await;
            self.register_component(&mut *state, "genetic_optimizer", true, None)
                .await;
            state.overall_health = HealthStatus::healthy();
        }

        info!("✅ BearDog Core startup completed successfully");
        Ok(())
    }

    /// Shutdown all components gracefully
    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("🛑 BearDog Core shutdown initiated");

        // Update state to stopping
        {
            let mut state = self.state.write().await;
            for (name, status) in state.components.iter_mut() {
                *status = ComponentStatus::Stopping;
            }
        }

        // Graceful shutdown would happen here
        // For now, just mark as stopped
        {
            let mut state = self.state.write().await;
            for (name, status) in state.components.iter_mut() {
                *status = ComponentStatus::Stopped;
            }
            state.overall_health = HealthStatus::unhealthy("System shutting down".to_string());
        }

        info!("✅ BearDog Core shutdown completed");
        Ok(())
    }

    /// Perform comprehensive health check
    pub async fn health_check(&self) -> BearDogResult<HealthCheck> {
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
}
