// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! # `BearDog` Security Crate
//!
//! Comprehensive security functionality for the `BearDog` platform, providing
//! cryptographic operations, key management, and hardware security module (HSM) integration.
//!
//! ## Features
//!
//! - **Quantum-Resistant Cryptography**: Post-quantum cryptographic algorithms
//! - **Hardware Security Modules**: Integration with `YubiKey`, TPM, and software HSMs
//! - **Memory-safe only**: All operations are memory-safe
//! - **SIMD Acceleration**: Hardware-accelerated cryptographic operations
//! - **Secure Key Management**: Safe key storage and lifecycle management
//!
//! ## Core Components
//!
//! - [`encryption`]: Encryption and decryption operations
//! - [`memory_key_manager`]: In-memory secure key management
//! - [`simd_crypto`]: SIMD-accelerated cryptographic primitives
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_security::compute_sha256_hash;
//!
//! let data = b"Hello, BearDog!";
//! let hash = compute_sha256_hash(data)?;
//! tracing::info!("SHA-256: {:?}", hash);
//! # Ok::<(), beardog_errors::BearDogError>(())
//! ```
//!
//! ## Safety
//!
//! This crate maintains full memory safety, ensuring complete memory safety
//! for all security-critical operations. All cryptographic operations are
//! compiler-verified for safety.
//!
//! ## Performance
//!
//! SIMD acceleration provides 2-5x performance improvement for cryptographic
//! operations when available, automatically falling back to safe scalar
//! implementations on unsupported platforms.

/// Authorization types and permission management
///
/// Provides types and utilities for managing authorization, permissions,
/// and access control within the `BearDog` security system.
pub mod authorization_types;

/// Cryptographic utility functions
///
/// Core cryptographic operations including hashing, HMAC, signing,
/// and secure random number generation.
pub mod crypto_utils;

/// Encryption services and algorithms
///
/// High-level encryption and decryption services supporting multiple
/// symmetric algorithms with secure key management.
pub mod encryption;

pub mod hsm;
pub mod key_rotation_manager;

/// In-memory key management system
///
/// Secure key storage and management with memory protection,
/// designed for temporary key handling and secure operations.
pub mod memory_key_manager;

/// SIMD-accelerated cryptographic operations
///
/// Hardware-accelerated crypto functions using SIMD instructions
/// for improved performance on supported platforms.
pub mod simd_crypto;

/// Genesis module - Physical Bootstrap with Cryptographic Witness
///
/// Implements physical genesis bootstrap for new nodes, ensuring they
/// receive cryptographic identity at birth via witnessed ceremony.
///
/// **"Never let a bird be alone in the dark forest"**
pub mod genesis;

/// Simulated post-quantum cryptography (requires `pqc-simulation` feature or `cfg(test)`).
///
/// **Not real PQC** — uses placeholder logic for API wiring only. See module docs.
#[cfg(any(test, feature = "pqc-simulation"))]
pub mod quantum_crypto;

/// Whether the simulated PQC module is compiled in (never true in production builds
/// unless `pqc-simulation` is explicitly enabled).
#[must_use]
pub const fn pqc_simulation_enabled() -> bool {
    cfg!(any(test, feature = "pqc-simulation"))
}

mod primitives;

// Comprehensive test modules
#[cfg(test)]
mod tests;

#[cfg(test)]
mod security_operations_comprehensive_tests;

// Re-export main types and functions
pub use authorization_types::*;
pub use beardog_errors::BearDogError;
pub use encryption::*;
pub use genesis::{
    GenesisWitness, GenesisWitnessVerifier, PhysicalChannelType, PhysicalProofError,
    PhysicalProximityVerifier, TrustLevel, WitnessVerificationError,
};
pub use key_rotation_manager::{KeyRotationManager, RotationStatistics};
pub use memory_key_manager::*;
pub use primitives::{
    compute_sha256_hash, compute_sha512_hash, constant_time_compare, derive_key_from_password,
    generate_secure_random_bytes, secure_zero_memory,
};
