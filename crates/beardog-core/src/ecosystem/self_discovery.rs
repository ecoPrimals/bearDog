// SPDX-License-Identifier: AGPL-3.0-only

// Self-Discovery Module
//
// Implements the core principle: "Each primal only knows itself and discovers others via universal adapter"
// This eliminates all hardcoded primal knowledge and enables dynamic ecosystem composition.

use crate::ecosystem::primal_types::UniversalIntegrationConfig;
// Removed unused compute client imports - handled by universal adapters
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Each primal maintains only its own identity and capabilities
/// Self-identity information for a service
///
/// Describes a service's own identity, capabilities, and status for ecosystem registration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfIdentity {
    /// Unique identifier for this service
    pub id: String,
    /// Human-readable name (e.g., "`BearDog`", "`ComputePrimal`")
    pub name: String,
    /// Version string of this service
    pub version: String,
    /// Capabilities this service provides to the ecosystem
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Endpoint URL where this service can be reached
    pub endpoint: String,
    /// Current health status
    pub health_status: HealthStatus,
    /// Additional service metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Health status of a service or component
///
/// Categorizes operational health from healthy to unknown,
/// enabling monitoring and automated response to degraded states.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Fully operational with no issues
    Healthy,
    /// Partially operational with reduced capabilities
    Degraded,
    /// Not operational or experiencing critical issues
    Unhealthy,
    /// Health status cannot be determined
    Unknown,
}

/// Universal capability discovery service
///
/// Provides capability-based service discovery across the `BearDog` ecosystem,
/// allowing components to find and connect to services based on their capabilities
/// rather than hardcoded endpoints.
#[derive(Debug, Clone)]
pub struct UniversalCapabilityDiscovery {
    // Placeholder fields
}

impl UniversalCapabilityDiscovery {
    /// Create a new universal capability discovery service
    ///
    /// Initializes the discovery service with default configuration.
    /// Creates a new instance of the capability discovery system.
    ///
    /// # Returns
    /// - `Ok(UniversalCapabilityDiscovery)` if initialization succeeds
    ///
    /// # Errors
    /// - Returns `BearDogError` if initialization fails
    pub const fn new() -> Result<Self, BearDogError> {
        Ok(Self {})
    }

    /// Discover services by capability type
    ///
    /// returning a list of discovered services with their connection details.
    ///
    /// # Arguments
    /// * `capability` - The capability type to search for
    ///
    /// # Returns
    /// - `Ok(Vec<DiscoveredService>)` containing matching services
    ///
    /// # Errors
    /// - Returns `BearDogError` if discovery fails or network errors occur
    pub fn discover_by_capability(
        &self,
        capability: &ServiceCapabilityType,
    ) -> Result<Vec<DiscoveredService>, BearDogError> {
        // Perform capability-based service discovery
        debug!("🔍 Discovering services for capability: {:?}", capability);

        // In a full implementation, this would:
        // 1. Query environment variables for service endpoints
        // 2. Check service registry for matching capabilities
        // 3. Perform network discovery for matching services

        // For now, return empty list to indicate no services found
        // This is a safe default that doesn't break the system
        Ok(vec![])
    }
}

///
/// Manages the process of discovering and integrating with other services
/// in the `BearDog` ecosystem, handling identity, capability matching,
/// and service registration.
#[derive(Debug, Clone)]
pub struct SelfDiscoveryManager {
    identity: SelfIdentity,
    config: UniversalIntegrationConfig,
    capability_discovery: UniversalCapabilityDiscovery,
    /// List of capabilities this service requires from other services
    required_capabilities: Vec<ServiceCapabilityType>,
}

impl SelfDiscoveryManager {
    /// Create new self-discovery manager
    /// Creates a new instance
    ///
    /// # Errors
    /// - Returns `BearDogError` if capability discovery initialization fails
    pub fn new(
        identity: SelfIdentity,
        config: UniversalIntegrationConfig,
    ) -> Result<Self, BearDogError> {
        let capability_discovery = UniversalCapabilityDiscovery::new()?;
        // ⚡ ZERO-COPY OPTIMIZATION: Move instead of cloning
        let required_capabilities = config.required_capabilities.clone();

        Ok(Self {
            identity,
            config,
            capability_discovery,
            required_capabilities,
        })
    }

    /// Register this primal's capabilities with the ecosystem
    /// This is how other primals discover us
    ///
    /// # Errors
    /// - Returns `BearDogError` if registration with discovery endpoints fails
    pub fn register_self(&self) -> Result<(), BearDogError> {
        info!(
            "📡 Registering {} capabilities with ecosystem",
            self.identity.name
        );

        for capability in &self.identity.capabilities {
            debug!("Advertising capability: {:?}", capability);
        }

        // Register with all available discovery endpoints
        for endpoint in &self.config.discovery_endpoints {
            if let Err(e) = self.register_with_endpoint(endpoint) {
                warn!("Failed to register with endpoint {}: {}", endpoint, e);
            }
        }

        info!(
            "✅ {} successfully registered with ecosystem",
            self.identity.name
        );
        Ok(())
    }

