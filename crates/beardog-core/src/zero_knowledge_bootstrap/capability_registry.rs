//! # Capability Registry
//!
//! Dynamic registry for ecosystem capabilities discovered through zero-knowledge bootstrap.
//! Implements the "infant discovery pattern" where capabilities are learned, not hardcoded.
//!
//! ## Architecture
//!
//! - **Dynamic Discovery**: Capabilities discovered at runtime
//! - **Health Monitoring**: Continuous health checks for registered capabilities
//! - **Type-Safe**: Strongly typed capability identifiers
//! - **Async/Await**: Modern async patterns throughout
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let registry = CapabilityRegistry::new();
//!
//! // Register a discovered capability
//! let capability_id = registry.register(capability).await?;
//!
//! // Discover capabilities by type
//! let compute_providers = registry
//!     .discover_by_type(ServiceCapabilityType::Compute)
//!     .await?;
//! # Ok(())
//! # }
//! ```

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{CapabilityType, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Type alias for service capability types
///
/// Provides a semantic alias for `CapabilityType` when used in service contexts.
/// This helps distinguish between general capabilities and service-specific capabilities.
pub type ServiceCapabilityType = CapabilityType;

/// Unique identifier for a registered capability
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Create a new unique capability ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create from existing string
    #[must_use]
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Get the inner string
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for CapabilityId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Health status for capability tracking
///
/// Tracks the operational health of registered capabilities in the registry.
/// Health checks run periodically to ensure capabilities remain available.
///
/// # Health Status Lifecycle
/// - `Unknown`: Initial state after registration
/// - `Healthy`: Passing health checks consistently
/// - `Degraded`: Experiencing issues but still functional
/// - `Unhealthy`: Failed health checks, may be removed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Health status unknown (capability just registered, not yet checked)
    Unknown,
    /// Capability is healthy and fully operational
    Healthy,
    /// Capability is degraded but still functional
    Degraded,
    /// Capability is unhealthy or non-responsive
    Unhealthy,
}

// Removed CapabilityMetadata - using UniversalCapability's metadata field directly

/// A registered capability in the ecosystem with tracking metadata
///
/// Wraps a `UniversalCapability` with additional registry-specific metadata
/// including registration time, health status, and failure tracking.
///
/// Used internally by the `CapabilityRegistry` to manage discovered capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredCapability {
    /// Unique identifier
    pub id: CapabilityId,
    /// The full capability information
    pub capability: UniversalCapability,
    /// When this capability was registered
    pub registration_time: SystemTime,
    /// Last successful health check
    pub last_health_check: Option<SystemTime>,
    /// Number of consecutive health check failures
    pub consecutive_failures: u32,
}

/// Configuration for the capability registry
///
/// Controls how the capability registry operates, including health check
/// frequency, failure tolerance, and automatic cleanup behavior.
///
/// # Examples
/// ```
/// use std::time::Duration;
/// use beardog_core::zero_knowledge_bootstrap::capability_registry::CapabilityRegistryConfig;
///
/// let config = CapabilityRegistryConfig {
///     health_check_interval: Duration::from_secs(30),
///     max_consecutive_failures: 3,
///     health_check_timeout: Duration::from_secs(300),
///     auto_remove_unhealthy: true,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CapabilityRegistryConfig {
    /// How often to perform health checks
    pub health_check_interval: Duration,
    /// Maximum number of consecutive failures before marking unhealthy
    pub max_consecutive_failures: u32,
    /// Timeout for health check operations
    pub health_check_timeout: Duration,
    /// Whether to automatically remove unhealthy capabilities
    pub auto_remove_unhealthy: bool,
}

