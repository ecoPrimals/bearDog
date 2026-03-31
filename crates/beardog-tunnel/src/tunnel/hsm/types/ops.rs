// SPDX-License-Identifier: AGPL-3.0-only

//! Generic HSM operation, audit, and cache types (platform-agnostic).

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use super::key;

/// HSM operation types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HsmOperation {
    /// Key generation
    KeyGeneration {
        /// Unique identifier for the key
        key_id: String,
        /// Key size in bits
        key_size: u32,
    },
    /// Encryption operation
    Encryption {
        /// Key identifier used for encryption
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Decryption operation
    Decryption {
        /// Key identifier used for decryption
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Signing operation
    Signing {
        /// Key identifier used for signing
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Verification operation
    Verification {
        /// Key identifier used for verification
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Key deletion
    KeyDeletion {
        /// Key identifier to delete
        key_id: String,
    },
}

/// HSM operation result
#[derive(Debug, Clone)]
pub struct HsmOperationResult {
    /// The operation performed
    pub operation: HsmOperation,
    /// Whether the operation succeeded
    pub success: bool,
    /// Optional result data
    pub result_data: Option<Vec<u8>>,
    /// Optional error message
    pub error_message: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// HSM audit entry
#[derive(Debug, Clone)]
pub struct HsmAuditEntry {
    /// Unique entry ID
    pub id: String,
    /// Operation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// User ID (optional)
    pub user_id: Option<String>,
    /// Operation result
    pub result: HsmOperationResult,
    /// Security context metadata
    pub security_context: HashMap<String, String>,
}

/// HSM cache for performance optimization
#[derive(Debug, Clone)]
pub struct HsmCache {
    /// Cached key metadata
    pub key_metadata: Arc<RwLock<HashMap<String, key::KeyMetadata>>>,
    /// Cached operation results
    pub operation_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl HsmCache {
    /// Create new HSM cache
    pub fn new() -> Self {
        Self {
            key_metadata: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            operation_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
}

impl Default for HsmCache {
    fn default() -> Self {
        Self::new()
    }
}
