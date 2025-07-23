//! BearDog Core Lifecycle Management
//!
//! Manages the startup, shutdown, health checking, and metrics of BearDog core components.

use super::BearDogCore;
use crate::types::{ComponentStatus, HealthCheck, HealthStatus, SystemMetrics};
use beardog_errors::BearDogResult;
use chrono::Utc;
use tracing::info;

impl BearDogCore {
    /// Start the BearDog core and all components
    pub async fn start(&self) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        state.start_time = Some(Utc::now());
        state.health_status = HealthStatus::Starting;

        info!("Starting BearDog core components...");

        // Initialize components in order
        self.register_component(&mut state, "encryption", true, None)
            .await;
        self.register_component(&mut state, "threat_detection", true, None)
            .await;
        self.register_component(&mut state, "compliance", true, None)
            .await;
        self.register_component(&mut state, "audit", true, None)
            .await;
        self.register_component(&mut state, "workflows", true, None)
            .await;
        self.register_component(&mut state, "cross_node_auth", true, None)
            .await;
        self.register_component(&mut state, "hsm_manager", true, None)
            .await;
        self.register_component(&mut state, "nestgate_adapter", true, None)
            .await;
        self.register_component(&mut state, "songbird_adapter", true, None)
            .await;

        state.health_status = HealthStatus::Healthy;
        info!("BearDog core started successfully");

        Ok(())
    }

    /// Stop the BearDog core and all components
    pub async fn stop(&self) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        state.health_status = HealthStatus::Stopping;

        info!("Stopping BearDog core...");

        // Clear component statuses
        state.component_status.clear();

        info!("BearDog core stopped");
        Ok(())
    }

    /// Get current health status
    pub async fn health_check(&self) -> BearDogResult<HealthCheck> {
        let state = self.state.read().await;

        let uptime = state.start_time.map(|start_time| Utc::now() - start_time);

        let components: Vec<ComponentStatus> = state.component_status.values().cloned().collect();

        Ok(HealthCheck {
            component_name: "beardog-core".to_string(),
            healthy: matches!(state.health_status, HealthStatus::Healthy),
            status: state.health_status.clone(),
            uptime,
            details: None,
            check_duration_ms: 0,
            components,
            metrics: state.metrics.clone(),
            timestamp: Utc::now(),
        })
    }

    /// Update system metrics
    pub async fn update_metrics(&self, metrics: SystemMetrics) -> BearDogResult<()> {
        let mut state = self.state.write().await;
        state.metrics = metrics;
        Ok(())
    }

    /// Get health status for all components
    pub async fn get_health_status(
        &self,
    ) -> BearDogResult<std::collections::HashMap<String, String>> {
        let mut status = std::collections::HashMap::new();

        // Get current state
        let state = self.state.read().await;

        // Overall health
        status.insert("overall".to_string(), format!("{:?}", state.health_status));

        // Check individual component health
        for (name, component) in &state.component_status {
            let health = if component.healthy {
                "healthy"
            } else {
                "unhealthy"
            };
            status.insert(name.clone(), health.to_string());
        }

        // Node registry not yet implemented, report as unavailable
        status.insert("node_registry".to_string(), "not_implemented".to_string());
        status.insert("proof_verifier".to_string(), "not_implemented".to_string());

        Ok(status)
    }
}
