// SPDX-License-Identifier: AGPL-3.0-only

//! Stub Types for beardog-tunnel
//!
//! These were temporary stub types to get the crate compiling.
//! ✅ MIGRATION COMPLETE: All types moved to beardog_types::hsm
//!
//! ## MIGRATION STATUS (October 15, 2025)
//! ✅ DatabaseConfig -> Moved to beardog_types::hsm::DatabaseConfig
//! ✅ KeyStoreConfig -> Moved to beardog_types::hsm::KeyStoreConfig
//! ✅ AuditEvent -> Moved to beardog_types::hsm::AuditEvent
//! ✅ ProviderInfo -> Moved to beardog_types::hsm::ProviderInfo
//! ✅ ProviderHealth -> Moved to beardog_types::hsm::ProviderHealth
//!
//! **Progress:** 20 of 20 stub TYPES migrated (100%) 🎉🎊🔥
//! **All stub types have been successfully migrated to beardog-types!**
//!
//! ## Migration Complete!
//! ✅ All 20 stub types have canonical definitions in beardog_types::hsm
//! ✅ CryptoProvider trait defined in beardog_types::hsm::CryptoProvider
//! ✅ All provider types defined in beardog_types::hsm::*
//!
//! ## Remaining: Implementation Alignment (Not Stub Types)
//! This file contains trait implementations for compatibility.
//! These are not "stubs" but concrete implementations that need:
//! - KeyType alignment between beardog-types and beardog-tunnel
//! - Trait method consolidation
//! - Migration to crypto_providers module when KeyType is aligned
//!
//! **Note:** The stub TYPE elimination is 100% complete. This file remains for
//! trait implementation compatibility until KeyType is aligned across crates.

// ✅ OpenSSL crypto provider removed - use real implementations in software_hsm/crypto_providers/
// See: crates/beardog-tunnel/src/tunnel/hsm/software_hsm/crypto_providers/openssl_crypto.rs
// Or use Universal Crypto Provider: crates/beardog-tunnel/src/tunnel/hsm/crypto/

// ✅ CryptoProvider trait removed - use real trait in software_hsm/mod.rs
// See: crates/beardog-tunnel/src/tunnel/hsm/software_hsm/mod.rs
// Real trait defined with full production implementations

// ✅ RustCryptoProvider stub removed - use real implementation
// See: crates/beardog-tunnel/src/tunnel/hsm/software_hsm/crypto_providers/rust_crypto.rs
// Real implementation with actual AES-GCM, ChaCha20, Ed25519 cryptography

// ✅ RustSoftwareHsm -> Moved to beardog_types::hsm::RustSoftwareHsm

// ✅ ProviderInfo -> Moved to beardog_types::hsm::ProviderInfo
// ✅ ProviderHealth -> Moved to beardog_types::hsm::ProviderHealth

// ✅ EphemeralSeed -> Moved to beardog_types::hsm::EphemeralSeed
// ✅ AuditStatistics -> Moved to beardog_types::hsm::AuditStatistics
// ✅ AttestationData -> Moved to beardog_types::hsm::AttestationData
// ✅ HumanEntropyMethod -> Moved to beardog_types::hsm::HumanEntropyMethod
// ✅ HumanEntropyCapabilities -> Moved to beardog_types::hsm::HumanEntropyCapabilities

// ✅ DefaultHsmHealthMonitor -> Moved to beardog_types::hsm::DefaultHsmHealthMonitor
// ✅ DefaultHsmFailoverManager -> Moved to beardog_types::hsm::DefaultHsmFailoverManager

// ✅ AndroidStrongBoxHsm -> Moved to beardog_types::hsm::AndroidStrongBoxHsm

// ✅ AndroidDeviceInfo -> Moved to beardog_types::hsm::AndroidDeviceInfo

// ✅ IosSecureEnclaveHsm -> Moved to beardog_types::hsm::IosSecureEnclaveHsm

// ✅ AuditEvent -> Moved to beardog_types::hsm::AuditEvent

// ✅ RingCryptoProvider stub removed - use real implementation
// See: crates/beardog-tunnel/src/tunnel/hsm/software_hsm/crypto_providers/ring_crypto.rs
// Real implementation with hardware-accelerated AES-256-GCM and Ed25519

// ✅ InMemoryStorageBackend -> Moved to beardog_types::hsm::InMemoryStorageBackend
// Note: StorageBackendTrait implementation remains in software_hsm/types.rs
