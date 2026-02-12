//! # Quantum-Resistant Cryptography Module
//!
//! Provides post-quantum cryptographic operations following NIST PQC standards:
//! - **Kyber** KEM (Key Encapsulation Mechanism) - ML-KEM
//! - **Dilithium** signatures - ML-DSA
//! - **SPHINCS+** signatures (hash-based, stateless)
//!
//! ## Module Structure (Smart Refactored)
//!
//! - `types` - Core types (security levels, algorithm enums, keypairs)
//! - `kem` - Key Encapsulation Mechanism (Kyber)
//! - `signatures` - Digital signatures (Dilithium, SPHINCS+)
//! - `engine` - Main `QuantumCryptoEngine` orchestrator
//!
//! ## Security Note
//!
//! Current implementations are **placeholder/simulation** for API design.
//! Production use requires integration with actual PQC libraries when stable
//! (e.g., pqcrypto, liboqs-rust when they reach 1.0).
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
