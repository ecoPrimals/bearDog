// SPDX-License-Identifier: AGPL-3.0-or-later

//! Quantum-Resistant Digital Signatures
//!
//! Implements ML-DSA (formerly Dilithium) and SPHINCS+ for quantum-resistant signatures.
//!
//! ## Security Note
//!
//! These are **simulation implementations** for API design and testing only.
//! Verify operations return `Err` (fail-closed) rather than unconditionally
//! accepting signatures. Production use requires integration with actual PQC
//! libraries (e.g. `pqcrypto`, `ml-dsa`) when stable.

use super::types::{
    QuantumPrivateKey, QuantumSignature, QuantumSignatureResult, SecurityLevel, SignatureAlgorithm,
    dilithium_sizes::{
        DILITHIUM2_PK_SIZE, DILITHIUM2_SIG_SIZE, DILITHIUM2_SK_SIZE, DILITHIUM3_PK_SIZE,
        DILITHIUM3_SIG_SIZE, DILITHIUM3_SK_SIZE, DILITHIUM5_PK_SIZE, DILITHIUM5_SIG_SIZE,
        DILITHIUM5_SK_SIZE,
    },
};
use beardog_errors::BearDogError;
use rand::RngCore;

// ============================================================
// Dilithium Engine
// ============================================================

/// Dilithium signature engine
///
/// Provides ML-DSA (Dilithium) digital signatures.
pub struct DilithiumEngine {
    security_level: SecurityLevel,
}

impl DilithiumEngine {
    /// Create a new Dilithium engine with specified security level
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok`; reserved for future validation.
    pub const fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    /// Generate a Dilithium keypair
    ///
    /// # Errors
    ///
    /// Currently infallible in this simulation.
    pub fn generate_keypair(&self) -> Result<QuantumSignature, BearDogError> {
        let mut rng = rand::rng();

        let (pk_size, sk_size, algorithm) = match self.security_level {
            SecurityLevel::Level1 | SecurityLevel::Level2 => (
                DILITHIUM2_PK_SIZE,
                DILITHIUM2_SK_SIZE,
                SignatureAlgorithm::Dilithium2,
            ),
            SecurityLevel::Level3 => (
                DILITHIUM3_PK_SIZE,
                DILITHIUM3_SK_SIZE,
                SignatureAlgorithm::Dilithium3,
            ),
            SecurityLevel::Level4 | SecurityLevel::Level5 => (
                DILITHIUM5_PK_SIZE,
                DILITHIUM5_SK_SIZE,
                SignatureAlgorithm::Dilithium5,
            ),
        };

        let mut public_key = vec![0u8; pk_size];
        let mut private_key_data = vec![0u8; sk_size];

        rng.fill_bytes(&mut public_key);
        rng.fill_bytes(&mut private_key_data);

        Ok(QuantumSignature {
            public_key,
            private_key: Some(QuantumPrivateKey::new(
                private_key_data,
                format!("{algorithm:?}"),
            )),
            algorithm,
            security_level: self.security_level,
        })
    }

    /// Sign a message
    ///
    /// # Errors
    ///
    /// Returns an error if the system clock is before the UNIX epoch (timestamp for the result).
    pub fn sign(
        &self,
        _private_key: &[u8],
        _message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        let mut rng = rand::rng();

        let (sig_size, algorithm) = match self.security_level {
            SecurityLevel::Level1 | SecurityLevel::Level2 => {
                (DILITHIUM2_SIG_SIZE, SignatureAlgorithm::Dilithium2)
            }
            SecurityLevel::Level3 => (DILITHIUM3_SIG_SIZE, SignatureAlgorithm::Dilithium3),
            SecurityLevel::Level4 | SecurityLevel::Level5 => {
                (DILITHIUM5_SIG_SIZE, SignatureAlgorithm::Dilithium5)
            }
        };

        let mut signature = vec![0u8; sig_size];
        rng.fill_bytes(&mut signature);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::internal(format!("Time error: {e:?}")))?
            .as_secs();

        Ok(QuantumSignatureResult {
            signature,
            algorithm_used: algorithm,
            security_level: self.security_level,
            timestamp,
        })
    }

    /// Verify a signature.
    ///
    /// Fail-closed: simulation cannot perform real cryptographic verification.
    ///
    /// # Errors
    ///
    /// Always returns `BearDogError::not_yet_available` in this simulation.
    pub fn verify(
        &self,
        _public_key: &[u8],
        _message: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::not_yet_available(
            "ML-DSA (Dilithium) verify — PQC library integration pending".to_string(),
        ))
    }

    /// Get the security level
    #[must_use]
    pub const fn security_level(&self) -> SecurityLevel {
        self.security_level
    }
}

impl std::fmt::Debug for DilithiumEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DilithiumEngine")
            .field("security_level", &self.security_level)
            .finish()
    }
}

// ============================================================
// SPHINCS+ Engine
// ============================================================

/// SPHINCS+ public key size (for small variant)
const SPHINCS_PK_SIZE: usize = 32;
/// SPHINCS+ secret key size (for small variant)
const SPHINCS_SK_SIZE: usize = 64;
/// SPHINCS+ signature size (for small variant)
const SPHINCS_SIG_SIZE: usize = 17_088;

/// SPHINCS+ signature engine
///
/// Provides stateless hash-based signatures for maximum security assurance.
pub struct SphincsEngine {
    security_level: SecurityLevel,
}

impl SphincsEngine {
    /// Create a new SPHINCS+ engine with specified security level
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok`; reserved for future validation.
    pub const fn new(security_level: SecurityLevel) -> Result<Self, BearDogError> {
        Ok(Self { security_level })
    }

