// SPDX-License-Identifier: AGPL-3.0-only

//! Quantum-resistant cryptography types
//!
//! Core types for post-quantum cryptographic operations.

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

// ============================================================
// Security Levels
// ============================================================

/// Security level for quantum-resistant operations
///
/// Maps to NIST security levels for post-quantum cryptography.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub enum SecurityLevel {
    /// NIST Level 1 (equivalent to AES-128)
    Level1,

    /// NIST Level 2 (equivalent to SHA-256)
    Level2,

    /// NIST Level 3 (equivalent to AES-192) - **Default**
    #[default]
    Level3,

    /// NIST Level 4 (equivalent to SHA-384)
    Level4,

    /// NIST Level 5 (equivalent to AES-256)
    Level5,
}

// ============================================================
// Algorithm Enums
// ============================================================

/// KEM (Key Encapsulation Mechanism) algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum KemAlgorithm {
    /// Kyber-512 (NIST Level 1)
    Kyber512,

    /// Kyber-768 (NIST Level 3)
    #[default]
    Kyber768,

    /// Kyber-1024 (NIST Level 5)
    Kyber1024,

    /// Classic `McEliece`
    McEliece,
}

/// Signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SignatureAlgorithm {
    /// Dilithium-2 (NIST Level 2)
    Dilithium2,

    /// Dilithium-3 (NIST Level 3)
    #[default]
    Dilithium3,

    /// Dilithium-5 (NIST Level 5)
    Dilithium5,

    /// SPHINCS+ (stateless hash-based signatures)
    SphincsPlus,
}

// ============================================================
// Key Types
// ============================================================

/// Quantum-resistant private key (zeroized on drop)
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct QuantumPrivateKey {
    /// Key data
    key_data: Vec<u8>,

    /// Algorithm identifier
    #[zeroize(skip)]
    algorithm: String,
}

impl QuantumPrivateKey {
    /// Create a new quantum private key
    #[must_use]
    pub fn new(key_data: Vec<u8>, algorithm: impl Into<String>) -> Self {
        Self {
            key_data,
            algorithm: algorithm.into(),
        }
    }

    /// Get key data reference
    #[must_use]
    pub fn key_data(&self) -> &[u8] {
        &self.key_data
    }

    /// Get algorithm identifier
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Key data length
    #[must_use]
    pub fn len(&self) -> usize {
        self.key_data.len()
    }

    /// Check if key is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.key_data.is_empty()
    }
}

impl std::fmt::Debug for QuantumPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuantumPrivateKey")
            .field("algorithm", &self.algorithm)
            .field("key_data", &"[REDACTED]")
            .finish()
    }
}

impl Clone for QuantumPrivateKey {
    fn clone(&self) -> Self {
        Self {
            key_data: self.key_data.clone(),
            algorithm: self.algorithm.clone(),
        }
    }
}

// ============================================================
// Keypair Structures
// ============================================================

/// Quantum KEM keypair
#[derive(Debug, Clone)]
pub struct QuantumKEM {
    /// Public key
    pub public_key: Vec<u8>,

    /// Private key (optional for recipient)
    pub private_key: Option<QuantumPrivateKey>,

    /// Algorithm used
    pub algorithm: KemAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,
}

/// Quantum signature keypair
#[derive(Debug, Clone)]
pub struct QuantumSignature {
    /// Public key
    pub public_key: Vec<u8>,

    /// Private key (optional for verifier)
    pub private_key: Option<QuantumPrivateKey>,

    /// Algorithm used
    pub algorithm: SignatureAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,
}

/// Key exchange result
#[derive(Debug, Clone)]
pub struct QuantumKeyExchange {
    /// Shared secret (derived key material)
    pub shared_secret: Vec<u8>,

    /// Ciphertext (sent to peer for decapsulation)
    pub ciphertext: Vec<u8>,

    /// Algorithm used
    pub algorithm: KemAlgorithm,
}

/// Signature result
#[derive(Debug, Clone)]
pub struct QuantumSignatureResult {
    /// The signature bytes
    pub signature: Vec<u8>,

    /// Algorithm used
    pub algorithm_used: SignatureAlgorithm,

    /// Security level
    pub security_level: SecurityLevel,

    /// Timestamp of signing
    pub timestamp: u64,
}

// ============================================================
// Algorithm Parameters
// ============================================================

/// Kyber key sizes by security level
pub mod kyber_sizes {
    /// Kyber-512 public key size
    pub const KYBER512_PK_SIZE: usize = 800;
    /// Kyber-512 secret key size
    pub const KYBER512_SK_SIZE: usize = 1632;
    /// Kyber-512 ciphertext size
    pub const KYBER512_CT_SIZE: usize = 768;
    /// Kyber-512 shared secret size
    pub const KYBER512_SS_SIZE: usize = 32;

    /// Kyber-768 public key size
    pub const KYBER768_PK_SIZE: usize = 1184;
    /// Kyber-768 secret key size
    pub const KYBER768_SK_SIZE: usize = 2400;
    /// Kyber-768 ciphertext size
    pub const KYBER768_CT_SIZE: usize = 1088;
    /// Kyber-768 shared secret size
    pub const KYBER768_SS_SIZE: usize = 32;

    /// Kyber-1024 public key size
    pub const KYBER1024_PK_SIZE: usize = 1568;
    /// Kyber-1024 secret key size
    pub const KYBER1024_SK_SIZE: usize = 3168;
    /// Kyber-1024 ciphertext size
    pub const KYBER1024_CT_SIZE: usize = 1568;
    /// Kyber-1024 shared secret size
    pub const KYBER1024_SS_SIZE: usize = 32;
}

/// Dilithium signature sizes by security level
pub mod dilithium_sizes {
    /// Dilithium2 public key size
    pub const DILITHIUM2_PK_SIZE: usize = 1312;
    /// Dilithium2 secret key size
    pub const DILITHIUM2_SK_SIZE: usize = 2528;
    /// Dilithium2 signature size
    pub const DILITHIUM2_SIG_SIZE: usize = 2420;

    /// Dilithium3 public key size
    pub const DILITHIUM3_PK_SIZE: usize = 1952;
    /// Dilithium3 secret key size
    pub const DILITHIUM3_SK_SIZE: usize = 4000;
    /// Dilithium3 signature size
    pub const DILITHIUM3_SIG_SIZE: usize = 3293;

    /// Dilithium5 public key size
    pub const DILITHIUM5_PK_SIZE: usize = 2592;
    /// Dilithium5 secret key size
    pub const DILITHIUM5_SK_SIZE: usize = 4864;
    /// Dilithium5 signature size
    pub const DILITHIUM5_SIG_SIZE: usize = 4595;
}
