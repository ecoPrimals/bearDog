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
