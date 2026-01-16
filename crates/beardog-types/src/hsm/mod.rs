//! HSM (Hardware Security Module) type definitions
//!
//! This module provides comprehensive type definitions for HSM operations,
//! configurations, and integrations across multiple platforms.

pub mod attestation;
pub mod audit;
pub mod config;
pub mod crypto;
pub mod crypto_providers;
pub mod entropy;
pub mod health;
pub mod implementations;
pub mod key_lifecycle;
pub mod managers;
pub mod mobile;
pub mod mobile_hsm;
pub mod providers;

// Re-export commonly used types
pub use attestation::{AttestationData, AuditStatistics};
pub use audit::AuditEvent;
pub use config::{DatabaseConfig, HsmConfig, KeyStoreConfig};
pub use crypto::CryptoProvider;
pub use crypto_providers::{OpenSslCryptoProvider, RustCryptoProvider};
// RingCryptoProvider removed (Jan 16, 2026) - evolved to RustCryptoProvider (100% Pure Rust)
pub use entropy::{EphemeralSeed, HumanEntropyCapabilities, HumanEntropyMethod};
pub use health::{ProviderHealth, ProviderInfo};
pub use implementations::{InMemoryStorageBackend, RustSoftwareHsm};
pub use key_lifecycle::{
    KeyLifecycleState, KeyMetadataWithLifecycle, KeyRotationConfig, KeyRotationEvent,
    KeyRotationReason,
};
pub use managers::{DefaultHsmFailoverManager, DefaultHsmHealthMonitor};
pub use mobile::{AndroidDeviceInfo, SecurityLevel};
pub use mobile_hsm::{AndroidStrongBoxHsm, IosSecureEnclaveHsm};
pub use providers::{CryptoProviderConfig, ProviderCapabilities};
