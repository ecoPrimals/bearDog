//! BearDog Core Component Status Management
//!
//! Manages the status and health of individual BearDog components.

use super::BearDogCore;
use crate::types::{ComponentStatus, CoreState, HealthStatus};
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;

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
        let component_status = ComponentStatus {
            name: name.to_string(),
            healthy,
            error_message,
            last_checked: now,
            last_check: now,
            uptime: state.start_time.map(|start_time| now - start_time),
            metadata: HashMap::new(),
        };

        state
            .component_status
            .insert(name.to_string(), component_status);
    }

    /// Update component status
    pub async fn update_component_status(
        &self,
        component_name: &str,
        healthy: bool,
        error_message: Option<String>,
    ) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        let start_time = state.start_time; // Get start_time before mutable borrow

        if let Some(status) = state.component_status.get_mut(component_name) {
            status.healthy = healthy;
            status.last_check = Utc::now();
            status.error_message = error_message;
            status.uptime = start_time.map(|st| Utc::now() - st);
        } else {
            return Err(BearDogError::not_found(format!(
                "component {component_name}"
            )));
        }

        // Update overall health status based on component health
        let all_healthy = state.component_status.values().all(|s| s.healthy);
        let any_healthy = state.component_status.values().any(|s| s.healthy);

        state.health_status = if all_healthy {
            HealthStatus::Healthy
        } else if any_healthy {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };

        Ok(())
    }
}