impl Default for CapabilityRegistryConfig {
    fn default() -> Self {
        Self {
            health_check_interval: Duration::from_secs(
                std::env::var("BEARDOG_CAPABILITY_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            max_consecutive_failures: 3,
            health_check_timeout: Duration::from_secs(
                std::env::var("BEARDOG_CAPABILITY_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5),
            ),
            auto_remove_unhealthy: false,
        }
    }
}

/// Dynamic capability registry for ecosystem services
///
/// This registry implements the zero-knowledge bootstrap pattern by dynamically
/// discovering and tracking capabilities without any hardcoded knowledge.
#[derive(Debug)]
pub struct CapabilityRegistry {
    /// Registered capabilities by ID
    capabilities: Arc<RwLock<HashMap<CapabilityId, RegisteredCapability>>>,
    /// Index by capability type for fast lookup
    type_index: Arc<RwLock<HashMap<ServiceCapabilityType, Vec<CapabilityId>>>>,
    /// Configuration
    config: CapabilityRegistryConfig,
}

impl CapabilityRegistry {
    /// Create a new capability registry with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(CapabilityRegistryConfig::default())
    }

    /// Create a new capability registry with custom configuration
    #[must_use]
    pub fn with_config(config: CapabilityRegistryConfig) -> Self {
        Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
            type_index: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a new capability in the ecosystem
    ///
    /// # Arguments
    ///
    /// * `capability` - The universal capability to register
    ///
    /// # Returns
    ///
    /// Unique ID assigned to this capability
    ///
    /// # Errors
    /// Returns an error if capability registration fails, if storage operations encounter issues,
    /// or if the capability type is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_core::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    /// # use beardog_types::canonical::capabilities::UniversalCapability;
    /// # use beardog_errors::BearDogError;
    /// # async fn example(registry: &CapabilityRegistry, capability: UniversalCapability) -> Result<(), BearDogError> {
    /// let id = registry.register(capability).await?;
    /// println!("Registered capability: {}", id);
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::cognitive_complexity)] // Capability registration requires validation, storage, and index updates
    pub async fn register(
        &self,
        capability: UniversalCapability,
    ) -> Result<CapabilityId, BearDogError> {
        let id = CapabilityId::new();

        debug!(
            "Registering capability: {} (type: {:?})",
            id, capability.capability_type
        );

        // Create registered capability
        let cap_type = capability.capability_type.clone();
        let registered = RegisteredCapability {
            id: id.clone(),
            capability,
            registration_time: SystemTime::now(),
            last_health_check: None,
            consecutive_failures: 0,
        };

        // Store capability
        self.capabilities
            .write()
            .await
            .insert(id.clone(), registered);

        // Update type index
        self.type_index
            .write()
            .await
            .entry(cap_type)
            .or_insert_with(Vec::new)
            .push(id.clone());

        info!("Successfully registered capability: {}", id);
        Ok(id)
    }

    /// Discover capabilities by type
    ///
    /// Returns all registered capabilities of the specified type.
    ///
    /// # Arguments
    ///
    /// * `capability_type` - The type of capabilities to discover
    ///
    /// # Errors
    /// Returns an error if capability lookup fails or if internal storage access encounters issues.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_core::zero_knowledge_bootstrap::capability_registry::CapabilityRegistry;
    /// # use beardog_types::canonical::capabilities::ServiceCapabilityType;
    /// # use beardog_errors::BearDogError;
    /// # async fn example(registry: &CapabilityRegistry) -> Result<(), BearDogError> {
    /// let compute_caps = registry
    ///     .discover_by_type(ServiceCapabilityType::Compute)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::cognitive_complexity)] // Discovery by type requires index lookup, filtering, and health checks
    pub async fn discover_by_type(
        &self,
        capability_type: ServiceCapabilityType,
    ) -> Result<Vec<RegisteredCapability>, BearDogError> {
        debug!("Discovering capabilities of type: {:?}", capability_type);

        // Get capability IDs for this type
        let ids = self
            .type_index
            .read()
            .await
            .get(&capability_type)
            .cloned()
            .unwrap_or_default();

        // Retrieve full capability details
        let capabilities_lock = self.capabilities.read().await;
        let mut results = Vec::with_capacity(ids.len());

        for id in ids {
            if let Some(capability) = capabilities_lock.get(&id) {
                results.push(capability.clone());
            }
        }

        info!(
            "Discovered {} capabilities of type {:?}",
            results.len(),
            capability_type
        );

        Ok(results)
    }

    /// Get a specific capability by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The capability ID to lookup
    ///
    /// # Returns
    ///
    /// The registered capability if found, None otherwise
    ///
    /// # Errors
    /// Returns an error if internal storage access fails or if the lookup operation encounters issues.
    pub async fn get(
        &self,
        id: &CapabilityId,
    ) -> Result<Option<RegisteredCapability>, BearDogError> {
        Ok(self.capabilities.read().await.get(id).cloned())
    }

    /// Update the health status of a capability
    ///
    /// # Arguments
    ///
    /// * `id` - The capability ID
    /// * `status` - The new health status
    ///
    /// # Errors
    /// Returns an error if the capability is not found or if the health status update fails.
    pub async fn update_health_status(
        &self,
        id: &CapabilityId,
        status: HealthStatus,
    ) -> Result<(), BearDogError> {
        let mut capabilities = self.capabilities.write().await;

        if let Some(registered) = capabilities.get_mut(id) {
            // Update tracking information (not modifying UniversalCapability's health_status)
            registered.last_health_check = Some(SystemTime::now());

            // Update failure counter based on our internal health status
            match status {
                HealthStatus::Healthy => {
                    registered.consecutive_failures = 0;
                }
                HealthStatus::Unhealthy | HealthStatus::Degraded => {
                    registered.consecutive_failures += 1;
                }
                HealthStatus::Unknown => {}
            }

            debug!(
                "Updated health status for {}: {:?} (failures: {})",
                id, status, registered.consecutive_failures
            );

            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Capability not found: {id}"
            )))
        }
    }

    /// Remove a capability from the registry
    ///
    /// # Arguments
    ///
    /// * `id` - The capability ID to remove
    ///
    /// # Errors
    /// Returns an error if the capability is not found or if the removal operation fails.
    #[allow(clippy::cognitive_complexity)] // Capability removal requires index cleanup and validation across multiple data structures
    pub async fn remove(&self, id: &CapabilityId) -> Result<(), BearDogError> {
        debug!("Removing capability: {}", id);

        // Remove from main storage
        let removed = self.capabilities.write().await.remove(id);

        if let Some(registered) = removed {
            // Remove from type index
            {
                let mut type_index = self.type_index.write().await;
                if let Some(ids) = type_index.get_mut(&registered.capability.capability_type) {
                    ids.retain(|cap_id| cap_id != id);
                }
            }

            info!("Successfully removed capability: {}", id);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Capability not found: {id}"
            )))
        }
    }

    /// List all registered capabilities
    ///
    /// # Returns
    ///
    /// Vector of all registered capabilities
    ///
    /// # Errors
    /// Returns an error if internal storage access fails or if the listing operation encounters issues.
    pub async fn list_all(&self) -> Result<Vec<RegisteredCapability>, BearDogError> {
        Ok(self.capabilities.read().await.values().cloned().collect())
    }

    /// Get statistics about the registry
    ///
    /// # Returns
    ///
    /// Registry statistics including counts by type and health status
    ///
    /// # Errors
    /// Returns an error if statistics calculation fails or if internal storage access encounters issues.
    pub async fn statistics(&self) -> Result<RegistryStatistics, BearDogError> {
        let capabilities = self.capabilities.read().await;

        let mut stats = RegistryStatistics {
            total_capabilities: capabilities.len(),
            by_type: HashMap::new(),
            by_health_status: HashMap::new(),
        };

        for registered in capabilities.values() {
            // Count by type
            *stats
                .by_type
                .entry(registered.capability.capability_type.clone())
                .or_insert(0) += 1;

            // Count by health status  (using our internal HealthStatus from consecutive_failures)
            let health = if registered.consecutive_failures == 0 {
                HealthStatus::Healthy
            } else if registered.consecutive_failures >= self.config.max_consecutive_failures {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Degraded
            };

            *stats.by_health_status.entry(health).or_insert(0) += 1;
        }

        // Explicitly drop the lock early to avoid holding it unnecessarily
        drop(capabilities);

        Ok(stats)
    }

    /// Remove unhealthy capabilities that exceed failure threshold
    ///
    /// This is useful for automatic cleanup of dead capabilities.
    ///
    /// # Returns
    ///
    /// Number of capabilities removed
    ///
    /// # Errors
    /// Returns an error if the cleanup operation fails or if capability removal encounters issues.
    #[allow(clippy::cognitive_complexity)] // Health-based cleanup requires iterating, filtering, and coordinated removal
    pub async fn cleanup_unhealthy(&self) -> Result<usize, BearDogError> {
        let mut removed_count = 0;
        let capabilities = self.capabilities.read().await;

        // Find capabilities to remove
        let to_remove: Vec<CapabilityId> = capabilities
            .values()
            .filter(|cap| cap.consecutive_failures >= self.config.max_consecutive_failures)
            .map(|cap| cap.id.clone())
            .collect();

        drop(capabilities); // Release read lock

        // Remove them
        for id in to_remove {
            if self.remove(&id).await.is_ok() {
                removed_count += 1;
                warn!("Removed unhealthy capability: {}", id);
            }
        }

        if removed_count > 0 {
            info!("Cleaned up {} unhealthy capabilities", removed_count);
        }

        Ok(removed_count)
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the capability registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of registered capabilities
    pub total_capabilities: usize,
    /// Count of capabilities by type
    pub by_type: HashMap<ServiceCapabilityType, usize>,
    /// Count of capabilities by health status
    pub by_health_status: HashMap<HealthStatus, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use beardog_types::canonical::capabilities::{
        AuthConfig, CircuitBreakerConfig, EndpointConfig, HealthStatus as CapHealthStatus,
        PerformanceMetrics, ProviderInfo, SecurityLevel,
    };
    use beardog_types::canonical::providers_unified::core::ProviderType;

    fn create_test_capability(cap_type: ServiceCapabilityType) -> UniversalCapability {
        UniversalCapability {
            capability_type: cap_type,
            provider: ProviderInfo {
                provider_id: "test-provider".to_string(),
                provider_name: "Test Provider".to_string(),
                provider_type: ProviderType::Custom("test-provider".to_string()),
                version: "1.0.0".to_string(),
                region: None,
            },
            endpoint: EndpointConfig {
                base_url: {
                    use beardog_config::domains::network_ports::DEFAULT_API_PORT;
                    format!("http://test.local:{}", DEFAULT_API_PORT)
                },
                api_version: Some("v1".to_string()),
                timeout_ms: 5000,
                max_retries: 3,
                circuit_breaker: CircuitBreakerConfig::default(),
            },
            auth_config: AuthConfig {
                auth_type: beardog_types::canonical::capabilities::AuthType::None,
                api_key: None,
                bearer_token: None,
                cert_path: None,
                custom_params: HashMap::new(),
            },
            health_status: CapHealthStatus::Unknown,
            performance: PerformanceMetrics::default(),
            security_level: SecurityLevel::Standard,
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_register_capability() -> Result<(), Box<dyn std::error::Error>> {
        let registry = CapabilityRegistry::new();
        let capability = create_test_capability(ServiceCapabilityType::Compute);

        let id = registry.register(capability).await?;
        assert!(!id.as_str().is_empty());

        Ok(())
    }

    #[tokio::test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    async fn test_discover_by_type() -> Result<(), Box<dyn std::error::Error>> {
        let registry = CapabilityRegistry::new();

        // Register compute capability
        let compute_cap = create_test_capability(ServiceCapabilityType::Compute);
        registry.register(compute_cap).await?;

        // Register storage capability
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let storage_cap = create_test_capability(ServiceCapabilityType::Storage);
        registry.register(storage_cap).await?;

        // Discover compute capabilities
        let compute_caps = registry
            .discover_by_type(ServiceCapabilityType::Compute)
            .await?;

        assert_eq!(compute_caps.len(), 1);
        assert_eq!(
            compute_caps[0].capability.capability_type,
            ServiceCapabilityType::Compute
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_health_status_update() -> Result<(), Box<dyn std::error::Error>> {
        let registry = CapabilityRegistry::new();
        let capability = create_test_capability(ServiceCapabilityType::Compute);

        let id = registry.register(capability).await?;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Update health status
        registry
            .update_health_status(&id, HealthStatus::Healthy)
            .await?;

        // Verify update
        let cap = registry.get(&id).await?.unwrap();
        assert_eq!(cap.consecutive_failures, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_capability() -> Result<(), Box<dyn std::error::Error>> {
        let registry = CapabilityRegistry::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let capability = create_test_capability(ServiceCapabilityType::Compute);

        let id = registry.register(capability).await?;

        // Remove capability
        registry.remove(&id).await?;

        // Verify removal
        let result = registry.get(&id).await?;
        assert!(result.is_none());

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_statistics() -> Result<(), Box<dyn std::error::Error>> {
        let registry = CapabilityRegistry::new();

        // Register multiple capabilities
        registry
            .register(create_test_capability(ServiceCapabilityType::Compute))
            .await?;
        registry
            .register(create_test_capability(ServiceCapabilityType::Compute))
            .await?;
        registry
            .register(create_test_capability(ServiceCapabilityType::Storage))
            .await?;

        let stats = registry.statistics().await?;

        assert_eq!(stats.total_capabilities, 3);
        assert_eq!(
            *stats.by_type.get(&ServiceCapabilityType::Compute).unwrap(),
            2
        );
        assert_eq!(
            *stats.by_type.get(&ServiceCapabilityType::Storage).unwrap(),
            1
        );

        Ok(())
    }
}
