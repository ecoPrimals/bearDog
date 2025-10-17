//! HSM (Hardware Security Module) integration for BearDog
//!
//! Provides multi-platform HSM support with zero-cost abstractions.

// Core HSM implementations
pub mod software_hsm;

#[cfg(target_os = "android")]
pub mod android_strongbox;

// TEMPORARILY DISABLED: iOS Secure Enclave module has syntax errors
// TODO: Re-enable after fixing types.rs and related files
// #[cfg(target_os = "ios")]
// pub mod ios_secure_enclave;

// HSM infrastructure
// NOTE: capabilities.rs was corrupted and removed - HsmCapabilityDetector is in manager/capability.rs
// pub mod capabilities;
pub mod config;
pub mod manager;
pub mod providers;
pub mod types;

// Platform abstractions
pub mod safe_ffi;
pub mod universal_discovery;

// Crypto and dispatch
pub mod crypto_dispatch;
// NOTE: provider_dispatch depends on universal_hsm which is disabled
// TODO: Re-enable after universal_hsm module rebuild
// pub mod provider_dispatch;
pub mod zero_cost_provider;

// Mobile platform support
pub mod mobile_ephemeral_integration;
pub mod mobile_setup;
pub mod native_device_detection;

// Key management
pub mod human_entropy_unified;
pub mod key_manager;

// Performance and health
pub mod failover;
pub mod health;
pub mod performance;
pub mod unified_provider;

// Stub types (temporary)
pub mod stub_types;

// Re-exports for convenience
pub use config::HsmConfig;
pub use failover::HsmFailoverManager;
pub use health::{HsmHealthMonitor, HsmHealthStatus};
pub use manager::capability::HsmCapabilityDetector;
pub use manager::{HsmManager, HsmProvider};
pub use mobile_ephemeral_integration::HumanEntropyData;
pub use performance::HsmPerformanceTracker;
pub use software_hsm::core::RustSoftwareHsm as SoftwareHsm;
// Use proper types from beardog-types
pub use beardog_types::hsm::{
    AndroidDeviceInfo, AndroidStrongBoxHsm, AttestationData, AuditEvent, AuditStatistics,
    DatabaseConfig, DefaultHsmFailoverManager, DefaultHsmHealthMonitor, EphemeralSeed,
    HumanEntropyCapabilities, HumanEntropyMethod, InMemoryStorageBackend, IosSecureEnclaveHsm,
    KeyStoreConfig, ProviderHealth, ProviderInfo, RustSoftwareHsm,
};

// Use crypto provider implementations from stub_types (until KeyType alignment is complete)
pub use stub_types::{
    CryptoProvider, OpenSslCryptoProvider, RingCryptoProvider, RustCryptoProvider,
};
pub use types::{
    AuthenticationMethod, HsmCapability, HsmKey, HsmKeyInfo, HsmKeyMetadata, HsmOperation, HsmTier,
    KeyType, SecurityLevel,
};

// Re-export key types
pub use types::config::SoftwareHsmConfig;
pub use types::key::{KeyHealthStatus, KeyMaterial, KeyMetadata, UniversalKey};
pub use types::status::HsmHealthStatus as HealthStatus;

#[cfg(target_os = "android")]
pub use android_strongbox::AndroidStrongBox;

#[cfg(target_os = "ios")]
pub use ios_secure_enclave::IosSecureEnclave;

// Request/Response types for compatibility
#[derive(Debug, Clone)]
pub struct GenerateKeyRequest {
    pub key_id: String,
    pub key_type: KeyType,
}

#[derive(Debug, Clone)]
pub struct HsmInfo {
    pub hsm_id: String,
    pub hsm_type: String,
    pub is_available: bool,
}

// Security requirements
#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    pub min_security_level: SecurityLevel,
    pub require_hardware: bool,
}
