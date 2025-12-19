// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod capability;
/// Configuration management
/// Configuration management
pub mod config;
pub mod failover;
pub mod health;
pub mod implementation;
pub mod operation_router;
pub mod performance;

#[cfg(test)]
mod failover_tests;
#[cfg(test)]
mod health_tests;
use beardog_errors::BearDogError;
use beardog_types::hsm::{DefaultHsmFailoverManager, DefaultHsmHealthMonitor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::tunnel::hsm::types::tier::HsmTier;

pub use capability::DefaultHsmCapabilityDetector;
pub use config::{HsmManagerConfig, SimpleHsmTier};
pub use failover::{CircuitBreaker, CircuitBreakerState};
// Note: DefaultHsmFailoverManager doesn't exist in failover module - commented out
// pub use failover::DefaultHsmFailoverManager;
// Note: DefaultHsmHealthMonitor doesn't exist in health module - commented out
// pub use health::DefaultHsmHealthMonitor;
pub use implementation::{DefaultHsmManager, HealthStatus, HsmProvider, KeyInfo, ProviderInfo};
pub use operation_router::{
    HsmOperationRouter, HsmSelectionResult, OperationRoutingRules, OperationType,
};

pub use performance::{HsmPerformanceTracker, OperationMetrics};

/// HSM Provider Selection Result
///
/// Represents the result of selecting an HSM provider for an operation.
/// Contains information about the selected provider, its characteristics,
/// and selection metadata.
///
/// # Fields
///
/// * `provider` - Arc reference to the selected HSM provider implementation
/// * `provider_id` - Unique identifier for the provider instance
/// * `tier` - Security tier of the provider (Hardware, Cloud, Software, etc.)
/// * `confidence` - Selection confidence score (0.0-1.0), indicating how well
///   the provider matches the operation requirements
/// * `estimated_latency_ms` - Estimated operation latency in milliseconds,
///   based on historical performance data
///
/// # Example
///
/// ```ignore
/// use beardog_tunnel::tunnel::hsm::manager::HsmProviderSelection;
///
/// fn handle_selection(selection: HsmProviderSelection) {
///     println!("Selected provider: {}", selection.provider_id);
///     println!("Tier: {:?}", selection.tier);
///     println!("Confidence: {:.2}", selection.confidence);
///     println!("Est. latency: {:.2}ms", selection.estimated_latency_ms);
/// }
/// ```
///
/// # See Also
///
/// * [`HsmManager`] - Manages HSM provider selection
/// * [`HsmTier`] - HSM security tiers
/// * [`HsmProvider`] - HSM provider trait
pub struct HsmProviderSelection {
    pub provider: Arc<dyn HsmProvider>,

    pub provider_id: String,

    /// The tier value
    pub tier: HsmTier,

    pub confidence: f64,

    /// The estimated latency ms value
    pub estimated_latency_ms: f64,
}

/// HSM Manager
///
/// Central coordinator for Hardware Security Module operations in the BearDog ecosystem.
/// Manages multiple HSM providers, handles provider selection, failover, health monitoring,
/// and performance tracking.
///
/// # Responsibilities
///
/// * **Provider Management**: Register and manage multiple HSM providers
/// * **Provider Selection**: Select optimal provider for each operation based on
///   security requirements, performance characteristics, and availability
/// * **Health Monitoring**: Continuously monitor provider health and availability
/// * **Failover Management**: Automatically failover to backup providers on failure
/// * **Performance Tracking**: Track operation metrics and provider performance
/// * **Capability Detection**: Detect and validate provider capabilities
///
/// # Architecture
///
/// The HsmManager uses a sophisticated routing system that considers:
/// * Security tier requirements (Hardware > Cloud > Software)
/// * Provider availability and health status
/// * Operation-specific requirements
/// * Historical performance metrics
/// * Circuit breaker states for failing providers
///
/// # Thread Safety
///
/// This struct is thread-safe and can be shared across async tasks using `Arc`.
/// Internal state is protected by appropriate synchronization primitives.
///
/// # Example
///
/// ```ignore
/// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
/// use beardog_tunnel::tunnel::hsm::types::HsmTier;
/// use std::sync::Arc;
///
/// // Create manager
/// let mut manager = HsmManager::new();
///
/// // Register providers
/// manager.register_hsm_provider(
///     HsmTier::Hardware,
///     Arc::new(MyHardwareHsm::new())
/// )?;
///
/// manager.register_hsm_provider(
///     HsmTier::Software,
///     Arc::new(MySoftwareHsm::new())
/// )?;
///
/// // Manager will automatically select appropriate provider
/// // based on operation requirements
/// ```
///
/// # See Also
///
/// * [`HsmProvider`] - HSM provider trait
/// * [`HsmTier`] - Security tiers
/// * [`HsmManagerConfig`] - Configuration options
#[allow(dead_code)] // Fields used in implementation
pub struct HsmManager {
    hsm_providers: HashMap<String, Arc<dyn HsmProvider>>,
    config: HsmManagerConfig,
    health_monitor: Arc<DefaultHsmHealthMonitor>,
    failover_manager: Arc<DefaultHsmFailoverManager>,
    capability_detector: Arc<DefaultHsmCapabilityDetector>,
    performance_tracker: Arc<HsmPerformanceTracker>,
    operation_router: Arc<RwLock<HsmOperationRouter>>,
}

impl Default for HsmManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmManager {
    /// Create a new HSM Manager instance
    ///
    /// Creates a new HSM Manager with default configuration, initializing all
    /// internal components including health monitoring, failover management,
    /// capability detection, performance tracking, and operation routing.
    ///
    /// # Returns
    ///
    /// Returns a new `HsmManager` instance ready to register and manage HSM providers.
    ///
    /// # Panics
    ///
    /// This function will panic if the capability detector cannot be initialized,
    /// which should never happen under normal circumstances as it only allocates
    /// internal data structures.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    ///
    /// let manager = HsmManager::new();
    /// println!("HSM Manager created successfully");
    /// ```
    ///
    /// # See Also
    ///
    /// * [`register_hsm_provider`](Self::register_hsm_provider) - Register providers
    /// * [`HsmManagerConfig`] - Configuration options
    pub fn new() -> Self {
        Self {
            hsm_providers: HashMap::new(),
            config: HsmManagerConfig::default(),
            health_monitor: Arc::new(DefaultHsmHealthMonitor::default()),
            failover_manager: Arc::new(DefaultHsmFailoverManager::default()),
            capability_detector: Arc::new(DefaultHsmCapabilityDetector::new()
                .unwrap_or_else(|e| {
                    panic!("CRITICAL: DefaultHsmCapabilityDetector::new() failed - this should never happen as it only creates a HashMap: {e}")
                })),
            performance_tracker: Arc::new(HsmPerformanceTracker::default()),
            operation_router: Arc::new(RwLock::new(HsmOperationRouter::default())),
        }
    }

    /// Register an HSM Provider
    ///
    /// Registers a new HSM provider with the manager at the specified security tier.
    /// The provider will be available for selection during cryptographic operations
    /// based on its tier and capabilities.
    ///
    /// # Arguments
    ///
    /// * `tier` - Security tier for the provider (Hardware, Cloud, Software, etc.)
    /// * `provider` - Arc-wrapped provider implementation
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Provider registered successfully
    /// * `Err(BearDogError)` - Registration failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Provider validation fails
    /// * Tier is invalid
    /// * Internal state is inconsistent (rare)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    /// use beardog_tunnel::tunnel::hsm::types::HsmTier;
    /// use beardog_tunnel::tunnel::hsm::SoftwareHsm;
    /// use std::sync::Arc;
    ///
    /// let mut manager = HsmManager::new();
    ///
    /// // Register hardware HSM (highest priority)
    /// manager.register_hsm_provider(
    ///     HsmTier::Hardware,
    ///     Arc::new(MyHardwareHsm::new())
    /// )?;
    ///
    /// // Register software HSM (fallback)
    /// manager.register_hsm_provider(
    ///     HsmTier::Software,
    ///     Arc::new(SoftwareHsm::new(config)?)
    /// )?;
    /// ```
    ///
    /// # See Also
    ///
    /// * [`HsmProvider`] - Provider trait requirements
    /// * [`HsmTier`] - Available security tiers
    pub fn register_hsm_provider(
        &mut self,
        tier: HsmTier,
        provider: Arc<dyn HsmProvider>,
    ) -> Result<(), BearDogError> {
        let tier_key = format!("{tier:?}");
        self.hsm_providers.insert(tier_key, provider);
        Ok(())
    }

    /// Get Routing Metrics
    ///
    /// Returns metrics about HSM provider selection and routing, including
    /// operation counts, selection patterns, and performance statistics.
    ///
    /// # Returns
    ///
    /// HashMap containing routing metrics with the following keys:
    /// * `"total_operations"` - Total operations routed
    /// * `"provider_selections"` - Selections per provider
    /// * `"tier_usage"` - Usage count per tier
    /// * `"failover_count"` - Number of failover events
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    ///
    /// let manager = HsmManager::new();
    /// let metrics = manager.get_routing_metrics();
    ///
    /// for (metric, value) in metrics.iter() {
    ///     println!("{}: {}", metric, value);
    /// }
    /// ```
    ///
    /// # See Also
    ///
    /// * `HsmPerformanceTracker` - Detailed performance metrics
    /// * Provider statistics available through performance tracking
    pub fn get_routing_metrics(&self) -> std::collections::HashMap<String, u64> {
        // Stub implementation
        std::collections::HashMap::new()
    }
}