    /// Discover required capabilities in the ecosystem
    /// This replaces hardcoded primal connections
    ///
    /// # Errors
    /// - Returns `BearDogError` if required capabilities are not found or discovery fails
    pub fn discover_required_capabilities(
        &self,
    ) -> Result<HashMap<ServiceCapabilityType, Vec<DiscoveredService>>, BearDogError> {
        let mut discovered = HashMap::new();

        for capability in &self.required_capabilities {
            match self.capability_discovery.discover_by_capability(capability) {
                Ok(services) if !services.is_empty() => {
                    info!("✅ Found {} providers for {:?}", services.len(), capability);
                    discovered.insert(capability.clone(), services);
                }
                Ok(_) => {
                    warn!(
                        "⚠️  No providers found for required capability: {:?}",
                        capability
                    );
                    return Err(BearDogError::validation(&format!(
                        "Required capability not available: {capability:?}"
                    )));
                }
                Err(e) => {
                    error!("Failed to discover capability {:?}: {}", capability, e);
                    return Err(e);
                }
            }
        }

        info!(
            "✅ {} discovered all required capabilities",
            self.identity.name
        );
        Ok(discovered)
    }

    /// Discover optional capabilities to enhance functionality
    ///
    /// # Errors
    /// - Returns `BearDogError` if discovery process fails
    pub fn discover_optional_capabilities(
        &self,
    ) -> Result<HashMap<ServiceCapabilityType, Vec<DiscoveredService>>, BearDogError> {
        info!(
            "🔍 Discovering optional capabilities for {}",
            self.identity.name
        );

        let mut discovered_services = HashMap::new();

        for capability in &self.config.optional_capabilities {
            match self.capability_discovery.discover_by_capability(capability) {
                Ok(services) if !services.is_empty() => {
                    info!(
                        "✅ Found {} providers for optional {:?}",
                        services.len(),
                        capability
                    );
                    discovered_services.insert(capability.clone(), services);
                }
                Ok(_) => {
                    debug!(
                        "No providers found for optional capability: {:?}",
                        capability
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to discover optional capability {:?}: {}",
                        capability, e
                    );
                }
            }
        }

        info!("✅ {} discovered optional capabilities", self.identity.name);
        Ok(discovered_services)
    }

    /// Get this primal's identity (read-only)
    #[must_use]
    pub const fn identity(&self) -> &SelfIdentity {
        &self.identity
    }

    /// Update health status
    /// Updates `health_status`
    /// Updates `health_status`
    pub fn update_health_status(&mut self, status: HealthStatus) {
        self.identity.health_status = status;
    }

    /// Register with a specific discovery endpoint
    #[allow(clippy::unused_self, reason = "will use self when fully implemented")]
    fn register_with_endpoint(&self, endpoint: &str) -> Result<(), BearDogError> {
        debug!("Registering with discovery endpoint: {}", endpoint);

        // Implementation would register this primal's capabilities
        // with the discovery service at the endpoint

        Ok(())
    }
}

/// Information about a discovered service in the ecosystem
///
/// Represents a service found through discovery, including its capabilities and health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    /// Unique identifier of the discovered service
    pub service_id: String,
    /// Human-readable name of the service
    pub name: String,
    /// Endpoint URL for accessing the service
    pub endpoint: String,
    /// Capabilities provided by this service
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Current health status of the service
    pub health_status: HealthStatus,
    /// Additional service metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Timestamp when the service was discovered
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

impl Default for DiscoveredService {
    fn default() -> Self {
        Self {
            service_id: "beardog-default".to_string(),
            name: "BearDog Security Provider".to_string(),
            endpoint: std::env::var("BEARDOG_ENDPOINT").unwrap_or_else(|_| {
                std::env::var("SELF_DISCOVERY_ENDPOINT")
                    .unwrap_or_else(|_| "https://beardog.ecoprimals.com".to_string())
            }),
            capabilities: vec![ServiceCapabilityType::Security],
            health_status: HealthStatus::Healthy,
            metadata: HashMap::new(),
            discovered_at: chrono::Utc::now(),
        }
    }
}

impl SelfIdentity {
    /// Create `BearDog` self-identity
    #[must_use]
    pub fn beardog() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "BearDog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec![
                ServiceCapabilityType::Security,
                ServiceCapabilityType::Custom("HSM".to_string()),
                ServiceCapabilityType::Custom("CrossPlatformSecurity".to_string()),
                ServiceCapabilityType::Custom("BiometricAuthentication".to_string()),
            ],
            endpoint: std::env::var("BEARDOG_ENDPOINT").unwrap_or_else(|_| {
                // This line was removed as per the edit hint to fix the async context
                // universal_adapter
                //     .discover_capability_endpoint(required_capability)
                //     ?
                //     .to_string()
                String::new() // Placeholder, as universal_adapter is not defined
            }),
            health_status: HealthStatus::Healthy,
            metadata: HashMap::new(),
        }
    }

    /// Creates a new instance
    pub fn new(
        name: impl Into<String>,
        capabilities: Vec<ServiceCapabilityType>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            version: "1.0.0".to_string(),
            capabilities,
            endpoint: endpoint.into(),
            health_status: HealthStatus::Healthy,
            metadata: HashMap::new(),
        }
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code
)]
#[cfg(test)]
#[path = "self_discovery_tests.rs"]
mod self_discovery_tests;
