// SPDX-License-Identifier: AGPL-3.0-only

use crate::constants::time;

// HSM key management types and structures
// Provides definitions for HSM keys, metadata, and lifecycle management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// HSM key representation
/// `HsmKey`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    /// Unique key identifier
    pub key_id: String,
    /// Key algorithm type
    /// The algorithm value
    pub algorithm: String,
    /// Key size in bits
    /// Number of `key_size`
    pub key_size: u32,
    /// Key usage permissions
    /// Collection of usage
    pub usage: Vec<KeyUsage>,
    /// Key material (may be encrypted or reference)
    /// The material value
    pub material: KeyMaterial,
    /// Key metadata
    /// The metadata value
    pub metadata: KeyMetadata,
    /// Key health status
    /// The health value
    pub health: KeyHealth,
    /// Creation timestamp
    /// The created at value
    pub created_at: SystemTime,
    /// Last access timestamp
    /// Optional last accessed
    pub last_accessed: Option<SystemTime>,
    /// Key expiration time
    /// Optional expires at
    pub expires_at: Option<SystemTime>,
}

impl Default for HsmKey {
    fn default() -> Self {
        Self {
            key_id: uuid::Uuid::new_v4().to_string(),
            algorithm: "AES".to_string(),
            key_size: 256,
            usage: vec![KeyUsage::Encrypt, KeyUsage::Decrypt],
            material: KeyMaterial::default(),
            metadata: KeyMetadata::default(),
            health: KeyHealth::default(),
            created_at: SystemTime::now(),
            last_accessed: None,
            expires_at: None,
        }
    }
}

/// Key usage types
/// `KeyUsage`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyUsage {
    /// Encryption operations
    /// Encrypt
    Encrypt,
    /// Decryption operations
    /// Decrypt
    Decrypt,
    /// Digital signing
    /// Sign
    Sign,
    /// Signature verification
    /// Verify
    Verify,
    /// Key derivation
    /// Derive
    Derive,
    /// Key wrapping
    /// Wrap
    Wrap,
    /// Key unwrapping
    /// Unwrap
    Unwrap,
    /// Key agreement
    /// `KeyAgreement`
    KeyAgreement,
}

/// Key material storage
/// `KeyMaterial`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMaterial {
    /// Storage type (hardware, software, reference)
    /// The storage type value
    pub storage_type: StorageType,
    /// Encrypted key data (if stored in software)
    /// Collection of key data
    pub key_data: Vec<u8>,
    /// HSM handle or reference (if stored in hardware)
    /// Optional hsm handle
    pub hsm_handle: Option<String>,
    /// Optional encryption info
    pub encryption_info: Option<EncryptionInfo>,
    /// Optional backup info
    pub backup_info: Option<BackupInfo>,
}

impl Default for KeyMaterial {
    fn default() -> Self {
        Self {
            storage_type: StorageType::Software,
            key_data: Vec::new(),
            hsm_handle: None,
            encryption_info: None,
            backup_info: None,
        }
    }
}

/// Key storage types
/// `StorageType`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of storage
pub enum StorageType {
    /// Software storage (encrypted)
    /// Software
    Software,
    /// Hardware security module
    /// Hardware
    Hardware,
    /// Reference to external key
    /// Reference
    Reference,
    /// Distributed storage
    /// Distributed
    Distributed,
}

/// Key metadata
/// `KeyMetadata`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key name or label
    /// Name of the item
    pub name: String,
    /// Key description
    /// Optional description
    pub description: Option<String>,
    /// Key owner
    /// The owner value
    pub owner: String,
    /// Collection of tags
    pub tags: Vec<String>,
    /// Custom attributes
    /// Mapping of attributes
    pub attributes: HashMap<String, String>,
    /// Key version
    /// Number of version
    pub version: u32,
    /// Optional lineage link to a wrapping or predecessor key in hierarchical schemes.
    pub parent_key_id: Option<String>,
}

impl Default for KeyMetadata {
    fn default() -> Self {
        Self {
            name: "Unnamed Key".to_string(),
            description: None,
            owner: "system".to_string(),
            tags: Vec::new(),
            attributes: HashMap::new(),
            version: 1,
            parent_key_id: None,
        }
    }
}

