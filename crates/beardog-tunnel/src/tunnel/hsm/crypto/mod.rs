// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal Crypto Provider System
//!
//! Provides vendor-agnostic cryptographic operations through a trait-based
//! abstraction similar to our Universal HSM architecture.
//!
//! This module eliminates crypto library lock-in by allowing runtime discovery
//! and selection of crypto providers based on capabilities and requirements.

pub mod algorithms;
pub mod capabilities;
pub mod manager;
pub mod provider;
pub mod providers;
pub mod requirements;
mod universal_crypto_backend;

pub use universal_crypto_backend::UniversalCryptoBackend;

#[cfg(test)]
mod algorithms_tests;

// Re-exports for convenience
pub use algorithms::{
    AesMode, AlgorithmCategory, AlgorithmSpec, Argon2Variant, AsymmetricAlgorithm, CryptoAlgorithm,
    CryptoOperation, DecryptionOptions, EncryptedData, EncryptionOptions, HashAlgorithm,
    KdfAlgorithm, Signature, SignatureAlgorithm, SigningOptions, SymmetricAlgorithm,
    VerificationOptions,
};
pub use capabilities::{
    CryptoCapabilities, HardwareFeature, PerformanceProfile, Platform, SideChannelResistance,
};
pub use manager::CryptoProviderManager;
pub use provider::UniversalCryptoProvider;
pub use providers::RustCryptoProvider;
pub use requirements::CryptoRequirements;
