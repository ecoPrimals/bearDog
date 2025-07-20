//! Capability Registry for Universal Ecosystem Integration
//!
//! This module handles capability-based service discovery and registration
//! following the Universal Primal Architecture Standard

use std::collections::HashMap;

use super::*;
use crate::EcosystemResult;

/// Universal capability registry for ecosystem service discovery
#[derive(Debug, Clone)]
pub struct CapabilityRegistry {
    /// Registered services by capability
    services_by_capability: HashMap<String, Vec<UniversalServiceRegistration>>,

    /// All registered services by service ID
    all_services: HashMap<uuid::Uuid, UniversalServiceRegistration>,
}

impl CapabilityRegistry {
    /// Create new capability registry
    pub fn new() -> Self {
        Self {
            services_by_capability: HashMap::new(),
            all_services: HashMap::new(),
        }
    }

    /// Register a service and its capabilities
    pub async fn register_service(
        &mut self,
        registration: UniversalServiceRegistration,
    ) -> EcosystemResult<()> {
        // Store in all services
        self.all_services
            .insert(registration.service_id, registration.clone());

        // Index by capabilities
        for capability in &registration.capabilities {
            self.services_by_capability
                .entry(capability.capability_id.clone())
                .or_default()
                .push(registration.clone());
        }

        Ok(())
    }

    /// Find services by capability
    pub async fn find_by_capability(
        &self,
        capability_id: &str,
    ) -> EcosystemResult<Vec<UniversalServiceRegistration>> {
        Ok(self
            .services_by_capability
            .get(capability_id)
            .cloned()
            .unwrap_or_else(Vec::new))
    }

    /// Get all registered services
    pub async fn get_all_services(&self) -> EcosystemResult<Vec<UniversalServiceRegistration>> {
        Ok(self.all_services.values().cloned().collect())
    }

    /// Remove a service registration
    pub async fn unregister_service(&mut self, service_id: uuid::Uuid) -> EcosystemResult<()> {
        if let Some(registration) = self.all_services.remove(&service_id) {
            // Remove from capability indexes
            for capability in &registration.capabilities {
                if let Some(services) = self
                    .services_by_capability
                    .get_mut(&capability.capability_id)
                {
                    services.retain(|s| s.service_id != service_id);
                    if services.is_empty() {
                        self.services_by_capability
                            .remove(&capability.capability_id);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}
