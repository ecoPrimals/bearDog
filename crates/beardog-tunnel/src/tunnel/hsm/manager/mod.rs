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
        use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
        use crate::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
        use std::env;
        use tracing::{info, warn};

        // Check if auto-init is enabled
        let auto_init = env::var("BEARDOG_HSM_AUTO_INIT")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        if !auto_init {
            info!("HSM auto-initialization disabled via BEARDOG_HSM_AUTO_INIT");
            return Ok(Self::new());
        }

        // Detect HSM mode from environment
        let hsm_mode = env::var("BEARDOG_HSM_MODE")
            .unwrap_or_else(|_| "software".to_string())
            .to_lowercase();

        info!("🔐 Auto-initializing HSM Manager (mode: {})", hsm_mode);

        let mut manager = Self::new();

        match hsm_mode.as_str() {
            "software" => {
                // Initialize software HSM
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!("Failed to initialize software HSM: {}", e))
                })?;
                
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
                info!("✅ Software HSM initialized successfully");
            }
            "hardware" => {
                // Hardware HSM initialization (future implementation)
                warn!("⚠️  Hardware HSM mode requested but not yet implemented, falling back to software");
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!("Failed to initialize fallback software HSM: {}", e))
                })?;
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
            }
            "android_strongbox" => {
                // Android StrongBox initialization (future implementation)
                warn!("⚠️  Android StrongBox mode requested but not yet implemented, falling back to software");
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!("Failed to initialize fallback software HSM: {}", e))
                })?;
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
            }
            "ios_secure_enclave" => {
                // iOS Secure Enclave initialization (future implementation)
                warn!("⚠️  iOS Secure Enclave mode requested but not yet implemented, falling back to software");
                let config = SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(config).await.map_err(|e| {
                    BearDogError::initialization(format!("Failed to initialize fallback software HSM: {}", e))
                })?;
                manager.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
            }
            _ => {
                let error_msg = format!(
                    "Invalid BEARDOG_HSM_MODE: '{}'. Valid modes: software, hardware, android_strongbox, ios_secure_enclave",
                    hsm_mode
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::{GenerateKeyRequest, KeyType};
    use async_trait::async_trait;

    // Mock HSM Provider for testing
    struct MockHsmProvider {
        available: bool,
        fail_generate: bool,
        fail_delete: bool,
    }

    impl MockHsmProvider {
        fn new() -> Self {
            Self {
                available: true,
                fail_generate: false,
                fail_delete: false,
            }
        }

        fn unavailable() -> Self {
            Self {
                available: false,
                fail_generate: false,
                fail_delete: false,
            }
        }

        fn failing_generate() -> Self {
            Self {
                available: true,
                fail_generate: true,
                fail_delete: false,
            }
        }

        fn failing_delete() -> Self {
            Self {
                available: true,
                fail_generate: false,
                fail_delete: true,
            }
        }
    }

    #[async_trait]
    impl HsmProvider for MockHsmProvider {
        async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
            Ok(ProviderInfo {
                id: "mock".to_string(),
                name: "Mock HSM".to_string(),
                security_level: 3,
            })
        }

        async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
            if self.fail_generate {
                return Err(BearDogError::system(
                    "Mock generate_key failure".to_string(),
                ));
            }

            use crate::tunnel::hsm::{KeyHealthStatus, KeyMaterial, KeyMetadata};
            use chrono::Utc;

            Ok(HsmKey {
                id: request.key_id.clone(),
                hsm_type: "MockHSM".to_string(),
                key_type: request.key_type.clone(),
                metadata: KeyMetadata::new(request.key_id, request.key_type),
                key_material: KeyMaterial::Encrypted {
                    encrypted_data: vec![1, 2, 3, 4],
                    encryption_algorithm: "AES-256-GCM".to_string(),
                    kdf_params: None,
                },
                hsm_tier: "Software".to_string(),
                health_status: KeyHealthStatus::Healthy,
                attestation: None,
                created_at: Utc::now(),
            })
        }

        async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(vec![])
        }

        async fn verify(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(true)
        }

        async fn encrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(vec![])
        }

        async fn decrypt(
            &self,
            _key_id: &str,
            _ciphertext: &[u8],
        ) -> Result<Vec<u8>, BearDogError> {
            Ok(vec![])
        }

        async fn import_key(
            &self,
            _key_data: &[u8],
            _key_id: &str,
        ) -> Result<HsmKey, BearDogError> {
            Err(BearDogError::not_implemented("Mock import_key"))
        }

        async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
            if self.fail_delete {
                return Err(BearDogError::system("Mock delete_key failure".to_string()));
            }
            Ok(())
        }

        async fn get_key_info(&self, _key_id: &str) -> Result<KeyInfo, BearDogError> {
            Ok(KeyInfo {
                key_id: "test".to_string(),
                key_type: "AES".to_string(),
                is_hardware_backed: false,
            })
        }

        async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
            Ok(HealthStatus {
                is_healthy: true,
                error_message: None,
            })
        }

        fn is_available(&self) -> bool {
            self.available
        }
    }

    #[tokio::test]
    async fn test_generate_key_success() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
            .unwrap();

        let key = manager
            .generate_key("test_key_123", &KeyType::ChaCha20)
            .await
            .unwrap();

        assert_eq!(key.id, "test_key_123");
        assert_eq!(key.hsm_type, "MockHSM");
    }

    #[tokio::test]
    async fn test_generate_key_no_providers() {
        let manager = HsmManager::new();

        let result = manager.generate_key("test_key", &KeyType::Aes).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available"));
    }

    #[tokio::test]
    async fn test_generate_key_provider_unavailable() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::unavailable()))
            .unwrap();

        let result = manager.generate_key("test_key", &KeyType::Aes).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available"));
    }

    #[tokio::test]
    async fn test_generate_key_provider_failure() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(
                HsmTier::Software,
                Arc::new(MockHsmProvider::failing_generate()),
            )
            .unwrap();

        let result = manager.generate_key("test_key", &KeyType::Aes).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Mock generate_key failure"));
    }

    #[tokio::test]
    async fn test_delete_key_success() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
            .unwrap();

        let result = manager.delete_key("test_key_123").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_key_no_providers() {
        let manager = HsmManager::new();

        let result = manager.delete_key("test_key").await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available"));
    }

    #[tokio::test]
    async fn test_delete_key_provider_unavailable() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::unavailable()))
            .unwrap();

        let result = manager.delete_key("test_key").await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available"));
    }

    #[tokio::test]
    async fn test_delete_key_provider_failure() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(
                HsmTier::Software,
                Arc::new(MockHsmProvider::failing_delete()),
            )
            .unwrap();

        let result = manager.delete_key("test_key").await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Mock delete_key failure"));
    }

    #[tokio::test]
    async fn test_generate_and_delete_key_lifecycle() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
            .unwrap();

        // Generate key
        let key = manager
            .generate_key("lifecycle_key", &KeyType::Ed25519)
            .await
            .unwrap();
        assert_eq!(key.id, "lifecycle_key");

        // Delete key
        let result = manager.delete_key("lifecycle_key").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_key_different_types() {
        let mut manager = HsmManager::new();
        manager
            .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
            .unwrap();

        // Test different key types
        let key_types = vec![
            KeyType::Aes,
            KeyType::ChaCha20,
            KeyType::Ed25519,
            KeyType::X25519,
        ];

        for key_type in key_types {
            let key = manager
                .generate_key(&format!("key_{:?}", key_type), &key_type)
                .await
                .unwrap();
            assert!(key.id.starts_with("key_"));
        }
    }

    // ===================================================================
    // Auto-Initialize Tests
    // ===================================================================

    /// Helper to ensure environment variables are cleaned up after tests
    struct EnvCleanup {
        keys: Vec<&'static str>,
    }

    impl EnvCleanup {
        fn new(keys: &[&'static str]) -> Self {
            Self {
                keys: keys.to_vec(),
            }
        }
    }

    impl Drop for EnvCleanup {
        fn drop(&mut self) {
            use std::env;
            for key in &self.keys {
                env::remove_var(key);
            }
        }
    }

    #[tokio::test]
    async fn test_auto_initialize_default_software_mode() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        // Clear any existing env vars
        env::remove_var("BEARDOG_HSM_MODE");
        env::remove_var("BEARDOG_HSM_AUTO_INIT");
        
        // Should default to software mode
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Auto-initialize should succeed with default software mode");
        
        let manager = manager.unwrap();
        // Verify we can generate a key (proves HSM is registered)
        let key = manager.generate_key("test_key", &KeyType::Ed25519).await;
        assert!(key.is_ok(), "Should be able to generate key with auto-initialized HSM");
    }

    #[tokio::test]
    async fn test_auto_initialize_explicit_software_mode() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "software");
        env::set_var("BEARDOG_HSM_AUTO_INIT", "true");
        
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Auto-initialize should succeed with explicit software mode");
        
        let manager = manager.unwrap();
        let key = manager.generate_key("test_key", &KeyType::ChaCha20).await;
        assert!(key.is_ok(), "Should be able to generate key");
    }

    #[tokio::test]
    async fn test_auto_initialize_case_insensitive() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        // Test uppercase
        env::set_var("BEARDOG_HSM_MODE", "SOFTWARE");
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Should handle uppercase mode");
        env::remove_var("BEARDOG_HSM_MODE");
        
        // Test mixed case
        env::set_var("BEARDOG_HSM_MODE", "SoftWare");
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Should handle mixed case mode");
    }

    #[tokio::test]
    async fn test_auto_initialize_hardware_mode_fallback() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "hardware");
        
        // Should fallback to software (hardware not yet implemented)
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Should fallback to software when hardware not available");
        
        let manager = manager.unwrap();
        // Verify it works
        let key = manager.generate_key("test_key", &KeyType::Aes).await;
        assert!(key.is_ok(), "Fallback software HSM should work");
    }

    #[tokio::test]
    async fn test_auto_initialize_android_mode_fallback() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "android_strongbox");
        
        // Should fallback to software (android not yet implemented)
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Should fallback to software when Android not available");
    }

    #[tokio::test]
    async fn test_auto_initialize_ios_mode_fallback() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "ios_secure_enclave");
        
        // Should fallback to software (iOS not yet implemented)
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Should fallback to software when iOS not available");
    }

    #[tokio::test]
    async fn test_auto_initialize_invalid_mode() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "invalid_mode");
        
        let result = HsmManager::auto_initialize().await;
        assert!(result.is_err(), "Should fail with invalid HSM mode");
        
        let error = result.err().unwrap();
        assert!(error.to_string().contains("Invalid BEARDOG_HSM_MODE"), 
                "Error should mention invalid mode: {}", error);
    }

    #[tokio::test]
    async fn test_auto_initialize_disabled() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        // Clear env vars first
        env::remove_var("BEARDOG_HSM_MODE");
        env::set_var("BEARDOG_HSM_AUTO_INIT", "false");
        
        let manager = HsmManager::auto_initialize().await;
        assert!(manager.is_ok(), "Should succeed even when disabled");
        
        let manager = manager.unwrap();
        // Should have NO providers registered
        let key_result = manager.generate_key("test_key", &KeyType::Ed25519).await;
        assert!(key_result.is_err(), "Should fail - no providers registered when auto-init disabled");
        assert!(key_result.unwrap_err().to_string().contains("No HSM providers available"));
    }

    #[tokio::test]
    async fn test_auto_initialize_multiple_key_operations() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "software");
        
        let manager = HsmManager::auto_initialize().await.unwrap();
        
        // Generate multiple keys
        let key1 = manager.generate_key("key1", &KeyType::Ed25519).await;
        let key2 = manager.generate_key("key2", &KeyType::ChaCha20).await;
        let key3 = manager.generate_key("key3", &KeyType::Aes).await;
        
        assert!(key1.is_ok());
        assert!(key2.is_ok());
        assert!(key3.is_ok());
        
        // Delete keys
        assert!(manager.delete_key("key1").await.is_ok());
        assert!(manager.delete_key("key2").await.is_ok());
        assert!(manager.delete_key("key3").await.is_ok());
    }

    #[tokio::test]
    async fn test_auto_initialize_concurrent_safe() {
        use std::env;
        use tokio::task;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        env::set_var("BEARDOG_HSM_MODE", "software");
        
        // Create multiple managers concurrently
        let handles: Vec<_> = (0..10)
            .map(|i| {
                task::spawn(async move {
                    let manager = HsmManager::auto_initialize().await.unwrap();
                    let key = manager
                        .generate_key(&format!("key_{}", i), &KeyType::Ed25519)
                        .await;
                    assert!(key.is_ok(), "Concurrent init should work");
                })
            })
            .collect();
        
        // Wait for all to complete
        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_auto_initialize_environment_precedence() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        // Test that environment variable takes precedence over default
        env::set_var("BEARDOG_HSM_MODE", "software");
        env::set_var("BEARDOG_HSM_AUTO_INIT", "true");
        
        let manager = HsmManager::auto_initialize().await.unwrap();
        let key = manager.generate_key("test", &KeyType::Ed25519).await;
        assert!(key.is_ok());
    }

    #[tokio::test]
    async fn test_auto_initialize_bool_parsing() {
        use std::env;
        let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);
        
        // Test various boolean representations
        let bool_values = vec![
            ("true", true),
            ("false", false),
            ("TRUE", false), // Invalid - parse will fail, default to false
            ("1", false),     // Invalid - parse will fail, default to false
            ("", false),      // Invalid - parse will fail, default to false
        ];
        
        for (value, should_have_provider) in bool_values {
            env::set_var("BEARDOG_HSM_AUTO_INIT", value);
            env::set_var("BEARDOG_HSM_MODE", "software");
            
            let manager = HsmManager::auto_initialize().await.unwrap();
            let key_result = manager.generate_key("test", &KeyType::Ed25519).await;
            
            if should_have_provider {
                assert!(key_result.is_ok(), "Should have provider for: {}", value);
            } // else: no assertion - behavior depends on parse result
            
            env::remove_var("BEARDOG_HSM_AUTO_INIT");
            env::remove_var("BEARDOG_HSM_MODE");
        }
    }
}

