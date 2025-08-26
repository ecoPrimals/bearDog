

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyType {

    Symmetric {
        algorithm: String,
        key_size: u32,
    },

    Asymmetric {
        algorithm: String,
        key_size: u32,
    },

    Signing {
        algorithm: String,
        key_size: u32,
    },

    KeyAgreement {
        algorithm: String,
        key_size: u32,
    },

    Authentication {
        algorithm: String,
        key_size: u32,
    },

    Aes128,
    Aes192,
    Aes256,

    Rsa1024,
    Rsa2048,
    Rsa3072,
    Rsa4096,

    EccP256,
    EccP384,
    EccP521,

    Ed25519,
    Ed448,

    X25519,
    X448,

    Secp256k1,

    ChaCha20,

    PostQuantum {
        algorithm: String,
        parameter_set: String,
    },

    SymmetricLegacy,

    AsymmetricLegacy,

    SigningLegacy,

    EncryptionLegacy,

    AuthenticationLegacy,

    DerivationLegacy,

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