/// Key health status
/// `KeyHealth`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyHealth {
    /// Health status
    /// Current status of the component
    pub status: String,
    /// Last health check
    /// The last check value
    pub last_check: SystemTime,
    /// Health check interval in seconds
    /// Number of `check_interval`
    pub check_interval: u32,
    /// Number of operation
    pub operation_count: u64,
    /// Error count
    /// Number of error
    pub error_count: u32,
}

impl Default for KeyHealth {
    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            last_check: SystemTime::now(),
            check_interval: time::SECONDS_PER_HOUR as u32, // 1 hour
            operation_count: 0,
            error_count: 0,
        }
    }
}

/// `EncryptionInfo`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionInfo {
    /// Encryption algorithm used
    /// The algorithm value
    pub algorithm: String,
    /// Encryption mode
    /// The mode value
    pub mode: String,
    /// Padding scheme
    /// The padding value
    pub padding: String,
    /// Initialization vector
    /// Collection of iv
    pub iv: Vec<u8>,
    /// Optional auth tag
    pub auth_tag: Option<Vec<u8>>,
}

impl Default for EncryptionInfo {
    fn default() -> Self {
        Self {
            algorithm: "AES-256".to_string(),
            mode: "GCM".to_string(),
            padding: "None".to_string(),
            iv: Vec::new(),
            auth_tag: None,
        }
    }
}

/// `BackupInfo`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Backup enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Backup location
    /// The backup location value
    pub backup_location: String,
    /// Last backup time
    /// Optional last backup
    pub last_backup: Option<SystemTime>,
    /// Backup encryption key ID
    pub backup_key_id: Option<String>,
    /// Backup verification hash
    /// Optional verification hash
    pub verification_hash: Option<String>,
}

impl Default for BackupInfo {
    fn default() -> Self {
        Self {
            enabled: true,
            backup_location: "local".to_string(),
            last_backup: None,
            backup_key_id: None,
            verification_hash: None,
        }
    }
}

/// Key lifecycle states
/// `KeyLifecycleState`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyLifecycleState {
    /// Key is being generated
    /// Generating
    Generating,
    /// Key is active and can be used
    /// Active
    #[default]
    Active,
    /// Key is suspended (temporarily disabled)
    /// Suspended
    Suspended,
    /// Key is compromised
    /// Compromised
    Compromised,
    /// Key is expired
    /// Expired
    Expired,
    /// Key is destroyed
    /// Destroyed
    Destroyed,
}

/// Key operation request
/// `KeyOperationRequest`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOperationRequest {
    /// Key ID to operate on
    pub key_id: String,
    /// Operation type
    /// The operation value
    pub operation: KeyOperation,
    /// Input data
    /// Collection of input data
    pub input_data: Vec<u8>,
    /// Additional parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
    /// Idempotency / tracing token echoed in HSM audit logs.
    pub request_id: String,
}

/// Key operations
/// `KeyOperation`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyOperation {
    /// Encrypt data
    /// Encrypt
    Encrypt,
    /// Decrypt data
    /// Decrypt
    Decrypt,
    /// Sign data
    /// Sign
    Sign,
    /// Verify signature
    /// Verify
    Verify,
    /// Derive key
    /// Derive
    Derive,
    /// Wrap key
    /// Wrap
    Wrap,
    /// Unwrap key
    /// Unwrap
    Unwrap,
}

/// Key operation response
/// `KeyOperationResponse`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOperationResponse {
    /// Request ID
    pub request_id: String,
    /// Operation result
    /// The result value
    pub result: OperationResult,
    /// Output data
    /// Collection of output data
    pub output_data: Vec<u8>,
    /// Operation metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Operation result status
/// `OperationResult`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationResult {
    /// Operation succeeded
    /// Success
    Success,
    /// Operation failed
    Failed {
        /// Error message describing the failure
        error: String,
    },
    /// Operation is pending
    /// Pending
    Pending,
    /// Operation was cancelled
    /// Cancelled
    Cancelled,
}

impl HsmKey {
    /// Create a new HSM key with specified parameters
    #[must_use]
    /// Creates a new instance
    pub fn new(key_id: String, algorithm: String, key_size: u32) -> Self {
        Self {
            key_id,
            algorithm,
            key_size,
            ..Default::default()
        }
    }

