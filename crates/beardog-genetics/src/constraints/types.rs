// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core types for self-enforcing key constraints

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Self-enforcing constraints embedded in a BearDog key
///
/// These constraints are cryptographically signed by the key's private key,
/// making them tamper-proof. Any operation must pass constraint verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyConstraints {
    /// What domains/scopes this key can operate in
    pub scope: ScopeConstraint,

    /// Time-based lifetime and evolution rules
    pub lifetime: LifetimeConstraint,

    /// Fine-grained data access rules
    pub data_access: DataAccessConstraint,

    /// Keys that must co-sign certain operations
    pub co_signers: Vec<String>,

    /// Behavioral requirements (biometrics, etc.)
    pub behavioral: BehavioralConstraint,
}

impl Default for KeyConstraints {
    fn default() -> Self {
        Self {
            scope: ScopeConstraint::Unrestricted,
            lifetime: LifetimeConstraint::Permanent,
            data_access: DataAccessConstraint::default(),
            co_signers: Vec::new(),
            behavioral: BehavioralConstraint::default(),
        }
    }
}

/// Scope constraint - what domains can this key operate in?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScopeConstraint {
    /// No scope restrictions
    Unrestricted,

    /// Limited to specific domains
    Limited {
        /// Allowed domain identifiers (e.g., "climate_modeling", "medical_research")
        domains: Vec<String>,
    },

    /// Explicitly forbidden from certain domains
    Forbidden {
        /// Forbidden domain identifiers
        domains: Vec<String>,
    },

    /// Only allowed for specific operations
    OperationSpecific {
        /// Allowed operation types
        allowed_operations: HashSet<OperationType>,
    },
}

/// Types of operations a key can perform
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationType {
    /// Cryptographic signing with the constrained key.
    Sign,
    /// Decrypt ciphertext or sealed payloads.
    Decrypt,
    /// Encrypt data for others or for storage.
    Encrypt,
    /// Irreversible removal of protected material.
    Delete,
    /// Mutate existing protected content.
    Modify,
    /// Read-only access to protected resources.
    Read,
    /// Delegate capabilities or sub-keys within policy.
    Delegate,
    /// Mix or fuse entropy or key material under policy.
    Mix,
}

/// Lifetime constraint - when does this key expire or evolve?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LifetimeConstraint {
    /// Key never expires
    Permanent,

    /// Expires at specific time
    ExpiresAt {
        /// Expiration timestamp
        timestamp: DateTime<Utc>,
    },

    /// Expires after duration from creation
    Duration {
        /// Months until expiry
        months: u32,

        /// Trigger evolution at this many months (before expiry)
        evolution_trigger: Option<u32>,
    },

    /// Expires after number of uses
    UseCount {
        /// Maximum uses allowed
        max_uses: u64,

        /// Current use count (mutable)
        current_uses: u64,
    },
}

/// Data access constraint - fine-grained control over data operations
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataAccessConstraint {
    /// Path patterns that cannot be deleted
    pub cannot_delete: Vec<String>,

    /// Path patterns that cannot be modified
    pub cannot_modify: Vec<String>,

    /// Path patterns that can only be read
    pub read_only: Vec<String>,

    /// All encrypted data must be encrypted to these key IDs
    /// (ensures multi-party access to sensitive data)
    pub must_encrypt_to: Vec<String>,

    /// Immutable paths - once written, cannot be changed
    pub immutable_paths: Vec<String>,
}

impl DataAccessConstraint {
    /// Check if a path matches any pattern in the list
    pub fn path_matches(path: &str, patterns: &[String]) -> bool {
        patterns.iter().any(|pattern| {
            // Simple glob matching (supports * wildcard)
            if pattern.ends_with("/*") {
                let prefix = &pattern[..pattern.len() - 2];
                path.starts_with(prefix)
            } else {
                path == pattern
            }
        })
    }
}

/// Behavioral constraint - additional verification requirements
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehavioralConstraint {
    /// Requires biometric verification for sensitive operations
    pub requires_biometric: bool,

    /// Requires multi-factor authentication
    pub requires_mfa: bool,

    /// Minimum time between operations (rate limiting)
    pub min_operation_interval_secs: Option<u64>,

    /// Requires specific entropy quality level
    pub min_entropy_quality: Option<EntropyQuality>,
}

