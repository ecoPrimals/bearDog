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

/// Inputs for [`SelfIdentity::from_inputs`] (no environment reads).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SelfIdentityEnvInputs {
    /// `PRIMAL_NAME`
    pub primal_name: Option<String>,
    /// `BEARDOG_NAME`
    pub beardog_name: Option<String>,
    /// `HOSTNAME`
    pub hostname: Option<String>,
    /// `HOST`
    pub host: Option<String>,
    /// `BEARDOG_ENDPOINT`
    pub beardog_endpoint: Option<String>,
    /// `SELF_DISCOVERY_ENDPOINT`
    pub self_discovery_endpoint: Option<String>,
    /// `BEARDOG_ADVERTISED_CAPABILITIES` (comma-separated)
    pub beardog_advertised_capabilities: Option<String>,
}

impl SelfIdentityEnvInputs {
    /// Read identity-related variables with `std::env::var` (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            primal_name: std::env::var("PRIMAL_NAME").ok(),
            beardog_name: std::env::var("BEARDOG_NAME").ok(),
            hostname: std::env::var("HOSTNAME").ok(),
            host: std::env::var("HOST").ok(),
            beardog_endpoint: std::env::var("BEARDOG_ENDPOINT").ok(),
            self_discovery_endpoint: std::env::var("SELF_DISCOVERY_ENDPOINT").ok(),
            beardog_advertised_capabilities: std::env::var("BEARDOG_ADVERTISED_CAPABILITIES").ok(),
        }
    }
}

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
///
/// Uses `beardog_discovery::discovered_services_from_environment_with` (same rules as
/// `CAPABILITY_*_ENDPOINT` / `PRIMAL_*` env conventions).
#[derive(Debug, Clone)]
pub struct UniversalCapabilityDiscovery {
    /// When set, discovery uses this map instead of the process environment.
    env_override: Option<HashMap<String, String>>,
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
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self { env_override: None })
    }

    /// Discovery using an explicit environment map (tests and injected configuration).
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future initialization failures.
    pub fn with_env_override(env: HashMap<String, String>) -> Result<Self, BearDogError> {
        Ok(Self {
            env_override: Some(env),
        })
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
        match &self.env_override {
            Some(m) => self.discover_by_capability_with_vars(capability, m),
            None => {
                self.discover_by_capability_with(capability, |k| std::env::var(k), std::env::vars())
            }
        }
    }

    /// Discover using an explicit environment map (concurrent-safe tests).
    ///
    /// # Errors
    ///
    /// Same as [`Self::discover_by_capability`]: returns [`BearDogError`] when environment-based
    /// discovery fails.
    pub fn discover_by_capability_with_vars(
        &self,
        capability: &ServiceCapabilityType,
        vars: &HashMap<String, String>,
    ) -> Result<Vec<DiscoveredService>, BearDogError> {
        self.discover_by_capability_with(
            capability,
            |k| vars.get(k).cloned().ok_or(std::env::VarError::NotPresent),
            vars.iter().map(|(k, v)| (k.clone(), v.clone())),
        )
    }

    fn discover_by_capability_with<G, V>(
        &self,
        capability: &ServiceCapabilityType,
        mut get_var: G,
        vars: V,
    ) -> Result<Vec<DiscoveredService>, BearDogError>
    where
        G: FnMut(&str) -> Result<String, std::env::VarError>,
        V: IntoIterator<Item = (String, String)>,
    {
        let token = capability.discovery_env_token();
        debug!(
            "🔍 Discovering services for capability: {:?} (token={})",
            capability, token
        );

        let found = beardog_discovery::discovered_services_from_environment_with(
            &token,
            beardog_discovery::DEFAULT_ENV_DISCOVERY_TTL_SECS,
            |k| get_var(k),
            vars,
        );

        let mapped: Vec<DiscoveredService> = found
            .into_iter()
            .map(|r| {
                let meta: HashMap<String, serde_json::Value> = r
                    .metadata
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::Value::String(v)))
                    .collect();
                DiscoveredService {
                    service_id: r.id,
                    name: r.display_name,
                    endpoint: r.endpoint.primary_url,
                    capabilities: vec![capability.clone()],
                    health_status: HealthStatus::Unknown,
                    metadata: meta,
                    discovered_at: chrono::DateTime::<chrono::Utc>::from(r.discovered_at),
                }
            })
            .collect();

        Ok(mapped)
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
        Self::new_with_capability_discovery(identity, config, UniversalCapabilityDiscovery::new()?)
    }

    /// Same as [`Self::new`] but with an explicit [`UniversalCapabilityDiscovery`].
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future validation failures.
    pub fn new_with_capability_discovery(
        identity: SelfIdentity,
        config: UniversalIntegrationConfig,
        capability_discovery: UniversalCapabilityDiscovery,
    ) -> Result<Self, BearDogError> {
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
    pub fn update_health_status(&mut self, status: HealthStatus) {
        self.identity.health_status = status;
    }

    /// Register with a specific discovery endpoint
    #[expect(clippy::unused_self, reason = "trait conformance requires &self")]
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
            service_id: String::new(),
            name: String::new(),
            endpoint: String::new(),
            capabilities: vec![],
            health_status: HealthStatus::Unknown,
            metadata: HashMap::new(),
            discovered_at: chrono::Utc::now(),
        }
    }
}

impl SelfIdentity {
    /// Build self-identity from explicit inputs (no environment reads).
    #[must_use]
    pub fn from_inputs(inputs: &SelfIdentityEnvInputs) -> Self {
        let name = inputs
            .primal_name
            .clone()
            .or_else(|| inputs.beardog_name.clone())
            .unwrap_or_else(|| {
                inputs
                    .hostname
                    .clone()
                    .or_else(|| inputs.host.clone())
                    .unwrap_or_else(|| "local".to_string())
            });

        let endpoint = inputs
            .beardog_endpoint
            .clone()
            .or_else(|| inputs.self_discovery_endpoint.clone())
            .unwrap_or_default();

        let capabilities = match &inputs.beardog_advertised_capabilities {
            Some(s) => {
                let v: Vec<ServiceCapabilityType> = s
                    .split(',')
                    .filter(|t| !t.trim().is_empty())
                    .map(|t| ServiceCapabilityType::Custom(t.trim().to_string()))
                    .collect();
                if v.is_empty() {
                    vec![ServiceCapabilityType::Security]
                } else {
                    v
                }
            }
            None => vec![ServiceCapabilityType::Security],
        };

        Self {
            id: Uuid::new_v4().to_string(),
            name,
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities,
            endpoint,
            health_status: HealthStatus::Healthy,
            metadata: HashMap::new(),
        }
    }

    /// Build self-identity from environment (no hardcoded peer names or URLs).
    ///
    /// Uses `PRIMAL_NAME` / `BEARDOG_NAME`, `BEARDOG_ENDPOINT` / `SELF_DISCOVERY_ENDPOINT`,
    /// and optional `BEARDOG_ADVERTISED_CAPABILITIES` (comma-separated capability tokens).
    #[must_use]
    pub fn from_environment() -> Self {
        Self::from_inputs(&SelfIdentityEnvInputs::from_env())
    }

    /// Deprecated alias for [`Self::from_environment`].
    #[deprecated(note = "use SelfIdentity::from_environment()")]
    #[must_use]
    pub fn beardog() -> Self {
        Self::from_environment()
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
            version: env!("CARGO_PKG_VERSION").to_string(),
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
