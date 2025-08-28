use beardog_errors::BearDogError;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SystemError = BearDogError;

#[derive(Debug)]
pub struct ComponentManager {
    components: Arc<RwLock<HashMap<String, ComponentStatus>>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_component(
        &self,
        name: String,
        status: ComponentStatus,
    ) -> Result<(), SystemError> {
        let mut components = self.components.write().await;
        components.insert(name, status);
        Ok(())
    }

    pub async fn update_component_status(
        &self,
        name: &str,
        status: ComponentStatus,
    ) -> Result<(), SystemError> {
        let mut components = self.components.write().await;
        if let Some(component_status) = components.get_mut(name) {
            *component_status = status;
            Ok(())
        } else {
            Err(BearDogError::system(&format!(
                "Component '{}' not found",
                name
            )))
        }
    }

    pub async fn get_component_status(&self, name: &str) -> Result<ComponentStatus, SystemError> {
        let components = self.components.read().await;
        components
            .get(name)
            .cloned()
            .ok_or_else(|| BearDogError::system(&format!("Component '{}' not found", name)))
    }

    pub async fn get_all_components(
        &self,
    ) -> Result<HashMap<String, ComponentStatus>, SystemError> {
        let components = self.components.read().await;
        Ok(components.clone())
    }

    pub async fn all_components_healthy(&self) -> Result<bool, SystemError> {
        let components = self.components.read().await;
        let all_healthy = components
            .values()
            .all(|status| matches!(status, ComponentStatus::Running));
        Ok(all_healthy)
    }

    pub async fn get_system_health(&self) -> Result<HealthStatus, SystemError> {
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
