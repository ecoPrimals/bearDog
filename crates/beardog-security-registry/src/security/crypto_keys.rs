//! Cryptographic key management for security registry

use serde::{Deserialize, Serialize};

/// Cryptographic key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Used in future implementation
pub struct CryptoKeyMetadata {
    /// Key identifier
    pub key_id: String,
    /// Key algorithm
    pub algorithm: String,
    /// Key creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)] // Used in future implementation
impl CryptoKeyMetadata {
    /// Creates new key metadata
    pub fn new(key_id: String, algorithm: String) -> Self {
        Self {
            key_id,
            algorithm,
            created_at: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_key_metadata_new() {
        let metadata = CryptoKeyMetadata::new("key-123".to_string(), "AES-256-GCM".to_string());
        assert_eq!(metadata.key_id, "key-123");
        assert_eq!(metadata.algorithm, "AES-256-GCM");
    }

    #[test]
    fn test_crypto_key_metadata_clone() {
        let metadata = CryptoKeyMetadata::new("key-456".to_string(), "RSA-4096".to_string());
        let cloned = metadata.clone();
        assert_eq!(metadata.key_id, cloned.key_id);
        assert_eq!(metadata.algorithm, cloned.algorithm);
    }

    #[test]
    fn test_crypto_key_metadata_serialization() {
        let metadata = CryptoKeyMetadata::new("key-789".to_string(), "Ed25519".to_string());
        let serialized = serde_json::to_string(&metadata).expect("serialize");
        assert!(serialized.contains("key-789"));
        assert!(serialized.contains("Ed25519"));

        let deserialized: CryptoKeyMetadata =
            serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(metadata.key_id, deserialized.key_id);
        assert_eq!(metadata.algorithm, deserialized.algorithm);
    }

    #[test]
    fn test_crypto_key_metadata_timestamp() {
        let before = chrono::Utc::now();
        let metadata = CryptoKeyMetadata::new("key-ts".to_string(), "ChaCha20".to_string());
        let after = chrono::Utc::now();

        assert!(metadata.created_at >= before);
        assert!(metadata.created_at <= after);
    }

    #[test]
    fn test_crypto_key_metadata_debug() {
        let metadata = CryptoKeyMetadata::new("debug-key".to_string(), "ECDSA".to_string());
        let debug_str = format!("{:?}", metadata);
        assert!(debug_str.contains("CryptoKeyMetadata"));
        assert!(debug_str.contains("debug-key"));
    }
}
