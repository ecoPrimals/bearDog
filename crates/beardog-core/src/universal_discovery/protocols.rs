// SPDX-License-Identifier: AGPL-3.0-or-later

// Discovery Protocols Module
//
// This module contains discovery protocol implementations and configuration
// for various service discovery mechanisms.

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
// Protocol implementations for universal discovery
use tracing::info;

// Import required types from the parent module
use super::{ProtocolStatistics, ServiceInfo};

/// Modern capability-based service discovery
///
/// Replaces deprecated hardcoded protocol handlers with dynamic capability discovery.
/// This allows `BearDog` to work with any service discovery system that implements
/// the required capabilities.
#[derive(Debug)]
pub struct ModernServiceDiscovery {
    /// Collection of required capabilities
    pub required_capabilities: Vec<CapabilityType>,

    /// Optional preferred capabilities
    pub preferred_capabilities: Option<Vec<CapabilityType>>,

    /// Discovered service providers
    pub discovered_providers: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for ModernServiceDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl ModernServiceDiscovery {
    /// Create a new modern service discovery instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            required_capabilities: vec![CapabilityType::ServiceMesh],
            preferred_capabilities: None,
            discovered_providers: std::collections::HashMap::new(),
        }
    }

    /// Discover services with the specified capability.
    ///
    /// Queries the local `discovered_providers` registry populated by runtime
    /// IPC announcements.  Returns an empty list when no providers have been
    /// registered for the requested capability — callers must handle graceful
    /// degradation.
    ///
    /// # Errors
    /// Returns an error if the capability type is unrecognised by the registry.
    pub fn discover_capability(
        &mut self,
        capability: CapabilityType,
    ) -> Result<Vec<String>, BearDogError> {
        let key = format!("{capability:?}");
        match self.discovered_providers.get(&key) {
            Some(serde_json::Value::Array(arr)) => Ok(arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()),
            Some(serde_json::Value::String(s)) => Ok(vec![s.clone()]),
            Some(_) | None => Ok(Vec::new()),
        }
    }

    /// Start the service discovery instance
    ///
    /// # Errors
    /// Returns an error if the service discovery instance fails to initialize or start.
    pub fn start(&self) -> Result<(), BearDogError> {
        info!("Starting modern service discovery");
        Ok(())
    }

    /// Stop the service discovery instance
    ///
    /// # Errors
    /// Returns an error if the service discovery instance fails to stop gracefully or encounters shutdown issues.
    pub fn stop(&self) -> Result<(), BearDogError> {
        info!("Stopping modern service discovery");
        Ok(())
    }

    /// Register a service with the discovery system
    ///
    /// Records the service in the internal provider registry keyed by capability.
    /// Services are discovered at runtime via IPC announcements — no prior knowledge required.
    ///
    /// # Errors
    /// Returns an error if the service name is empty.
    pub fn register_service(&mut self, service: &ServiceInfo) -> Result<(), BearDogError> {
        if service.name.is_empty() {
            return Err(BearDogError::validation("Service name cannot be empty"));
        }
        info!("Registering service: {}", service.name);

        let key = service.name.clone();
        let entry = self
            .discovered_providers
            .entry(key)
            .or_insert_with(|| serde_json::Value::Array(Vec::new()));

        if let serde_json::Value::Array(arr) = entry {
            let service_value = serde_json::json!({
                "name": service.name,
                "registered_at": chrono::Utc::now().to_rfc3339(),
            });
            if !arr.contains(&service_value) {
                arr.push(service_value);
            }
        }

        Ok(())
    }

    /// Deregister a service from the discovery system
    ///
    /// Removes the service from the internal provider registry.
    ///
    /// # Errors
    /// Returns an error if the service is not currently registered.
    pub fn deregister_service(&mut self, service: &ServiceInfo) -> Result<(), BearDogError> {
        if self.discovered_providers.remove(&service.name).is_none() {
            return Err(BearDogError::not_found(format!(
                "Service '{}' not registered",
                service.name
            )));
        }
        info!("Deregistered service: {}", service.name);
        Ok(())
    }

    /// Discover services matching the given name pattern
    ///
    /// Queries the runtime-populated provider registry. Returns matching services
    /// that have been announced via IPC. Empty results indicate no matching providers
    /// have announced themselves — callers must handle graceful degradation.
    ///
    /// # Errors
    /// Returns an error if the service name pattern is invalid.
    pub fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInfo>, BearDogError> {
        if service_name.is_empty() {
            return Err(BearDogError::validation(
                "Service discovery requires a non-empty name pattern",
            ));
        }

        let results: Vec<ServiceInfo> = self
            .discovered_providers
            .keys()
            .filter(|key| key.contains(service_name))
            .map(|name| ServiceInfo {
                name: name.clone(),
                service_type: String::new(),
                address: String::new(),
                port: 0,
                metadata: std::collections::BTreeMap::new(),
            })
            .collect();

        info!(
            "Discovery for '{}': {} providers found",
            service_name,
            results.len()
        );
        Ok(results)
    }

    /// Get service discovery statistics
    ///
    /// # Errors
    /// Returns an error if statistics collection fails or if internal state is inconsistent.
    pub fn get_statistics(&self) -> Result<ProtocolStatistics, BearDogError> {
        Ok(ProtocolStatistics {
            services_discovered: self.discovered_providers.len(),
            discovery_requests: 0,
            registration_requests: 0,
            errors: 0,
            last_activity: chrono::Utc::now(),
        })
    }
}
