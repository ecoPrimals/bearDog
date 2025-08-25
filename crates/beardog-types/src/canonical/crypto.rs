// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Cryptographic Types
///
/// **SINGLE SOURCE OF TRUTH** for all cryptographic types in the `BearDog` ecosystem.
/// This module consolidates key types, encryption parameters, and crypto operations.
use serde::{Deserialize, Serialize};

/// **UNIVERSAL CANONICAL** Key Type - Agnostic cryptographic key classification
/// This enum supports all known cryptographic key types across:
/// - All major HSM vendors (Thales, SafeNet, Utimaco, etc.)
/// - Cloud HSM services (AWS, Azure, GCP)
/// - Mobile secure elements (iOS, Android)
/// - Software HSM implementations
/// - Cryptocurrency and blockchain systems
/// - Government and military cryptographic standards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyType {
    // === UNIVERSAL ALGORITHM FAMILIES ===
    /// Symmetric encryption keys (AES, ChaCha20, etc.)
    Symmetric {
        algorithm: String,
        key_size: u32,
    },
    /// Asymmetric encryption keys (RSA, ECC, etc.)
    Asymmetric {
        algorithm: String,
        key_size: u32,
    },
    /// Digital signature keys (RSA-PSS, ECDSA, EdDSA, etc.)
    Signing {
        algorithm: String,
        key_size: u32,
    },
    /// Key derivation and agreement keys (ECDH, X25519, etc.)
    KeyAgreement {
        algorithm: String,
        key_size: u32,
    },
    /// Authentication keys (HMAC, etc.)
    Authentication {
        algorithm: String,
        key_size: u32,
    },
    
    // === SPECIFIC WELL-KNOWN ALGORITHMS (for performance and compatibility) ===
    /// AES symmetric encryption
    Aes128,
    Aes192,
    Aes256,
    /// RSA asymmetric encryption/signing
    Rsa1024,
    Rsa2048,
    Rsa3072,
    Rsa4096,
    /// Elliptic Curve keys
    EccP256,
    EccP384,
    EccP521,
    /// Edwards curve keys
    Ed25519,
    Ed448,
    /// Curve25519 key exchange
    X25519,
    X448,
    /// Bitcoin/cryptocurrency curves
    Secp256k1,
    /// ChaCha20 stream cipher
    ChaCha20,
    /// Post-quantum cryptography (future-proofing)
    PostQuantum {
        algorithm: String,
        parameter_set: String,
    },
    
    // === LEGACY AND COMPATIBILITY SUPPORT ===
    /// Generic symmetric (for legacy systems)
    SymmetricLegacy,
    /// Generic asymmetric (for legacy systems)  
    AsymmetricLegacy,
    /// Generic signing (for legacy systems)
    SigningLegacy,
    /// Generic encryption (for legacy systems)
    EncryptionLegacy,
    /// Generic authentication (for legacy systems)
    AuthenticationLegacy,
    /// Generic derivation (for legacy systems)
    DerivationLegacy,
    // === VENDOR-SPECIFIC EXTENSIONS ===
    /// Custom vendor-specific key type
    Custom {
        vendor: String,
        parameters: std::collections::HashMap<String, String>,
    },
}

impl Default for KeyType {
    fn default() -> Self {
        Self::Aes256 // Most commonly used secure symmetric key
    }
}

/// **CANONICAL** Encryption Algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    AES256GCM,
    AES128GCM,
    ChaCha20Poly1305,
    RSA2048,
    RSA4096,
    ECDSA256,
    ECDSA384,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::AES256GCM
    }
}

/// **CANONICAL** Key Usage Flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsage {
    pub encrypt: bool,
    pub decrypt: bool,
    pub sign: bool,
    pub verify: bool,
    pub key_agreement: bool,
    pub key_derivation: bool,
    pub wrap_key: bool,
    pub unwrap_key: bool,
}

impl Default for KeyUsage {
    fn default() -> Self {
        Self {
            encrypt: true,
            decrypt: true,
            sign: false,
            verify: false,
            key_agreement: false,
            key_derivation: false,
            wrap_key: false,
            unwrap_key: false,
        }
    }
}

/// **CANONICAL** Cryptographic Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoParams {
    pub algorithm: EncryptionAlgorithm,
    pub key_size_bits: u32,
    pub iv_size_bytes: Option<u32>,
    pub tag_size_bytes: Option<u32>,
    pub additional_data: Option<Vec<u8>>,
}

impl Default for CryptoParams {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::AES256GCM,
            key_size_bits: 256,
            iv_size_bytes: Some(12),  // Standard GCM IV size
            tag_size_bytes: Some(16), // Standard GCM tag size
            additional_data: None,
        }
    }
}
