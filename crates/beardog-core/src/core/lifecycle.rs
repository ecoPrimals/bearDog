

use super::BearDogCore;
use crate::types::{ComponentStatus, HealthCheck, HealthStatus};
use beardog_errors::BearDogResult;
use beardog_errors::idiomatic::SystemResult;
use chrono::Utc;
use std::collections::HashMap;
use tracing::info;
impl BearDogCore {

    pub async fn startup(&self) -> BearDogResult<()> {
        info!("🚀 BearDog Core startup initiated");

        self.security.initialize().await?;
        self.monitor.start().await?;
        self.genetic_optimizer.initialize().await?;

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

    pub async fn shutdown(&self) -> BearDogResult<()> {
        info!("🛑 BearDog Core shutdown initiated");

            for (name, status) in state.components.iter_mut() {
                *status = ComponentStatus::Stopping;
            }

                *status = ComponentStatus::Stopped;
            state.overall_health = HealthStatus::unhealthy("System shutting down".to_string());
        info!("✅ BearDog Core shutdown completed");

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
            details: ahash::HashMap::default(),
            uptime: None, // Add the missing uptime field
        };
        Ok(health_check)
}
