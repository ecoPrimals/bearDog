// SPDX-License-Identifier: AGPL-3.0-or-later

//! Quantum Crypto Engine
//!
//! Main orchestrator for quantum-resistant cryptographic operations.

use super::kem::KyberEngine;
use super::signatures::{DilithiumEngine, SphincsEngine};
use super::types::{
    QuantumKEM, QuantumKeyExchange, QuantumSignature, QuantumSignatureResult, SecurityLevel,
    SignatureAlgorithm,
};
use beardog_errors::BearDogError;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Quantum-resistant cryptographic engine
///
/// Orchestrates all post-quantum cryptographic operations:
/// - Key generation (KEM and signatures)
/// - Key encapsulation/decapsulation
/// - Digital signatures
/// - Hybrid classical + quantum-resistant mode
///
/// ## Example
///
/// ```rust,no_run
/// use beardog_security::quantum_crypto::{QuantumCryptoEngine, SecurityLevel};
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// let engine = QuantumCryptoEngine::new(SecurityLevel::Level3)?;
///
/// // Generate keypairs
/// let kem_keypair = engine.generate_kem_keypair()?;
/// let sig_keypair = engine.generate_signature_keypair()?;
///
/// // Key exchange
/// let exchange = engine.encapsulate(&kem_keypair.public_key)?;
///
/// // Sign and verify
/// let signature = engine.sign(&sig_keypair, b"message")?;
/// let valid = engine.verify(&sig_keypair, b"message", &signature.signature)?;
/// # Ok(())
/// # }
/// ```
pub struct QuantumCryptoEngine {
    /// Kyber KEM instance
    kyber_engine: Arc<KyberEngine>,

    /// Dilithium signature instance
    dilithium_engine: Arc<DilithiumEngine>,

    /// SPHINCS+ signature instance
    sphincs_engine: Arc<SphincsEngine>,

    /// Enable hybrid classical + quantum-resistant mode
    hybrid_mode: bool,

    /// Operation counter for metrics
    operations_count: AtomicU64,
}

impl QuantumCryptoEngine {
    /// Create a new quantum crypto engine
    ///
    /// # Arguments
    /// * `security_level` - Security level for all operations
    ///
    /// # Errors
    ///
    /// Returns an error if a sub-engine (Kyber, Dilithium, or SPHINCS+) fails to initialize.
    pub fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        let kyber_engine = Arc::new(KyberEngine::new(security_level)?);
        let dilithium_engine = Arc::new(DilithiumEngine::new(security_level)?);
        let sphincs_engine = Arc::new(SphincsEngine::new(security_level)?);

