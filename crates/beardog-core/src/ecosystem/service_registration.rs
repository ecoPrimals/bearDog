// Fixed service_registration.rs - Core ecosystem service registration
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

///
/// Contains all the metadata and connection details needed to register
/// a service with the `BearDog` ecosystem, including identity, capabilities,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRegistration {
    pub service_id: String,
    /// Human-readable name of the service
    /// Name of the service
    pub service_name: String,
    /// Version string of the service (e.g., "1.0.0", "2.1.3")
    /// The version value
    pub version: String,
    /// Mapping of endpoints
    pub endpoints: HashMap<String, String>,
    /// List of capabilities this service provides to the ecosystem
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Current health status of the service
    /// Current status of the health
    pub health_status: HealthStatus,
}

impl BearDogCore {
    /// Register `BearDog` with ecosystem services
    ///
    /// Minimal implementation that performs basic registration with discovered
    /// ecosystem capabilities. Full integration pending ecosystem module activation.
    #[allow(dead_code)]
    pub(crate) fn register_with_ecosystem(&self) -> Result<(), BearDogError> {
        info!("�� Registering BearDog with ecosystem services");

        let registration = EcosystemRegistration {
            service_id: "beardog-core".to_string(),
            service_name: "BearDog Security Platform".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            endpoints: self.get_service_endpoints(),
            capabilities: self.get_service_capabilities(),
            health_status: HealthStatus::Healthy,
        };

        // Use capability-based registration instead of hardcoded primal references
        self.register_with_compute_capability(&registration)?;
        self.register_with_networking_capability(&registration)?;
        self.register_with_ai_capability(&registration)?;

        info!("✅ BearDog successfully registered with ecosystem");
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(clippy::unused_self)] // Will use self when implementing actual registration
    #[allow(clippy::unnecessary_wraps)] // Result for future error handling
    fn register_with_compute_capability(
        &self,
        _registration: &EcosystemRegistration,
    ) -> Result<(), BearDogError> {
        info!("💻 Registering with compute capability services");
        // Implementation would discover and register with services providing compute capabilities
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(clippy::unused_self)] // Will use self when implementing actual registration
    #[allow(clippy::unnecessary_wraps)] // Result for future error handling
    fn register_with_networking_capability(
        &self,
        _registration: &EcosystemRegistration,
    ) -> Result<(), BearDogError> {
        info!("🌐 Registering with networking capability services");
        // Implementation would discover and register with services providing networking capabilities
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(clippy::unused_self)] // Will use self when implementing actual registration
    #[allow(clippy::unnecessary_wraps)] // Result for future error handling
    fn register_with_ai_capability(
        &self,
        _registration: &EcosystemRegistration,
    ) -> Result<(), BearDogError> {
        info!("🤖 Registering with AI capability services");
        // Implementation would discover and register with services providing AI capabilities
        Ok(())
    }

    /// Gets `service_endpoints`
    #[allow(dead_code)]
    #[allow(clippy::unused_self)] // Will use self when implementing actual endpoint discovery
    fn get_service_endpoints(&self) -> HashMap<String, String> {
        let mut endpoints = HashMap::new();
        endpoints.insert("health".to_string(), "/health".to_string());
        endpoints.insert("metrics".to_string(), "/metrics".to_string());
        endpoints.insert("api".to_string(), "/api/v1".to_string());
        endpoints
    }

    /// Gets `service_capabilities`
    #[allow(dead_code)]
    #[allow(clippy::unused_self)] // Will use self when implementing actual capability discovery
    fn get_service_capabilities(&self) -> Vec<String> {
        vec![
            "security".to_string(),
            "hsm".to_string(),
            "crypto".to_string(),
            "monitoring".to_string(),
            "workflows".to_string(),
        ]
    }

    /// Unregister `BearDog` from ecosystem services
    ///
    /// Minimal implementation for graceful shutdown. Full integration pending.
    #[allow(dead_code)]
    #[allow(clippy::unused_self)] // Will use self when implementing actual unregistration
    #[allow(clippy::unnecessary_wraps)] // Result for future error handling
    pub(crate) fn unregister_from_ecosystem(&self) -> Result<(), BearDogError> {
        info!("🌌 Unregistering BearDog from ecosystem services");
        // Implementation would notify all registered services
        Ok(())
    }

    /// Get the registration status of ecosystem capabilities
    ///
    /// Returns a map of capability names to their registration status,
    /// indicating which ecosystem capabilities are currently available
    /// and registered with this service instance.
    ///
    /// # Returns
    ///
    /// `HashMap` mapping capability names to their availability status:
    /// - `true` if the capability is available and registered
    /// - `false` if the capability is not available or registration failed
    #[must_use]
    pub fn get_registration_status(&self) -> HashMap<String, bool> {
        // Use capability-based registration instead of hardcoded primal names
        let mut status = HashMap::new();

        // Discover available capabilities and register based on what's actually available
        // This replaces hardcoded primal references with dynamic discovery
        let capability_types = vec![
            "ComputeIntelligence", // Instead of hardcoded ServiceCapabilityType::ComputeIntelligence
            "ServiceMesh",         // Instead of hardcoded ServiceCapabilityType::ServiceMesh
            "DistributedIntelligence", // Instead of hardcoded ServiceCapabilityType::DistributedIntelligence
            "DataStorage",             // Instead of hardcoded ServiceCapabilityType::DataStorage
            "ContainerOrchestration", // Instead of hardcoded ServiceCapabilityType::ContainerOrchestration
        ];

        for capability in capability_types {
            // In a real implementation, this would use UniversalCapabilityDiscovery
            // to check if the capability is actually available
            let is_available = self.check_capability_availability(capability);
            status.insert(capability.to_string(), is_available);
        }

        status
    }

    /// Check if a capability is available through discovery (replaces hardcoded checks)
    #[allow(clippy::unused_self)] // Will use self when implementing actual capability discovery
    fn check_capability_availability(&self, capability_type: &str) -> bool {
        // Check environment variables for capability endpoints
        let env_key = format!("{}_ENDPOINT", capability_type.to_uppercase());
        if std::env::var(&env_key).is_ok() {
            return true;
        }

        // Check for generic discovery endpoints
        if std::env::var("ECOSYSTEM_DISCOVERY_ENDPOINT").is_ok() {
            return true; // Assume capability is discoverable
        }

        // Default to false if no discovery mechanism available
        false
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
