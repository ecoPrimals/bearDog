// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ComponentManager {
    components: Arc<RwLock<HashMap<String, ComponentStatus>>>,
}

impl ComponentManager {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::with_capacity(20))),
        }
    }

    /// Register Component operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn register_component(
        &self,
        name: &str,
        status: ComponentStatus,
    ) -> Result<(), BearDogError> {
        let mut components = self.components.write();
        components.insert(name.to_string(), status);
        Ok(())
    }

    /// Update Component Status operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Updates component_status
    /// Updates component_status
    pub fn update_component_status(
        &self,
        name: &str,
        status: ComponentStatus,
    ) -> Result<(), BearDogError> {
        let mut components = self.components.write();
        if let Some(component_status) = components.get_mut(name) {
            *component_status = status;
            Ok(())
        } else {
            Err(BearDogError::system(format!(
                "Component '{name}' not found"
            )))
        }
    }

    /// Get Component Status operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets component_status
    /// Gets component_status
    pub fn get_component_status(&self, name: &str) -> Result<ComponentStatus, BearDogError> {
        let components = self.components.read();
        components
            .get(name)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Component '{name}' not found")))
    }

    /// Get All Components operation.
    /// Gets all_components
    /// Gets all_components
    pub fn get_all_components(&self) -> Result<HashMap<String, ComponentStatus>, BearDogError> {
        let components = self.components.read();
        Ok(components.clone())
    }

    /// All Components Healthy operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn all_components_healthy(&self) -> Result<bool, BearDogError> {
        let components = self.components.read();
        let all_healthy = components
            .values()
            .all(|status| matches!(status, ComponentStatus::Running));
        Ok(all_healthy)
    }

    /// Get System Health operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets system_health
    /// Gets system_health
    pub fn get_system_health(&self) -> Result<HealthStatus, BearDogError> {
        let all_healthy = self.all_components_healthy()?;
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