/// Entropy quality levels from the tunnel
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntropyQuality {
    /// Software PRNG (lowest)
    Software = 0,

    /// Hardware RNG
    Hardware = 1,

    /// TPM/Secure Enclave
    SecureHardware = 2,

    /// HSM (highest)
    Hsm = 3,
}

/// Signed constraints - cryptographically tamper-proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedConstraints {
    /// The actual constraints
    pub constraints: KeyConstraints,

    /// Ed25519 signature of constraints hash (signed by key's private key)
    pub signature: Vec<u8>,

    /// Key ID that signed these constraints
    pub signed_by_key_id: String,

    /// When these constraints were created
    pub created_at: DateTime<Utc>,

    /// Version of constraint format (for future upgrades)
    pub version: u32,
}

impl SignedConstraints {
    /// Current version of constraint format
    pub const CURRENT_VERSION: u32 = 1;

    /// Compute hash of constraints for signing
    pub fn compute_hash(constraints: &KeyConstraints) -> Vec<u8> {
        use blake3::Hasher;

        // Serialization is deterministic and should never fail for valid types
        let serialized = bincode::serialize(constraints).unwrap_or_default();

        Hasher::new()
            .update(&serialized)
            .finalize()
            .as_bytes()
            .to_vec()
    }
}

/// Operations that can be performed with a key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyOperation {
    /// Sign data
    Sign {
        /// Domain context
        domain: Option<String>,
    },

    /// Decrypt data
    Decrypt {
        /// Domain context
        domain: Option<String>,
    },

    /// Encrypt data
    Encrypt {
        /// Who should be able to decrypt
        recipients: Vec<String>,
    },

    /// Delete data at path
    Delete {
        /// File/data path
        path: String,
    },

    /// Modify data at path
    Modify {
        /// File/data path
        path: String,
    },

    /// Read data at path
    Read {
        /// File/data path
        path: String,
    },

    /// Delegate authority to another key
    Delegate {
        /// Target key ID
        to_key_id: String,
        /// What authority to delegate
        authority: Box<Self>,
    },

    /// Mix with other keys
    Mix {
        /// Other key IDs
        with_key_ids: Vec<String>,
    },
}

impl KeyOperation {
    /// Get the operation type
    pub const fn operation_type(&self) -> OperationType {
        match self {
            Self::Sign { .. } => OperationType::Sign,
            Self::Decrypt { .. } => OperationType::Decrypt,
            Self::Encrypt { .. } => OperationType::Encrypt,
            Self::Delete { .. } => OperationType::Delete,
            Self::Modify { .. } => OperationType::Modify,
            Self::Read { .. } => OperationType::Read,
            Self::Delegate { .. } => OperationType::Delegate,
            Self::Mix { .. } => OperationType::Mix,
        }
    }

    /// Get domain if applicable
    pub fn domain(&self) -> Option<&str> {
        match self {
            Self::Sign { domain } | Self::Decrypt { domain } => domain.as_deref(),
            _ => None,
        }
    }

    /// Get path if applicable
    pub fn path(&self) -> Option<&str> {
        match self {
            Self::Delete { path } | Self::Modify { path } | Self::Read { path } => Some(path),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_matching() {
        let patterns = vec!["raw_data/*".to_string(), "secrets/api_key.txt".to_string()];

        assert!(DataAccessConstraint::path_matches(
            "raw_data/file.txt",
            &patterns
        ));
        assert!(DataAccessConstraint::path_matches(
            "raw_data/subdir/file.txt",
            &patterns
        ));
        assert!(DataAccessConstraint::path_matches(
            "secrets/api_key.txt",
            &patterns
        ));
        assert!(!DataAccessConstraint::path_matches(
            "public/data.txt",
            &patterns
        ));
    }

    #[test]
    fn test_operation_type() {
        let op = KeyOperation::Delete {
            path: "test.txt".to_string(),
        };
        assert_eq!(op.operation_type(), OperationType::Delete);
        assert_eq!(op.path(), Some("test.txt"));
    }

    #[test]
    fn test_constraint_hash() {
        let constraints = KeyConstraints::default();
        let hash1 = SignedConstraints::compute_hash(&constraints);
        let hash2 = SignedConstraints::compute_hash(&constraints);
        assert_eq!(hash1, hash2); // Deterministic hashing

        let different = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["test".to_string()],
            },
            ..Default::default()
        };
        let hash3 = SignedConstraints::compute_hash(&different);
        assert_ne!(hash1, hash3); // Different constraints = different hash
    }
}
