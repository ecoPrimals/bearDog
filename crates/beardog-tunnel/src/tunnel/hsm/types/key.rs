// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Key Types
//!
//! Type definitions for cryptographic keys managed by the HSM.
//!
//! # Migration Note
//!
//! The vendor-specific `KeyType` enum defined here is **DEPRECATED** and being
//! migrated to the canonical vendor-agnostic `KeyType` from `beardog-types::canonical`.
//!
//! **For new code**: Use `CanonicalKeyType` (re-exported below)
//! **For old code**: This type remains for compatibility during migration

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ✅ CANONICAL VENDOR-AGNOSTIC KeyType - USE THIS FOR NEW CODE
pub use beardog_types::canonical::providers_unified::traits::security_traits::KeyType as CanonicalKeyType;

// Re-export canonical as primary KeyType (for migration)
pub use beardog_types::canonical::providers_unified::traits::security_traits::KeyType;

// LegacyKeyType removed Nov 7, 2025 - migration complete
// All code now uses CanonicalKeyType (re-exported as KeyType above)

/// Universal key structure
#[derive(Debug, Clone)]
pub struct UniversalKey {
    /// Key ID
    pub id: String,
    /// HSM type
    pub hsm_type: String,
    /// Key type
    pub key_type: KeyType,
    /// Key metadata
    pub metadata: KeyMetadata,
    /// Key material (encrypted or reference)
    pub key_material: KeyMaterial,
    /// HSM tier
    pub hsm_tier: String,
    /// Health status
    pub health_status: KeyHealthStatus,
    /// Optional attestation
    pub attestation: Option<KeyAttestation>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Type alias for compatibility
pub type HsmKey = UniversalKey;

/// Type alias for HSM key metadata (compatibility)
pub type HsmKeyMetadata = KeyMetadata;

/// Key material variants
#[derive(Debug, Clone)]
pub enum KeyMaterial {
    /// Encrypted key data
    Encrypted {
        /// Encrypted key bytes
        encrypted_data: Vec<u8>,
        /// Encryption algorithm used
        encryption_algorithm: String,
        /// Optional KDF parameters
        kdf_params: Option<HashMap<String, String>>,
    },
    /// Reference to HSM-stored key
    Reference {
        /// Key reference identifier
        key_reference: String,
        /// HSM instance identifier
        hsm_instance: String,
    },
    /// Hardware-backed key reference
    HardwareReference {
        /// Hardware reference
        reference: String,
        /// HSM location/identifier
        hsm_location: String,
    },
    /// Key handle for indirect access
    Handle {
        /// Key handle identifier
        key_handle: String,
        /// Handle type
        handle_type: String,
    },
}

impl KeyMaterial {
    /// Get the raw key data (for encrypted variant) or identifier (for references)
    pub fn data(&self) -> &[u8] {
        match self {
            Self::Encrypted { encrypted_data, .. } => encrypted_data,
            Self::Reference { key_reference, .. } => key_reference.as_bytes(),
            Self::HardwareReference { reference, .. } => reference.as_bytes(),
            Self::Handle { key_handle, .. } => key_handle.as_bytes(),
        }
    }
}

/// HSM key information
#[derive(Debug, Clone)]
pub struct HsmKeyInfo {
    /// Key identifier
    pub key_id: String,
    /// Performance metrics
    pub performance_metrics: KeyPerformanceMetrics,
    /// Last access timestamp
    pub last_accessed: Option<DateTime<Utc>>,
    /// Access count
    pub access_count: u64,
    /// Usage policy
    pub usage_policy: KeyUsagePolicy,
}

/// Key performance metrics
#[derive(Debug, Clone)]
pub struct KeyPerformanceMetrics {
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Operations per second
    pub ops_per_second: f64,
    /// Error rate (0.0-1.0)
    pub error_rate: f64,
    /// Total operations performed
    pub total_operations: u64,
}

impl Default for KeyPerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_latency_ms: 0.0,
            ops_per_second: 0.0,
            error_rate: 0.0,
            total_operations: 0,
        }
    }
}

/// Key attestation information
#[derive(Debug, Clone)]
pub struct KeyAttestation {
    /// Certificate chain
    pub certificate_chain: Vec<Vec<u8>>,
    /// Attestation statement
    pub attestation_statement: Vec<u8>,
    /// Attestation format
    pub format: String,
    /// Attestation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Key metadata
#[derive(Debug, Clone)]
pub struct KeyMetadata {
    /// Key identifier
    pub key_id: String,
    /// Key type
    pub key_type: KeyType,
    /// Key alias/name
    pub alias: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiration timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// Custom metadata tags
    pub tags: HashMap<String, String>,
}

impl KeyMetadata {
    /// Create new key metadata
    pub fn new(key_id: String, key_type: KeyType) -> Self {
        Self {
            key_id,
            key_type,
            alias: None,
            created_at: Utc::now(),
            expires_at: None,
            tags: HashMap::new(),
        }
    }
}

/// Key health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyHealthStatus {
    /// Key is healthy
    Healthy,
    /// Key is degraded
    Degraded,
    /// Key is compromised
    Compromised,
    /// Key is expired
    Expired,
    /// Key is revoked
    Revoked,
}

