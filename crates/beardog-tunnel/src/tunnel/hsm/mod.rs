// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM (Hardware Security Module) integration for BearDog
//!
//! Provides multi-platform HSM support with zero-cost abstractions.

/// Core HSM implementations
pub mod software_hsm;

#[cfg(target_os = "android")]
pub mod android_strongbox;

#[cfg(windows)]
pub mod windows_dpapi;

#[cfg(target_os = "linux")]
pub mod linux_secret_service;

// Solo V2 USB security key support
#[cfg(feature = "solo-v2")]
pub mod solo_v2;

// HSM infrastructure
// ARCHIVED: capabilities.rs → HsmCapabilityDetector is in manager/capability.rs
pub mod config;
pub mod manager;
/// HSM provider implementations
pub mod providers;

mod hsm_key_provider_backend;
mod hsm_provider_backend;
#[cfg(test)]
pub mod hsm_provider_mocks;
pub use hsm_key_provider_backend::HsmKeyProviderBackend;
pub use hsm_provider_backend::HsmProviderBackend;
/// HSM type definitions and enums
pub mod types;

// Platform abstractions
pub mod safe_ffi;
pub mod universal_discovery;

// Re-export discovery types for CLI usage
pub use universal_discovery::{
    DiscoveredHsm, DiscoveryEngine, HsmConnectionInfo, HsmHealthStatus as DiscoveryHsmHealthStatus,
    HsmInterfaceType, UniversalHsmCapabilities,
};

// Crypto system
pub mod crypto; // NEW: Universal Crypto Provider System
pub mod crypto_dispatch;
mod crypto_provider_backend;
// NOTE: provider_dispatch.rs was removed (Jan 2026)
// Performance optimization deferred to Phase 2
pub mod zero_cost_provider;

// Mobile platform support
pub mod mobile_ephemeral_integration;
pub mod mobile_setup;
pub mod native_device_detection;

// Key management
pub mod human_entropy_unified;
/// Key lifecycle management (reserved for HSM key rotation / revocation API)
mod key_manager;

// Performance and health
pub mod failover;
pub mod health;
pub mod performance;
pub mod unified_provider;

// Re-exports for convenience
pub use config::{HsmConfig, SimpleHsmTier};
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

// ✅ MIGRATED: Using real crypto providers from software_hsm/crypto_providers and canonical trait (100% Pure Rust!)
pub use beardog_types::hsm::CryptoProvider; // Canonical trait
pub use crypto_provider_backend::CryptoProviderBackend;
pub use software_hsm::crypto_providers::SoftwareHsmCryptoProvider;
// RingCryptoProvider removed - evolved to RustCrypto (100% Pure Rust, ARM-ready!)
// OpenSslCryptoProvider removed - evolved to pure Rust alternatives
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

/// Request to generate a new key in the HSM
#[derive(Debug, Clone)]
pub struct GenerateKeyRequest {
    /// Unique identifier for the key to generate
    pub key_id: String,
    /// Type of key to generate
    pub key_type: KeyType,
}

/// Information about an HSM device
#[derive(Debug, Clone)]
pub struct HsmInfo {
    /// Unique HSM device identifier
    pub hsm_id: String,
    /// HSM device type (e.g., "software", "hardware")
    pub hsm_type: String,
    /// Whether the HSM is currently available
    pub is_available: bool,
}

/// Security requirements for HSM operations
#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    /// Minimum required security level
    pub min_security_level: SecurityLevel,
    /// Whether hardware-backed operations are required
    pub require_hardware: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
