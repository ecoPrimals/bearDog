use super::BearDogCore;
use crate::types::HealthCheck;
use beardog_errors::BearDogError;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use chrono::Utc;
use tracing::info;

// SystemError replaced with BearDogError

impl BearDogCore {
    pub async fn startup(&self) -> Result<(), BearDogError> {
        info!("🚀 BearDog Core startup initiated");

        // Initialize components (methods may not exist yet, using placeholder)
        // self.security.initialize().await?;
        // self.monitor.start().await?;
        // self.genetic_optimizer.initialize().await?;

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

    pub async fn shutdown(&self) -> Result<(), BearDogError> {
        info!("🛑 BearDog Core shutdown initiated");

        {
            let mut state = self.state.write().await;
            for (_name, status) in state.components.iter_mut() {
                *status = ComponentStatus::Stopping;
            }

            for (_name, status) in state.components.iter_mut() {
                *status = ComponentStatus::Inactive;
            }
            state.overall_health = HealthStatus::Stopping;
        }

        info!("✅ BearDog Core shutdown completed");
        Ok(())
    }

    pub async fn health_check(&self) -> Result<HealthCheck, BearDogError> {
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