/// Key usage policy
#[derive(Debug, Clone)]
pub struct KeyUsagePolicy {
    /// Allowed operations
    pub allowed_operations: Vec<KeyOperation>,
    /// Maximum uses (None = unlimited)
    pub max_uses: Option<u64>,
    /// Usage rate limit (operations per second)
    pub rate_limit: Option<f64>,
    /// Require user authentication
    pub require_auth: bool,
}

impl Default for KeyUsagePolicy {
    fn default() -> Self {
        Self {
            allowed_operations: vec![
                KeyOperation::Encrypt,
                KeyOperation::Decrypt,
                KeyOperation::Sign,
                KeyOperation::Verify,
            ],
            max_uses: None,
            rate_limit: None,
            require_auth: false,
        }
    }
}

/// Key operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyOperation {
    /// Encryption
    Encrypt,
    /// Decryption
    Decrypt,
    /// Signing
    Sign,
    /// Signature verification
    Verify,
    /// Key wrapping
    Wrap,
    /// Key unwrapping
    Unwrap,
    /// Key derivation
    Derive,
    /// Key export
    Export,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_type_variants() {
        // Vendor-agnostic KeyType - no field access
        let aes = KeyType::Aes;
        let rsa = KeyType::Rsa;
        let ed25519 = KeyType::Ed25519;

        match aes {
            KeyType::Aes => {} // Key size stored separately in KeyGenerationSpec
            _ => panic!("Expected AES variant"),
        }

        match rsa {
            KeyType::Rsa => {} // Key size stored separately in KeyGenerationSpec
            _ => panic!("Expected RSA variant"),
        }

        assert_eq!(ed25519, KeyType::Ed25519);
    }

    #[test]
    fn test_key_metadata_creation() {
        let metadata = KeyMetadata::new("test-key".to_string(), KeyType::Ed25519);
        assert_eq!(metadata.key_id, "test-key");
        assert!(metadata.alias.is_none());
        assert!(metadata.expires_at.is_none());
        assert!(metadata.tags.is_empty());
    }

    #[test]
    fn test_key_health_status() {
        let healthy = KeyHealthStatus::Healthy;
        let expired = KeyHealthStatus::Expired;

        assert_eq!(healthy, KeyHealthStatus::Healthy);
        assert_eq!(expired, KeyHealthStatus::Expired);
        assert_ne!(healthy, expired);
    }

    #[test]
    fn test_key_usage_policy_default() {
        let policy = KeyUsagePolicy::default();
        assert_eq!(policy.allowed_operations.len(), 4);
        assert!(policy.allowed_operations.contains(&KeyOperation::Encrypt));
        assert!(policy.allowed_operations.contains(&KeyOperation::Decrypt));
        assert!(policy.max_uses.is_none());
        assert!(!policy.require_auth);
    }

    #[test]
    fn test_key_operations() {
        let ops = [
            KeyOperation::Encrypt,
            KeyOperation::Decrypt,
            KeyOperation::Sign,
            KeyOperation::Verify,
            KeyOperation::Wrap,
            KeyOperation::Unwrap,
            KeyOperation::Derive,
            KeyOperation::Export,
        ];
        assert_eq!(ops.len(), 8);
    }

    #[test]
    fn test_key_material_encrypted() {
        let material = KeyMaterial::Encrypted {
            encrypted_data: vec![1, 2, 3, 4, 5],
            encryption_algorithm: "AES-256-GCM".to_string(),
            kdf_params: None,
        };

        match material {
            KeyMaterial::Encrypted { encrypted_data, .. } => {
                assert_eq!(encrypted_data.len(), 5);
            }
            _ => panic!("Expected Encrypted variant"),
        }
    }

    #[test]
    fn test_key_material_reference() {
        let material = KeyMaterial::Reference {
            key_reference: "key-ref-123".to_string(),
            hsm_instance: "hsm-01".to_string(),
        };

        match material {
            KeyMaterial::Reference {
                key_reference,
                hsm_instance,
            } => {
                assert_eq!(key_reference, "key-ref-123");
                assert_eq!(hsm_instance, "hsm-01");
            }
            _ => panic!("Expected Reference variant"),
        }
    }

    #[test]
    fn test_key_attestation() {
        let attestation = KeyAttestation {
            certificate_chain: vec![vec![1, 2, 3]],
            attestation_statement: vec![4, 5, 6],
            format: "android-safetynet".to_string(),
            timestamp: Utc::now(),
        };

        assert_eq!(attestation.certificate_chain.len(), 1);
        assert_eq!(attestation.format, "android-safetynet");
    }
}