    /// Generate a SPHINCS+ keypair
    ///
    /// # Errors
    ///
    /// Currently infallible in this simulation.
    pub fn generate_keypair(&self) -> Result<QuantumSignature, BearDogError> {
        let mut rng = rand::rng();

        let mut public_key = vec![0u8; SPHINCS_PK_SIZE];
        let mut private_key_data = vec![0u8; SPHINCS_SK_SIZE];

        rand_core::RngCore::fill_bytes(&mut rng, &mut public_key);
        rand_core::RngCore::fill_bytes(&mut rng, &mut private_key_data);

        Ok(QuantumSignature {
            public_key,
            private_key: Some(QuantumPrivateKey::new(private_key_data, "SPHINCS+")),
            algorithm: SignatureAlgorithm::SphincsPlus,
            security_level: self.security_level,
        })
    }

    /// Sign a message
    ///
    /// # Errors
    ///
    /// Returns an error if the system clock is before the UNIX epoch (timestamp for the result).
    pub fn sign(
        &self,
        _private_key: &[u8],
        _message: &[u8],
    ) -> Result<QuantumSignatureResult, BearDogError> {
        let mut rng = rand::rng();

        let mut signature = vec![0u8; SPHINCS_SIG_SIZE];
        rand_core::RngCore::fill_bytes(&mut rng, &mut signature);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::internal(format!("Time error: {e:?}")))?
            .as_secs();

        Ok(QuantumSignatureResult {
            signature,
            algorithm_used: SignatureAlgorithm::SphincsPlus,
            security_level: self.security_level,
            timestamp,
        })
    }

    /// Verify a signature.
    ///
    /// Fail-closed: simulation cannot perform real cryptographic verification.
    ///
    /// # Errors
    ///
    /// Always returns `BearDogError::not_yet_available` in this simulation.
    pub fn verify(
        &self,
        _public_key: &[u8],
        _message: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::not_yet_available(
            "SPHINCS+ verify — PQC library integration pending".to_string(),
        ))
    }

    /// Get the security level
    #[must_use]
    pub const fn security_level(&self) -> SecurityLevel {
        self.security_level
    }
}

impl std::fmt::Debug for SphincsEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SphincsEngine")
            .field("security_level", &self.security_level)
            .finish()
    }
}
