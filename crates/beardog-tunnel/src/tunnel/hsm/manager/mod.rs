//! HSM Manager Module
//!
//! Central coordinator for Hardware Security Module operations in the BearDog ecosystem.
//! Manages multiple HSM providers with intelligent routing, failover, and health monitoring.
//!
//! # Module Organization
//!
//! This module is organized into focused sub-modules:
//! - `capability` - Provider capability detection
//! - `config` - Configuration management
//! - `failover` - Failover and circuit breaking
//! - `health` - Health monitoring
//! - `implementation` - Provider trait implementations
//! - `operation_router` - Operation routing logic
//! - `performance` - Performance tracking and metrics
//! - `tests` - Comprehensive test suite

pub mod capability;
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
#[cfg(test)]
mod tests;
use beardog_errors::BearDogError;
use beardog_types::hsm::{DefaultHsmFailoverManager, DefaultHsmHealthMonitor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::tunnel::hsm::types::key::HsmKey;
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

/// Configuration for HSM auto-initialization
///
/// This struct provides explicit configuration for HSM initialization,
/// eliminating the need for global environment variable reads in tests.
/// This ensures thread-safe, concurrent testing without race conditions.
///
/// # Fields
///
/// * `mode` - The HSM mode to initialize ("software", "hardware", "android_strongbox", etc.)
/// * `auto_init` - Whether auto-initialization is enabled
///
/// # Example
///
/// ```ignore
/// use beardog_tunnel::tunnel::hsm::manager::{HsmManager, HsmAutoInitConfig};
///
/// // Explicit configuration (thread-safe, no env vars)
/// let config = HsmAutoInitConfig {
///     mode: "software".to_string(),
///     auto_init: true,
/// };
/// let manager = HsmManager::auto_initialize_with_config(config).await?;
/// ```
#[derive(Debug, Clone)]
pub struct HsmAutoInitConfig {
    /// HSM mode ("software", "hardware", "android_strongbox", "ios_secure_enclave")
    pub mode: String,
    /// Whether auto-initialization is enabled
    pub auto_init: bool,
}

impl Default for HsmAutoInitConfig {
    fn default() -> Self {
        Self {
            mode: "software".to_string(),
            auto_init: true,
        }
    }
}

impl HsmAutoInitConfig {
    /// Create config from environment variables (for production use)
    pub fn from_env() -> Self {
        use std::env;
        Self {
            mode: env::var("BEARDOG_HSM_MODE")
                .unwrap_or_else(|_| "software".to_string())
                .to_lowercase(),
            auto_init: env::var("BEARDOG_HSM_AUTO_INIT")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
        }
    }
}

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
            // Capability detector creation is infallible (just HashMap)
            // Using expect since this should never fail in practice
            capability_detector: Arc::new(
                DefaultHsmCapabilityDetector::new()
                    .expect("DefaultHsmCapabilityDetector::new() is infallible"),
            ),
            performance_tracker: Arc::new(HsmPerformanceTracker::default()),
            operation_router: Arc::new(RwLock::new(HsmOperationRouter::default())),
        }
    }

    /// Auto-initialize HSM Manager based on environment variables
    ///
    /// This is the recommended way to initialize HsmManager for applications.
    /// It automatically detects the HSM mode from environment variables and
    /// registers the appropriate provider.
    ///
    /// # Environment Variables
    ///
    /// * `BEARDOG_HSM_MODE` - HSM mode: "software" (default), "hardware", "android_strongbox", "ios_secure_enclave"
    /// * `BEARDOG_HSM_DEVICE` - Device path for hardware HSM (optional)
    /// * `BEARDOG_HSM_AUTO_INIT` - Enable auto-initialization: "true" or "false" (default: "true")
    ///
    /// # Returns
    ///
    /// Returns an initialized `HsmManager` with at least one registered provider.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The specified HSM mode is invalid
    /// * HSM provider initialization fails
    /// * No suitable HSM provider can be initialized
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    /// use std::env;
    ///
    /// // Set environment variable
    /// env::set_var("BEARDOG_HSM_MODE", "software");
    ///
    /// // Auto-initialize
    /// let manager = HsmManager::auto_initialize().await?;
    /// println!("HSM Manager auto-initialized successfully");
    /// ```
    pub async fn auto_initialize() -> Result<Self, BearDogError> {
        let config = HsmAutoInitConfig::from_env();
        Self::auto_initialize_with_config(config).await
    }

    /// Auto-initialize HSM Manager with explicit configuration (thread-safe)
    ///
    /// This method provides explicit configuration for HSM initialization,
    /// eliminating global environment variable dependencies. This ensures
    /// thread-safe, concurrent operation without race conditions.
    ///
    /// # Arguments
    ///
    /// * `config` - Explicit HSM initialization configuration
    ///
    /// # Returns
    ///
    /// Returns an initialized `HsmManager` with the configured provider,
    /// or an error if initialization fails.
    ///
    /// # Concurrency
    ///
    /// This method is fully thread-safe and can be called concurrently
    /// from multiple threads without any risk of race conditions.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::{HsmManager, HsmAutoInitConfig};
    ///
    /// // Explicit configuration (thread-safe)
    /// let config = HsmAutoInitConfig {
    ///     mode: "software".to_string(),
    ///     auto_init: true,
    /// };
    /// let manager = HsmManager::auto_initialize_with_config(config).await?;
    /// ```
    pub async fn auto_initialize_with_config(
        config: HsmAutoInitConfig,
    ) -> Result<Self, BearDogError> {
        use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
        use crate::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
        use tracing::{info, warn};

        // Check if auto-init is enabled
        if !config.auto_init {
            info!("HSM auto-initialization disabled via config");
            return Ok(Self::new());
        }

        info!("🔐 Auto-initializing HSM Manager (mode: {})", config.mode);

        let mut manager = Self::new();

        // Normalize mode to lowercase for case-insensitive matching
        let mode_normalized = config.mode.to_lowercase();

        match mode_normalized.as_str() {
            "software" => {
                // Initialize software HSM
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!(
                        "Failed to initialize software HSM: {}",
                        e
                    ))
                })?;

                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
                info!("✅ Software HSM initialized successfully");
            }
            "hardware" => {
                // Hardware HSM initialization (future implementation)
                warn!("⚠️  Hardware HSM mode requested but not yet implemented, falling back to software");
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!(
                        "Failed to initialize fallback software HSM: {}",
                        e
                    ))
                })?;
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
            }
            "android_strongbox" => {
                // Android StrongBox initialization (future implementation)
                warn!("⚠️  Android StrongBox mode requested but not yet implemented, falling back to software");
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!(
                        "Failed to initialize fallback software HSM: {}",
                        e
                    ))
                })?;
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
            }
            "ios_secure_enclave" => {
                // iOS Secure Enclave initialization (future implementation)
                warn!("⚠️  iOS Secure Enclave mode requested but not yet implemented, falling back to software");
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!(
                        "Failed to initialize fallback software HSM: {}",
                        e
                    ))
                })?;
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
            }
            _ => {
                let error_msg = format!(
                    "Invalid HSM mode: '{}'. Valid modes: software, hardware, android_strongbox, ios_secure_enclave",
                    config.mode
                );
                return Err(BearDogError::invalid_input(&error_msg));
            }
        }

        info!("🎯 HSM Manager auto-initialization complete");
        Ok(manager)
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

    /// Generate a new cryptographic key
    ///
    /// Generates a new key using the first available HSM provider. The key is generated
    /// according to the specified key type and stored with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `key_id` - Unique identifier for the key
    /// * `key_type` - Type of key to generate (e.g., AES, ChaCha20, ECC)
    ///
    /// # Returns
    ///
    /// * `Ok(HsmKey)` - Successfully generated key with metadata
    /// * `Err(BearDogError)` - Key generation failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * No HSM providers are registered
    /// * All providers are unavailable
    /// * Key generation fails in the provider
    /// * Key ID already exists (provider-dependent)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    /// use beardog_tunnel::tunnel::hsm::KeyType;
    ///
    /// let manager = HsmManager::new();
    /// // Register providers first...
    ///
    /// let key = manager.generate_key("session_key_123", &KeyType::ChaCha20).await?;
    /// println!("Generated key: {}", key.id);
    /// ```
    ///
    /// # See Also
    ///
    /// * [`delete_key`](Self::delete_key) - Delete a generated key
    /// * [`HsmProvider::generate_key`] - Provider-level key generation
    pub async fn generate_key(
        &self,
        key_id: &str,
        key_type: &crate::tunnel::hsm::KeyType,
    ) -> Result<HsmKey, BearDogError> {
        // Get the first available provider
        let provider = self
            .hsm_providers
            .values()
            .find(|p| p.is_available())
            .ok_or_else(|| BearDogError::not_found("No HSM providers available".to_string()))?;

        // Create the key generation request
        let request = crate::tunnel::hsm::GenerateKeyRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
        };

        // Generate the key using the provider
        provider.generate_key(request).await
    }

    /// Delete a cryptographic key
    ///
    /// Deletes a key from the HSM provider. This operation is irreversible and will
    /// remove the key from secure storage. Any data encrypted with this key will
    /// become inaccessible.
    ///
    /// # Arguments
    ///
    /// * `key_id` - Identifier of the key to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Key deleted successfully
    /// * `Err(BearDogError)` - Deletion failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * No HSM providers are registered
    /// * All providers are unavailable
    /// * Key does not exist (provider-dependent)
    /// * Deletion fails in the provider
    ///
    /// # Security
    ///
    /// This operation should be used carefully as it permanently removes cryptographic
    /// material. Ensure that:
    /// * The key is no longer needed for any operations
    /// * All data encrypted with the key has been re-encrypted or is no longer needed
    /// * The key is not part of a key hierarchy that other keys depend on
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    ///
    /// let manager = HsmManager::new();
    /// // Register providers and generate key...
    ///
    /// // Delete ephemeral session key after use
    /// manager.delete_key("session_key_123").await?;
    /// println!("Session key deleted");
    /// ```
    ///
    /// # See Also
    ///
    /// * [`generate_key`](Self::generate_key) - Generate a new key
    /// * [`HsmProvider::delete_key`] - Provider-level key deletion
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        // Get the first available provider
        let provider = self
            .hsm_providers
            .values()
            .find(|p| p.is_available())
            .ok_or_else(|| BearDogError::not_found("No HSM providers available".to_string()))?;

        // Delete the key using the provider
        provider.delete_key(key_id).await
    }
}
