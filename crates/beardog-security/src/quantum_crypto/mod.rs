// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Quantum-Resistant Cryptography Module
//!
//! ## Simulation only — not production PQC
//!
//! **This module is a SIMULATION-ONLY implementation for API surface design and
//! integration testing. It does not perform real post-quantum cryptography and
//! must not be used for production security, key generation, or data protection.**
//!
//! Operations here use placeholder logic so callers can exercise types, error
//! paths, and wiring before a vetted PQC library is integrated. Treat all keys,
//! ciphertexts, and signatures produced by this module as non-cryptographic mocks.
//!
//! Production PQC will require audited implementations (e.g. ML-KEM / ML-DSA via
//! `pqcrypto`, `liboqs-rust`, or platform FIPS modules) once selected and reviewed.
//!
//! ## Intended API surface (future production target)
//!
//! - **Kyber** KEM (Key Encapsulation Mechanism) — ML-KEM
//! - **Dilithium** signatures — ML-DSA
//! - **SPHINCS+** signatures (hash-based, stateless)
//!
//! ## Module Structure
//!
//! - `types` — Core types (security levels, algorithm enums, keypairs)
//! - `kem` — Key Encapsulation Mechanism (Kyber) — simulated
//! - `signatures` — Digital signatures (Dilithium, SPHINCS+) — simulated
//! - `engine` — Main `QuantumCryptoEngine` orchestrator — simulated
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_security::quantum_crypto::{QuantumCryptoEngine, SecurityLevel};
//!
//! # fn main() -> Result<(), beardog_errors::BearDogError> {
//! let engine = QuantumCryptoEngine::new(SecurityLevel::Level3)?;
//! let kem = engine.generate_kem_keypair()?;
//! let sig = engine.generate_signature_keypair()?;
//! # Ok(())
//! # }
//! ```

mod engine;
mod kem;
mod signatures;
mod types;

// Re-export main types for ergonomic API
pub use engine::QuantumCryptoEngine;
pub use kem::KyberEngine;
pub use signatures::{DilithiumEngine, SphincsEngine};
pub use types::{
    KemAlgorithm, QuantumKEM, QuantumKeyExchange, QuantumPrivateKey, QuantumSignature,
    QuantumSignatureResult, SecurityLevel, SignatureAlgorithm,
};

#[cfg(test)]
mod tests;
