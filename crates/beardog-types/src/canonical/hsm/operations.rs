use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical HSM operation types for unified operation handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmOperation {
    /// Key generation operation
    KeyGeneration {
        key_type: String,
        key_size: u32,
        usage_policy: Option<String>,
    },
    /// Data encryption operation
    Encryption {
        key_id: String,
        algorithm: String,
        data_size: Option<usize>,
    },
    /// Data decryption operation
    Decryption {
        key_id: String,
        algorithm: String,
        data_size: Option<usize>,
    },
    /// Digital signing operation
    Signing {
        key_id: String,
        algorithm: String,
        digest_type: Option<String>,
    },
    /// Signature verification operation
    Verification {
        key_id: String,
        algorithm: String,
        digest_type: Option<String>,
    },
    /// Key deletion operation
    KeyDeletion { key_id: String },
    /// Key derivation operation
    KeyDerivation {
        parent_key_id: String,
        derivation_path: String,
        derived_key_type: String,
    },
    /// Random number generation
    RandomGeneration {
        length: usize,
        entropy_source: Option<String>,
    },
    /// Health check operation
    HealthCheck,
    /// Attestation operation
    Attestation { challenge: Vec<u8> },
}

/// Result of an HSM operation with comprehensive metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmOperationResult {
    /// The operation that was executed
    pub operation: HsmOperation,
    /// Whether the operation succeeded
    pub success: bool,
    /// Result data if successful
    pub result_data: Option<Vec<u8>>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Additional metadata about the operation
    pub metadata: HashMap<String, String>,
}

impl HsmOperation {
    /// Get the operation type as a string for logging and metrics
    #[must_use]
    pub const fn operation_type(&self) -> &'static str {
        match self {
            Self::KeyGeneration { .. } => "key_generation",
            Self::Encryption { .. } => "encryption",
            Self::Decryption { .. } => "decryption",
            Self::Signing { .. } => "signing",
            Self::Verification { .. } => "verification",
            Self::KeyDeletion { .. } => "key_deletion",
            Self::KeyDerivation { .. } => "key_derivation",
            Self::RandomGeneration { .. } => "random_generation",
            Self::HealthCheck => "health_check",
            Self::Attestation { .. } => "attestation",
        }
    }

    /// Check if this operation requires a specific key
    #[must_use]
    pub const fn requires_key(&self) -> bool {
        !matches!(
            self,
            Self::KeyGeneration { .. } | Self::RandomGeneration { .. } | Self::HealthCheck
        )
    }

    /// Get the key ID if this operation uses one
    #[must_use]
    pub fn key_id(&self) -> Option<&str> {
        match self {
            Self::Encryption { key_id, .. }
            | Self::Decryption { key_id, .. }
            | Self::Signing { key_id, .. }
            | Self::Verification { key_id, .. }
            | Self::KeyDeletion { key_id } => Some(key_id),
            Self::KeyDerivation { parent_key_id, .. } => Some(parent_key_id),
            _ => None,
        }
    }
}
