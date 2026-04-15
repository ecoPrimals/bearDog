// SPDX-License-Identifier: AGPL-3.0-or-later

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

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn key_type_display_covers_variants() {
        assert_eq!(format!("{}", KeyType::Ed25519), "Ed25519");
        assert_eq!(format!("{}", KeyType::Aes256Gcm), "AES-256-GCM");
        assert_eq!(format!("{}", KeyType::X25519), "X25519");
        assert_eq!(format!("{}", KeyType::Rsa2048), "RSA-2048");
        assert_eq!(format!("{}", KeyType::Rsa4096), "RSA-4096");
    }

    #[test]
    fn key_usage_display_covers_variants() {
        assert_eq!(format!("{}", KeyUsage::Sign), "sign");
        assert_eq!(format!("{}", KeyUsage::Verify), "verify");
        assert_eq!(format!("{}", KeyUsage::Encrypt), "encrypt");
        assert_eq!(format!("{}", KeyUsage::Decrypt), "decrypt");
        assert_eq!(format!("{}", KeyUsage::KeyAgreement), "key_agreement");
    }

    #[test]
    fn key_storage_display_covers_variants() {
        assert_eq!(format!("{}", KeyStorage::Ephemeral), "ephemeral");
        let p = std::path::PathBuf::from("/tmp/keys");
        assert_eq!(
            format!("{}", KeyStorage::File(p.clone())),
            format!("file:{}", p.display())
        );
        assert_eq!(
            format!("{}", KeyStorage::Hsm("slot7".to_string())),
            "hsm:slot7"
        );
    }

    #[test]
    fn key_metadata_serde_roundtrip() {
        let created = chrono::DateTime::parse_from_rfc3339("2024-01-15T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let expires = chrono::DateTime::parse_from_rfc3339("2025-01-15T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let meta = KeyMetadata {
            key_id: "k1".to_string(),
            key_type: KeyType::Aes256Gcm,
            usage: vec![KeyUsage::Encrypt, KeyUsage::Decrypt],
            storage: KeyStorage::File(std::path::PathBuf::from("/var/bd/keys")),
            created_at: created,
            expires_at: Some(expires),
            description: Some("unit".to_string()),
        };
        let json = serde_json::to_string(&meta).unwrap();
        let back: KeyMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(back.key_id, meta.key_id);
        assert_eq!(back.key_type, meta.key_type);
        assert_eq!(back.usage, meta.usage);
        assert_eq!(back.description, meta.description);
        assert_eq!(back.created_at, meta.created_at);
        assert_eq!(back.expires_at, meta.expires_at);
        match &back.storage {
            KeyStorage::File(pb) => assert!(pb.ends_with("keys")),
            _ => panic!("expected file storage"),
        }
    }
}