        Ok(Self {
            kyber_engine,
            dilithium_engine,
            sphincs_engine,
            hybrid_mode: true,
            operations_count: AtomicU64::new(0),
        })
    }

    /// Create with custom configuration
    ///
    /// # Errors
    ///
    /// Returns an error if [`Self::new`] fails.
    pub fn with_config(
        security_level: SecurityLevel,
        hybrid_mode: bool,
    ) -> Result<Self, BearDogError> {
        let mut engine = Self::new(security_level)?;
        engine.hybrid_mode = hybrid_mode;
        Ok(engine)
    }

    // ========== KEM Operations ==========

    /// Generate a KEM keypair
    ///
    /// # Errors
    ///
    /// Returns an error if Kyber key generation fails.
    pub fn generate_kem_keypair(&self) -> Result<QuantumKEM, BearDogError> {
        self.increment_ops();
        self.kyber_engine.generate_keypair()
    }

    /// Encapsulate a shared secret
    ///
    /// # Errors
    ///
    /// Returns an error if encapsulation fails.
    pub fn encapsulate(&self, peer_public_key: &[u8]) -> Result<QuantumKeyExchange, BearDogError> {
        self.increment_ops();
        self.kyber_engine.encapsulate(peer_public_key)
    }

    /// Decapsulate a shared secret
    ///
    /// # Errors
    ///
    /// Returns an error if decapsulation fails.
    pub fn decapsulate(
        &self,
        ciphertext: &[u8],
        private_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        self.increment_ops();
        self.kyber_engine.decapsulate(ciphertext, private_key)
    }

    // ========== Signature Operations ==========

    /// Generate a signature keypair
    ///
    /// Uses Dilithium by default for balance of security and performance.
    ///
    /// # Errors
    ///
    /// Returns an error if Dilithium key generation fails.
    pub fn generate_signature_keypair(&self) -> Result<QuantumSignature, BearDogError> {
        self.increment_ops();
        self.dilithium_engine.generate_keypair()
    }

    /// Generate a signature keypair with specific algorithm
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails for the selected algorithm.
    pub fn generate_signature_keypair_with(
        &self,
        algorithm: SignatureAlgorithm,
    ) -> Result<QuantumSignature, BearDogError> {
        self.increment_ops();
        match algorithm {
            SignatureAlgorithm::Dilithium2
            | SignatureAlgorithm::Dilithium3
            | SignatureAlgorithm::Dilithium5 => self.dilithium_engine.generate_keypair(),
            SignatureAlgorithm::SphincsPlus => self.sphincs_engine.generate_keypair(),
        }
    }

    /// Sign a message
    ///
    /// # Errors
    ///
    /// Returns an error if the private key is missing or signing fails.
    pub fn sign(
        &self,
        keypair: &QuantumSignature,
        message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        self.increment_ops();
        let private_key = keypair
            .private_key
            .as_ref()
            .ok_or_else(|| BearDogError::validation("Private key required for signing"))?;

        match keypair.algorithm {
            SignatureAlgorithm::Dilithium2
            | SignatureAlgorithm::Dilithium3
            | SignatureAlgorithm::Dilithium5 => {
                self.dilithium_engine.sign(private_key.key_data(), message)
            }
            SignatureAlgorithm::SphincsPlus => {
                self.sphincs_engine.sign(private_key.key_data(), message)
            }
        }
    }

    /// Verify a signature
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails internally (simulation currently returns `Ok(true)`).
    pub fn verify(
        &self,
        keypair: &QuantumSignature,
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        self.increment_ops();
        match keypair.algorithm {
            SignatureAlgorithm::Dilithium2
            | SignatureAlgorithm::Dilithium3
            | SignatureAlgorithm::Dilithium5 => {
                self.dilithium_engine
                    .verify(&keypair.public_key, message, signature)
            }
            SignatureAlgorithm::SphincsPlus => {
                self.sphincs_engine
                    .verify(&keypair.public_key, message, signature)
            }
        }
    }

    // ========== Configuration ==========

    /// Check if hybrid mode is enabled
    #[must_use]
    pub const fn is_hybrid_mode(&self) -> bool {
        self.hybrid_mode
    }

    /// Enable or disable hybrid mode
    pub fn set_hybrid_mode(&mut self, enabled: bool) {
        self.hybrid_mode = enabled;
    }

    /// Get total operations count
    #[must_use]
    pub fn operations_count(&self) -> u64 {
        self.operations_count.load(Ordering::Relaxed)
    }

    /// Get security level
    #[must_use]
    pub fn security_level(&self) -> SecurityLevel {
        self.kyber_engine.security_level()
    }

    /// Increment operations counter
    fn increment_ops(&self) {
        self.operations_count.fetch_add(1, Ordering::Relaxed);
    }
}

impl std::fmt::Debug for QuantumCryptoEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuantumCryptoEngine")
            .field("kyber_engine", &self.kyber_engine)
            .field("dilithium_engine", &self.dilithium_engine)
            .field("sphincs_engine", &self.sphincs_engine)
            .field("security_level", &self.security_level())
            .field("hybrid_mode", &self.hybrid_mode)
            .field("operations_count", &self.operations_count())
            .finish()
    }
}

impl Clone for QuantumCryptoEngine {
    fn clone(&self) -> Self {
        Self {
            kyber_engine: Arc::clone(&self.kyber_engine),
            dilithium_engine: Arc::clone(&self.dilithium_engine),
            sphincs_engine: Arc::clone(&self.sphincs_engine),
            hybrid_mode: self.hybrid_mode,
            operations_count: AtomicU64::new(self.operations_count.load(Ordering::Relaxed)),
        }
    }
}
