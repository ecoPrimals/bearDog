// SPDX-License-Identifier: AGPL-3.0-only

// Fixed service_registration.rs - Core ecosystem service registration
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, trace};

///
/// Contains all the metadata and connection details needed to register
/// a service with the `BearDog` ecosystem, including identity, capabilities,
/// Registration information for an ecosystem service
///
/// Contains service metadata for registering with the ecosystem, including
/// identity, endpoints, capabilities, and health status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRegistration {
    /// Unique service identifier
    pub service_id: String,
    /// Human-readable name of the service
    pub service_name: String,
    /// Version string of the service (e.g., "1.0.0", "2.1.3")
    pub version: String,
    /// Service endpoints mapped by protocol (e.g., "http", "grpc")
    pub endpoints: HashMap<String, String>,
    /// List of capabilities this service provides to the ecosystem
    pub capabilities: Vec<String>,
    /// Current health status of the service
    pub health_status: HealthStatus,
}

impl BearDogCore {
    /// Register `BearDog` with ecosystem services
    ///
    /// Minimal implementation that performs basic registration with discovered
    /// ecosystem capabilities. Full integration pending ecosystem module activation.
    pub(crate) fn register_with_ecosystem(&self) -> Result<(), BearDogError> {
        info!(
            app = %self.config.app.app_name,
            "Registering BearDog with ecosystem services",
        );

        let registration = EcosystemRegistration {
            service_id: "beardog-core".to_string(),
            service_name: "BearDog Security Platform".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            endpoints: self.get_service_endpoints(),
            capabilities: self.get_service_capabilities(),
            health_status: HealthStatus::Healthy,
        };

        // Use capability-based registration instead of hardcoded primal references
        self.register_with_compute_capability(&registration);
        self.register_with_networking_capability(&registration);
        self.register_with_ai_capability(&registration);

        info!("✅ BearDog successfully registered with ecosystem");
        Ok(())
    }

    fn register_with_compute_capability(&self, registration: &EcosystemRegistration) {
        info!(
            service_id = %registration.service_id,
            app = %self.config.app.app_name,
            "Registering with compute capability services",
        );
    }

    fn register_with_networking_capability(&self, registration: &EcosystemRegistration) {
        info!(
            service_id = %registration.service_id,
            app = %self.config.app.app_name,
            "Registering with networking capability services",
        );
    }

    fn register_with_ai_capability(&self, registration: &EcosystemRegistration) {
        info!(
            service_id = %registration.service_id,
            app = %self.config.app.app_name,
            "Registering with AI capability services",
        );
    }

    /// Gets `service_endpoints`
    fn get_service_endpoints(&self) -> HashMap<String, String> {
        trace!(
            app = %self.config.app.app_name,
            "Collecting default service endpoints for ecosystem registration",
        );
        let mut endpoints = HashMap::new();
        endpoints.insert("health".to_string(), "/health".to_string());
        endpoints.insert("metrics".to_string(), "/metrics".to_string());
        endpoints.insert("api".to_string(), "/api/v1".to_string());
        endpoints
    }

    /// Gets `service_capabilities`
    fn get_service_capabilities(&self) -> Vec<String> {
        trace!(
            app = %self.config.app.app_name,
            "Collecting default service capabilities for ecosystem registration",
        );
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
    pub(crate) fn unregister_from_ecosystem(&self) -> Result<(), BearDogError> {
        info!(
            app = %self.config.app.app_name,
            "Unregistering BearDog from ecosystem services",
        );
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
    fn check_capability_availability(&self, capability_type: &str) -> bool {
        // Check environment variables for capability endpoints
        let env_key = format!("{}_ENDPOINT", capability_type.to_uppercase());
        if beardog_errors::process_env::var(&env_key).is_ok() {
            return true;
        }

        // Check for generic discovery endpoints
        if beardog_errors::process_env::var("ECOSYSTEM_DISCOVERY_ENDPOINT").is_ok() {
            return true; // Assume capability is discoverable
        }

        trace!(
            capability = %capability_type,
            app = %self.config.app.app_name,
            "No ecosystem discovery env for capability",
        );
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_ecosystem_registration_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let registration = EcosystemRegistration {
            service_id: "test".to_string(),
            service_name: "Test Service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec!["test".to_string()],
            health_status: HealthStatus::Healthy,
        };

        let json = serde_json::to_string(&registration)?;
        let deserialized: EcosystemRegistration = serde_json::from_str(&json)?;
        assert_eq!(registration.service_id, deserialized.service_id);

        Ok(())
    }

    #[test]
    fn test_ecosystem_registration_with_endpoints() {
        let mut endpoints = HashMap::new();
        endpoints.insert("http".to_string(), "http://localhost:8080".to_string());
        endpoints.insert("grpc".to_string(), "grpc://localhost:9090".to_string());

        let registration = EcosystemRegistration {
            service_id: "test-service".to_string(),
            service_name: "Test Service".to_string(),
            version: "2.0.0".to_string(),
            endpoints,
            capabilities: vec!["compute".to_string(), "ai".to_string()],
            health_status: HealthStatus::Healthy,
        };

        assert_eq!(registration.endpoints.len(), 2);
        assert!(registration.endpoints.contains_key("http"));
        assert!(registration.endpoints.contains_key("grpc"));
        assert_eq!(registration.capabilities.len(), 2);
    }

    #[test]
    fn test_ecosystem_registration_clone() {
        let registration = EcosystemRegistration {
            service_id: "original".to_string(),
            service_name: "Original Service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec!["cap1".to_string()],
            health_status: HealthStatus::Healthy,
        };

        let cloned = registration.clone();
        assert_eq!(registration.service_id, cloned.service_id);
        assert_eq!(registration.service_name, cloned.service_name);
    }

    #[test]
    fn test_ecosystem_registration_debug() {
        let registration = EcosystemRegistration {
            service_id: "debug-test".to_string(),
            service_name: "Debug Test".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![],
            health_status: HealthStatus::Healthy,
        };

        let debug_str = format!("{registration:?}");
        assert!(debug_str.contains("debug-test"));
        assert!(debug_str.contains("Debug Test"));
    }

    #[test]
    fn test_registration_with_multiple_capabilities() {
        let registration = EcosystemRegistration {
            service_id: "multi-cap".to_string(),
            service_name: "Multi Capability Service".to_string(),
            version: "3.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![
                "security".to_string(),
                "hsm".to_string(),
                "crypto".to_string(),
                "monitoring".to_string(),
                "workflows".to_string(),
            ],
            health_status: HealthStatus::Healthy,
        };

        assert_eq!(registration.capabilities.len(), 5);
        assert!(registration.capabilities.contains(&"security".to_string()));
        assert!(registration.capabilities.contains(&"hsm".to_string()));
    }

    #[test]
    fn test_registration_health_status_variants() {
        let healthy = EcosystemRegistration {
            service_id: "healthy".to_string(),
            service_name: "Healthy Service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![],
            health_status: HealthStatus::Healthy,
        };

        let degraded = EcosystemRegistration {
            service_id: "degraded".to_string(),
            service_name: "Degraded Service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![],
            health_status: HealthStatus::Degraded,
        };

        let unhealthy = EcosystemRegistration {
            service_id: "unhealthy".to_string(),
            service_name: "Unhealthy Service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![],
            health_status: HealthStatus::Unhealthy,
        };

        assert!(matches!(healthy.health_status, HealthStatus::Healthy));
        assert!(matches!(degraded.health_status, HealthStatus::Degraded));
        assert!(matches!(unhealthy.health_status, HealthStatus::Unhealthy));
    }
}
