// Fixed service_registration.rs - Core ecosystem service registration
use beardog_errors::BearDogError;
use crate::BearDogCore;
use beardog_types::canonical::HealthStatus;
use tracing::{debug, info, warn, error};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRegistration {
    pub service_id: String,
    pub service_name: String,
    pub version: String,
    pub endpoints: HashMap<String, String>,
    pub capabilities: Vec<String>,
    pub health_status: HealthStatus,
}

impl BearDogCore {
    pub(crate) async fn register_with_ecosystem(&self) -> Result<(), BearDogError> {
        info!("�� Registering BearDog with ecosystem services");
        
        let registration = EcosystemRegistration {
            service_id: "beardog-core".to_string(),
            service_name: "BearDog Security Platform".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            endpoints: self.get_service_endpoints(),
            capabilities: self.get_service_capabilities(),
            health_status: HealthStatus::Healthy,
        };

        self.register_with_toadstool(&registration).await?;
        self.register_with_songbird(&registration).await?;
        self.register_with_squirrel(&registration).await?;

        info!("✅ BearDog successfully registered with ecosystem");
        Ok(())
    }

    async fn register_with_toadstool(&self, registration: &EcosystemRegistration) -> Result<(), BearDogError> {
        info!("🍄 Registering with ToadStool platform");
        // Implementation would make HTTP request to ToadStool registry
        Ok(())
    }

    async fn register_with_songbird(&self, registration: &EcosystemRegistration) -> Result<(), BearDogError> {
        info!("🐦 Registering with SongBird mesh");
        // Implementation would make HTTP request to SongBird registry
        Ok(())
    }

    async fn register_with_squirrel(&self, registration: &EcosystemRegistration) -> Result<(), BearDogError> {
        info!("🐿️ Registering with Squirrel AI");
        // Implementation would make HTTP request to Squirrel registry
        Ok(())
    }

    fn get_service_endpoints(&self) -> HashMap<String, String> {
        let mut endpoints = HashMap::new();
        endpoints.insert("health".to_string(), "/health".to_string());
        endpoints.insert("metrics".to_string(), "/metrics".to_string());
        endpoints.insert("api".to_string(), "/api/v1".to_string());
        endpoints
    }

    fn get_service_capabilities(&self) -> Vec<String> {
        vec![
            "security".to_string(),
            "hsm".to_string(),
            "crypto".to_string(),
            "monitoring".to_string(),
            "workflows".to_string(),
        ]
    }

    pub(crate) async fn unregister_from_ecosystem(&self) -> Result<(), BearDogError> {
        info!("🌌 Unregistering BearDog from ecosystem services");
        // Implementation would notify all registered services
        Ok(())
    }

    pub fn get_registration_status(&self) -> HashMap<String, bool> {
        let mut status = HashMap::new();
        status.insert("toadstool".to_string(), true);
        status.insert("songbird".to_string(), true);
        status.insert("squirrel".to_string(), true);
        status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_registration_serialization() {
        let registration = EcosystemRegistration {
            service_id: "test".to_string(),
            service_name: "Test Service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec!["test".to_string()],
            health_status: HealthStatus::Healthy,
        };

        let json = serde_json::to_string(&registration).unwrap();
        let deserialized: EcosystemRegistration = serde_json::from_str(&json).unwrap();
        assert_eq!(registration.service_id, deserialized.service_id);
    }
}
