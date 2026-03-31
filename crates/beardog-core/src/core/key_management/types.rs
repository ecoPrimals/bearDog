// SPDX-License-Identifier: AGPL-3.0-only

//! Key types and metadata for cryptographic key management.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Cryptographic key type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    /// Ed25519 signing key
    #[default]
    Ed25519,
    /// AES-256-GCM encryption key
    Aes256Gcm,
    /// X25519 key exchange key
    X25519,
    /// RSA-2048 key
    Rsa2048,
    /// RSA-4096 key
    Rsa4096,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ed25519 => "Ed25519",
            Self::Aes256Gcm => "AES-256-GCM",
            Self::X25519 => "X25519",
            Self::Rsa2048 => "RSA-2048",
            Self::Rsa4096 => "RSA-4096",
        };
        f.write_str(s)
    }
}

/// Key usage flags
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyUsage {
    /// Key can be used for signing
    #[default]
    Sign,
    /// Key can be used for verification
    Verify,
    /// Key can be used for encryption
    Encrypt,
    /// Key can be used for decryption
    Decrypt,
    /// Key can be used for key agreement
    KeyAgreement,
}

impl fmt::Display for KeyUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Sign => "sign",
            Self::Verify => "verify",
            Self::Encrypt => "encrypt",
            Self::Decrypt => "decrypt",
            Self::KeyAgreement => "key_agreement",
        };
        f.write_str(s)
    }
}

/// Key storage location
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyStorage {
    /// Ephemeral key (in-memory only)
    #[default]
    Ephemeral,
    /// File-based storage
    File(PathBuf),
    /// Hardware Security Module
    Hsm(String), // HSM slot/token identifier
}

impl fmt::Display for KeyStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ephemeral => write!(f, "ephemeral"),
            Self::File(p) => write!(f, "file:{}", p.display()),
            Self::Hsm(id) => write!(f, "hsm:{id}"),
        }
    }
}

/// Key metadata for cryptographic key management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Unique identifier for this cryptographic key
    pub key_id: String,
    /// Type of cryptographic key (symmetric, asymmetric, etc.)
    pub key_type: KeyType,
    /// Permitted usages for this key (encrypt, sign, etc.)
    pub usage: Vec<KeyUsage>,
    /// Storage location/method for this key (HSM, software, etc.)
    pub storage: KeyStorage,
    /// Timestamp when this key was created (UTC)
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional expiration timestamp for key rotation (UTC)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Optional human-readable description of key purpose
    pub description: Option<String>,
}
