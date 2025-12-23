//! HSM security levels and cryptographic algorithms
//!
//! This module defines the security levels and cryptographic algorithms
//! supported by the unified HSM provider.

use serde::{Deserialize, Serialize};

/// Security levels for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[derive(Default)]
pub enum SecurityLevel {
    /// Software-based security (development/testing)
    #[default]
    Software,
    /// Trusted Execution Environment
    TrustedExecutionEnvironment,
    /// Hardware-backed security
    HardwareBacked,
    /// `StrongBox` security level (highest)
    StrongBox,
}

/// Supported cryptographic algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeyAlgorithm {
    /// RSA algorithm
    Rsa,
    /// Elliptic Curve algorithm
    Ec,
    /// AES symmetric algorithm
    Aes,
    /// HMAC algorithm
    Hmac,
    /// Ed25519 signature algorithm
    Ed25519,
    /// X25519 key agreement algorithm
    X25519,
}

/// Key usage purposes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeyPurpose {
    /// Key for encryption operations
    Encrypt,
    /// Key for decryption operations
    Decrypt,
    /// Key for signing operations
    Sign,
    /// Key for signature verification
    Verify,
    /// Key for key derivation
    DeriveKey,
    /// Key for key agreement protocols
    KeyAgreement,
}

impl SecurityLevel {
    /// Check if this security level is hardware-backed
    pub fn is_hardware_backed(&self) -> bool {
        matches!(self, SecurityLevel::HardwareBacked | SecurityLevel::StrongBox)
    }
    
    /// Get the security strength score (0-100)
    pub fn security_score(&self) -> u8 {
        match self {
            SecurityLevel::Software => 25,
            SecurityLevel::TrustedExecutionEnvironment => 50,
            SecurityLevel::HardwareBacked => 75,
            SecurityLevel::StrongBox => 100,
        }
    }
}

impl KeyAlgorithm {
    /// Check if this is a symmetric algorithm
    pub fn is_symmetric(&self) -> bool {
        matches!(self, KeyAlgorithm::Aes | KeyAlgorithm::Hmac)
    }
    
    /// Check if this is an asymmetric algorithm
    pub fn is_asymmetric(&self) -> bool {
        matches!(
            self,
            KeyAlgorithm::Rsa | KeyAlgorithm::Ec | KeyAlgorithm::Ed25519 | KeyAlgorithm::X25519
        )
    }
    
    /// Get default key size for this algorithm
    pub fn default_key_size(&self) -> usize {
        match self {
            KeyAlgorithm::Rsa => 2048,
            KeyAlgorithm::Ec => 256,
            KeyAlgorithm::Aes => 256,
            KeyAlgorithm::Hmac => 256,
            KeyAlgorithm::Ed25519 => 256,
            KeyAlgorithm::X25519 => 256,
        }
    }
} 