    /// Check if the key is active and can be used
    #[must_use]
    /// Checks if active
    /// Checks if active
    pub fn is_active(&self) -> bool {
        self.health.status == "healthy" && self.expires_at.is_none_or(|exp| exp > SystemTime::now())
    }

    /// Check if the key supports a specific usage
    #[must_use]
    pub fn supports_usage(&self, usage: &KeyUsage) -> bool {
        self.usage.contains(usage)
    }

    /// Update the last accessed timestamp
    pub fn mark_accessed(&mut self) {
        self.last_accessed = Some(SystemTime::now());
        self.health.operation_count += 1;
    }

    /// Add a tag to the key metadata
    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
        }
    }

    /// Remove a tag from the key metadata
    /// Removes tag
    /// Removes tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.metadata.tags.retain(|t| t != tag);
    }

    /// Set a custom attribute
    /// Sets attribute
    /// Sets attribute
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.metadata.attributes.insert(key, value);
    }

    /// Get a custom attribute
    #[must_use]
    /// Gets attribute
    /// Gets attribute
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.metadata.attributes.get(key)
    }

    /// Check if the key is expired
    #[must_use]
    /// Checks if expired
    /// Checks if expired
    pub fn is_expired(&self) -> bool {
        self.expires_at.is_some_and(|exp| exp <= SystemTime::now())
    }

    /// Get the key age in seconds
    #[must_use]
    pub fn age_seconds(&self) -> u64 {
        SystemTime::now()
            .duration_since(self.created_at)
            .unwrap_or_default()
            .as_secs()
    }

    /// Update key health status
    /// Updates health
    /// Updates health
    pub fn update_health(&mut self, status: String) {
        self.health.status = status;
        self.health.last_check = SystemTime::now();
    }

    /// Increment error count
    pub fn record_error(&mut self) {
        self.health.error_count += 1;
        if self.health.error_count > 10 {
            self.health.status = "degraded".to_string();
        }
    }

    /// Check if key needs health check
    #[must_use]
    pub fn needs_health_check(&self) -> bool {
        SystemTime::now()
            .duration_since(self.health.last_check)
            .unwrap_or_default()
            .as_secs()
            > u64::from(self.health.check_interval)
    }
}

/// Key management utilities
pub struct KeyManager;

impl KeyManager {
    /// Generate a new key ID
    #[must_use]
    pub fn generate_key_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    #[must_use]
    /// Validates `key_id`
    /// Validates `key_id`
    pub const fn validate_key_id(key_id: &str) -> bool {
        !key_id.is_empty() && key_id.len() <= 255
    }

    /// Check if algorithm is supported
    #[must_use]
    /// Checks if algorithm supported
    /// Checks if algorithm supported
    pub fn is_algorithm_supported(algorithm: &str) -> bool {
        matches!(
            algorithm,
            "AES" | "RSA" | "ECC" | "HMAC" | "ChaCha20" | "Ed25519"
        )
    }

    /// Returns whether `key_size` is a conventional length for `algorithm` (AES/RSA/ECC/etc.).
    #[must_use]
    pub fn is_key_size_valid(algorithm: &str, key_size: u32) -> bool {
        match algorithm {
            "AES" => matches!(key_size, 128 | 192 | 256),
            "RSA" => (1024..=4096).contains(&key_size),
            "ECC" => matches!(key_size, 256 | 384 | 521),
            "HMAC" => key_size >= 128,
            "ChaCha20" | "Ed25519" => key_size == 256,
            _ => false,
        }
    }

    /// Conservative default key size in bits for `algorithm`, if one exists.
    #[must_use]
    pub fn recommended_key_size(algorithm: &str) -> Option<u32> {
        match algorithm {
            "AES" | "ECC" | "HMAC" | "ChaCha20" | "Ed25519" => Some(256),
            "RSA" => Some(2048),
            _ => None,
        }
    }

    #[expect(
        dead_code,
        reason = "Reserved helper for default key sizes; not yet wired into public API"
    )]
    /// Gets `default_key_size`
    fn get_default_key_size(algorithm: &str) -> Option<usize> {
        match algorithm {
            "RSA" => Some(2048),
            "AES" | "ECC" | "HMAC" | "ChaCha20" | "Ed25519" => Some(256),
            _ => None,
        }
    }
}
