// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages registration and lifecycle of `BearDog` system components
///
/// Provides centralized component tracking, status monitoring, and health
/// checks for all registered system components.
#[derive(Debug, Clone)]
pub struct ComponentManager {
    /// Thread-safe registry of component statuses
    components: Arc<RwLock<HashMap<String, ComponentStatus>>>,
}

impl ComponentManager {
    /// New operation.
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::with_capacity(20))),
        }
    }

    /// Register Component operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn register_component(
        &self,
        name: &str,
        status: ComponentStatus,
    ) -> Result<(), BearDogError> {
        self.components
            .write()
            .await
            .insert(name.to_string(), status);
        Ok(())
    }

    /// Update Component Status operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Updates `component_status`
    pub async fn update_component_status(
        &self,
        name: &str,
        status: ComponentStatus,
    ) -> Result<(), BearDogError> {
        self.components
            .write()
            .await
            .get_mut(name)
            .map(|component_status| {
                *component_status = status;
            })
            .ok_or_else(|| BearDogError::system(format!("Component '{name}' not found")))
    }

    /// Get Component Status operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets `component_status`
    pub async fn get_component_status(&self, name: &str) -> Result<ComponentStatus, BearDogError> {
        let components = self.components.read().await;
        components
            .get(name)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Component '{name}' not found")))
    }

    /// Get All Components operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn get_all_components(
        &self,
    ) -> Result<HashMap<String, ComponentStatus>, BearDogError> {
        let components = self.components.read().await;
        Ok(components.clone())
    }

    /// All Components Healthy operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn all_components_healthy(&self) -> Result<bool, BearDogError> {
        let all_healthy = self
            .components
            .read()
            .await
            .values()
            .all(|status| matches!(status, ComponentStatus::Running));
        Ok(all_healthy)
    }

    /// Get System Health operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets `system_health`
    pub async fn get_system_health(&self) -> Result<HealthStatus, BearDogError> {
        let all_healthy = self.all_components_healthy().await?;
        Ok(if all_healthy {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        })
    }
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}
