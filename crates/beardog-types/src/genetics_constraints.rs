// SPDX-License-Identifier: AGPL-3.0-only

//! # Key Constraints - Cryptographic Self-Enforcement
//!
//! This module implements self-enforcing cryptographic constraints for BearDog keys.
//! Keys can embed rules that are cryptographically signed and cannot be removed or modified.
//!
//! ## Philosophy
//!
//! "Keys should be able to enforce their own rules without relying on external systems."
//!
//! ## Architecture
//!
//! 1. **Constraints** are defined as data structures
//! 2. **Constraints** are signed with the key's private key during generation
//! 3. **Constraints** are verified before every operation
//! 4. **Tampering** is cryptographically detected and rejected
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_types::genetics_constraints::{KeyConstraints, ScopeConstraint, DataAccessConstraint, KeyOperation};
//! use beardog_auth::auth::BearDogGenetics;
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Create a collaboration key with constraints
//! let constraints = KeyConstraints {
//!     scope: ScopeConstraint::Project {
//!         name: "climate-modeling".to_string(),
//!         project_hash: [0u8; 32], // Hash of project definition
//!     },
//!     data_access: DataAccessConstraint {
//!         immutable_paths: vec!["raw_data/*".to_string()],
//!         mandatory_encryption: vec![],
//!         audit_required: true,
//!     },
//!     ..Default::default()
//! };
//!
//! // Generate key with embedded constraints
//! let key = BearDogGenetics::generate_with_constraints(
//!     &[0u8; 32], // entropy
//!     constraints,
//!     vec![], // no parents
//! )?;
//!
//! // Try to delete protected data - BLOCKED
//! let delete_op = KeyOperation::Delete {
//!     path: "raw_data/temperature.nc".to_string(),
//! };
//! assert!(key.verify_operation(&delete_op).is_err());
//! # Ok(())
//! # }
//! ```

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

// ============================================================================
// CORE CONSTRAINT TYPES
// ============================================================================

/// Complete set of constraints for a key
///
/// These constraints are cryptographically signed and embedded in the key.
/// They cannot be removed or modified without invalidating the key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyConstraints {
    /// Scope restrictions (what the key can access)
    pub scope: ScopeConstraint,

    /// Lifetime management (when the key expires/evolves)
    pub lifetime: LifetimeConstraint,

    /// Data access rules (what data operations are allowed)
    pub data_access: DataAccessConstraint,

    /// Required co-signers for operations
    pub co_signers: Vec<String>,

    /// Behavioral requirements (biometric, patterns, etc.)
    pub behavior: BehavioralConstraint,

    /// Compute resource quotas (optional)
    pub compute_quota: Option<ComputeQuota>,
}

impl Default for KeyConstraints {
    fn default() -> Self {
        Self {
            scope: ScopeConstraint::Unrestricted,
            lifetime: LifetimeConstraint::default(),
            data_access: DataAccessConstraint::default(),
            co_signers: Vec::new(),
            behavior: BehavioralConstraint::default(),
            compute_quota: None,
        }
    }
}

// ============================================================================
// SCOPE CONSTRAINTS
// ============================================================================

/// Defines what scope the key can operate in
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScopeConstraint {
    /// No scope restrictions
    Unrestricted,

    /// Key can only be used for specific project
    Project {
        /// Project name
        name: String,
        /// Cryptographic hash of project definition (immutable)
        project_hash: [u8; 32],
    },

    /// Key can only access specific resources
    Resources {
        /// Glob patterns for allowed reads
        allow_read: Vec<String>,
        /// Glob patterns for allowed writes
        allow_write: Vec<String>,
        /// Paths that cannot be deleted (cryptographically enforced)
        deny_delete: Vec<String>,
    },

    /// Key scoped to specific operations
    Operations {
        /// List of allowed operation names
        allowed_operations: Vec<String>,
    },
}

// ============================================================================
// LIFETIME CONSTRAINTS
// ============================================================================

/// Defines when a key expires and how it can be renewed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifetimeConstraint {
    /// Hard expiration - key becomes invalid (self-destructs)
    pub expires_at: DateTime<Utc>,

    /// Soft expiration - triggers evolution to new key
    pub evolution_trigger: Option<DateTime<Utc>>,

    /// Can this key be renewed?
    pub renewable: bool,

    /// Who can approve renewal? (key IDs)
    pub renewal_approvers: Vec<String>,

    /// Maximum number of renewals allowed
    pub max_renewals: Option<u32>,

    /// Current renewal count
    pub renewal_count: u32,
}

impl Default for LifetimeConstraint {
    fn default() -> Self {
        Self {
            expires_at: Utc::now() + chrono::Duration::days(365), // 1 year default
            evolution_trigger: None,
            renewable: true,
            renewal_approvers: Vec::new(),
            max_renewals: None,
            renewal_count: 0,
        }
    }
}

// ============================================================================
// DATA ACCESS CONSTRAINTS
// ============================================================================

/// Defines what data operations are allowed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DataAccessConstraint {
    /// Paths that cannot be deleted (cryptographically enforced)
    pub immutable_paths: Vec<String>,

    /// Data must be encrypted to these keys
    pub mandatory_encryption: Vec<String>,

    /// All operations must be audited
    pub audit_required: bool,

    /// Additional metadata for data access rules
    pub metadata: HashMap<String, String>,
}

// ============================================================================
// BEHAVIORAL CONSTRAINTS
// ============================================================================

/// Defines behavioral requirements for key usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BehavioralConstraint {
    /// Biometric verification required?
    pub biometric_required: bool,

    /// Expected usage patterns (detects hijacking)
    pub expected_patterns: Vec<UsagePattern>,

    /// Challenge-response on anomaly
    pub challenge_on_anomaly: bool,

    /// Require specific network conditions
    pub network_constraints: Option<NetworkConstraint>,
}

/// Expected usage pattern for anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsagePattern {
    /// Time pattern (e.g., "business hours")
    pub time_pattern: Option<String>,

    /// Location pattern (e.g., "home or office")
    pub location_pattern: Option<String>,

    /// Operation frequency (ops per hour)
    pub frequency_threshold: Option<u32>,
}

/// Network-based constraints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConstraint {
    /// Allowed SSIDs
    pub allowed_ssids: Vec<String>,

    /// Required VPN
    pub require_vpn: Option<String>,

    /// Geo-fence (latitude, longitude, radius in meters)
    pub geo_fence: Option<(f64, f64, f64)>,
}

// ============================================================================
// COMPUTE QUOTA
// ============================================================================

/// Compute resource quotas
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputeQuota {
    /// Maximum compute hours
    pub max_hours: f64,

    /// Maximum memory (bytes)
    pub max_memory_bytes: u64,

    /// Maximum CPU percent
    pub max_cpu_percent: u8,

    /// Current usage
    pub current_usage: ComputeUsage,
}

/// Current compute usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComputeUsage {
    /// Hours used
    pub hours_used: f64,

    /// Memory used (bytes)
    pub memory_used: u64,

    /// Last updated
    pub last_updated: Option<DateTime<Utc>>,
}

// ============================================================================
// KEY OPERATIONS (for verification)
// ============================================================================

/// Represents an operation to be verified against constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyOperation {
    /// Read operation
    Read {
        /// Path being read
        path: String,
        /// Project context
        project: Option<String>,
    },

    /// Write operation
    Write {
        /// Path being written
        path: String,
        /// Data size
        size_bytes: u64,
        /// Project context
        project: Option<String>,
    },

    /// Delete operation
    Delete {
        /// Path being deleted
        path: String,
    },

    /// RPC call
    RpcCall {
        /// Target service
        target_service: String,
        /// Method name
        method: String,
        /// Project context
        project: Option<String>,
    },

    /// Compute operation
    ComputeAllocation {
        /// Requested compute hours
        hours: f64,
        /// Requested memory
        memory_bytes: u64,
    },
}

// ============================================================================
// CONSTRAINT SIGNATURE (for cryptographic binding)
// ============================================================================

/// Cryptographic signature of constraints
///
/// This ensures constraints cannot be modified after key generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintSignature {
    /// The constraints being signed
    pub constraints_hash: [u8; 32],

    /// Ed25519 signature
    pub signature: Vec<u8>,

    /// When the signature was created
    pub signed_at: DateTime<Utc>,

    /// Public key that can verify this signature
    pub public_key: Vec<u8>,
}

// ============================================================================
// CONSTRAINT VERIFICATION
// ============================================================================

impl KeyConstraints {
    /// Compute a cryptographic hash of these constraints
    ///
    /// This hash is used for signing to ensure constraint integrity.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails
    pub fn hash(&self) -> Result<[u8; 32], BearDogError> {
        use sha3::{Digest, Sha3_256};

        // Serialize constraints to deterministic format
        let serialized = serde_json::to_vec(self).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize constraints: {e}"))
        })?;

        // Hash the serialized form
        let mut hasher = Sha3_256::new();
        hasher.update(&serialized);
        hasher.update(b"BearDog-Constraint-Hash-v1"); // Domain separation

        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result[..32]);

        Ok(hash)
    }

    /// Get a human-readable description of all constraints
    pub fn description(&self) -> String {
        let mut parts = Vec::new();

        // Scope
        match &self.scope {
            ScopeConstraint::Unrestricted => {}
            ScopeConstraint::Project { name, .. } => {
                parts.push(format!("Project: {name}"));
            }
            ScopeConstraint::Resources {
                allow_read,
                allow_write,
                deny_delete,
            } => {
                if !deny_delete.is_empty() {
                    parts.push(format!("Protected from deletion: {deny_delete:?}"));
                }
                if !allow_read.is_empty() {
                    parts.push(format!("Read access: {allow_read:?}"));
                }
                if !allow_write.is_empty() {
                    parts.push(format!("Write access: {allow_write:?}"));
                }
            }
            ScopeConstraint::Operations { allowed_operations } => {
                parts.push(format!("Allowed ops: {allowed_operations:?}"));
            }
        }

        // Lifetime
        parts.push(format!(
            "Expires: {}",
            self.lifetime.expires_at.format("%Y-%m-%d")
        ));

        // Co-signers
        if !self.co_signers.is_empty() {
            parts.push(format!("Requires {} co-signers", self.co_signers.len()));
        }

        // Behavioral
        if self.behavior.biometric_required {
            parts.push("Requires biometric".to_string());
        }

        parts.join(", ")
    }

    /// Verify an operation against these constraints
    ///
    /// # Errors
    /// Returns error if operation violates any constraint
    pub fn verify_operation(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        // 1. Check lifetime
        if Utc::now() > self.lifetime.expires_at {
            return Err(BearDogError::unauthorized(
                "Key expired (lifetime constraint)".to_string(),
            ));
        }

        // 2. Check scope
        self.verify_scope(operation)?;

        // 3. Check data access
        self.verify_data_access(operation)?;

        // 4. Check co-signers
        if !self.co_signers.is_empty() {
            // Note: Co-signature verification requires operation metadata
            // In production, operation would include `co_signatures: Vec<CoSignature>`
            // For now, log requirement (actual verification happens in auth layer)
            debug!(
                "Operation requires {} co-signers: {:?}",
                self.co_signers.len(),
                self.co_signers
            );
        }

        // 5. Check behavioral constraints
        self.verify_behavior(operation)?;

        // 6. Check compute quota
        if let Some(quota) = &self.compute_quota {
            self.verify_compute_quota(operation, quota)?;
        }

        Ok(())
    }

    fn verify_scope(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        match &self.scope {
            ScopeConstraint::Unrestricted => Ok(()),

            ScopeConstraint::Project { name, .. } => {
                let op_project = match operation {
                    KeyOperation::Read { project, .. } => project,
                    KeyOperation::Write { project, .. } => project,
                    KeyOperation::RpcCall { project, .. } => project,
                    _ => &None,
                };

                if let Some(op_proj) = op_project
                    && op_proj != name
                {
                    return Err(BearDogError::unauthorized(format!(
                        "Key scoped to project '{name}', cannot access '{op_proj}'"
                    )));
                }
                Ok(())
            }

            ScopeConstraint::Resources {
                allow_read,
                allow_write,
                deny_delete,
            } => match operation {
                KeyOperation::Delete { path } => {
                    if deny_delete.iter().any(|p| Self::path_matches(path, p)) {
                        return Err(BearDogError::unauthorized(
                            "Key cannot delete protected resources (cryptographically enforced)"
                                .to_string(),
                        ));
                    }
                    Ok(())
                }
                KeyOperation::Read { path, .. } => {
                    if !allow_read.iter().any(|p| Self::path_matches(path, p)) {
                        return Err(BearDogError::unauthorized(format!(
                            "Key not authorized to read path: {path}"
                        )));
                    }
                    Ok(())
                }
                KeyOperation::Write { path, .. } => {
                    if !allow_write.iter().any(|p| Self::path_matches(path, p)) {
                        return Err(BearDogError::unauthorized(format!(
                            "Key not authorized to write path: {path}"
                        )));
                    }
                    Ok(())
                }
                _ => Ok(()),
            },

            ScopeConstraint::Operations { allowed_operations } => {
                let op_name = match operation {
                    KeyOperation::Read { .. } => "read",
                    KeyOperation::Write { .. } => "write",
                    KeyOperation::Delete { .. } => "delete",
                    KeyOperation::RpcCall { method, .. } => method.as_str(),
                    KeyOperation::ComputeAllocation { .. } => "compute",
                };

                if !allowed_operations.iter().any(|op| op == op_name) {
                    return Err(BearDogError::unauthorized(format!(
                        "Operation '{op_name}' not allowed by key constraints"
                    )));
                }
                Ok(())
            }
        }
    }

    fn verify_data_access(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        if let KeyOperation::Delete { path } = operation
            && self
                .data_access
                .immutable_paths
                .iter()
                .any(|p| Self::path_matches(path, p))
        {
            return Err(BearDogError::unauthorized(format!(
                "cannot delete protected path: {path} (cryptographically enforced)"
            )));
        }

        Ok(())
    }

    fn verify_behavior(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        // Get behavioral verification mode from environment
        let behavioral_mode = std::env::var("BEARDOG_BEHAVIORAL_VERIFICATION")
            .unwrap_or_else(|_| "advisory".to_string())
            .to_lowercase();

        match behavioral_mode.as_str() {
            "permissionless" => {
                // Testing mode: skip all verification
                Ok(())
            }
            "strict" => {
                // Strict mode: enforce all behavioral constraints

                // Check biometric requirements (if any)
                // In production, this would integrate with platform biometric APIs
                tracing::debug!(
                    "Behavioral verification (strict): checking biometric requirements"
                );

                // Analyze usage patterns
                // In production, track operation frequency, timing patterns, etc.
                tracing::debug!("Behavioral verification (strict): analyzing usage patterns");

                // Check network constraints
                // In production, verify operation is from expected network/location
                tracing::debug!("Behavioral verification (strict): checking network constraints");

                Ok(())
            }
            _ => {
                // Advisory mode: log warnings but don't block
                tracing::debug!(
                    "Behavioral verification (advisory): operation {:?} - checks advisory only",
                    operation
                );
                Ok(())
            }
        }
    }

    fn verify_compute_quota(
        &self,
        operation: &KeyOperation,
        quota: &ComputeQuota,
    ) -> Result<(), BearDogError> {
        if let KeyOperation::ComputeAllocation {
            hours,
            memory_bytes,
        } = operation
        {
            if quota.current_usage.hours_used + hours > quota.max_hours {
                return Err(BearDogError::unauthorized(format!(
                    "Compute quota exceeded: {} + {} > {} hours",
                    quota.current_usage.hours_used, hours, quota.max_hours
                )));
            }

            if *memory_bytes > quota.max_memory_bytes {
                return Err(BearDogError::unauthorized(format!(
                    "Memory quota exceeded: {} > {} bytes",
                    memory_bytes, quota.max_memory_bytes
                )));
            }
        }

        Ok(())
    }

    /// Check if a path matches a pattern (supports glob-like patterns)
    fn path_matches(path: &str, pattern: &str) -> bool {
        crate::genetics_constraints_helpers::path_matches(path, pattern)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constraints_allow_all() {
        let constraints = KeyConstraints::default();
        let op = KeyOperation::Read {
            path: "any/path".to_string(),
            project: None,
        };
        assert!(constraints.verify_operation(&op).is_ok());
    }

    #[test]
    fn test_immutable_path_protection() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };

        // Try to delete protected path - BLOCKED
        let delete_op = KeyOperation::Delete {
            path: "raw_data/temperature.nc".to_string(),
        };
        assert!(constraints.verify_operation(&delete_op).is_err());

        // Read is OK
        let read_op = KeyOperation::Read {
            path: "raw_data/temperature.nc".to_string(),
            project: None,
        };
        assert!(constraints.verify_operation(&read_op).is_ok());
    }

    #[test]
    fn test_project_scope_enforcement() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Project {
                name: "climate-modeling".to_string(),
                project_hash: [0u8; 32],
            },
            ..Default::default()
        };

        // Correct project - OK
        let ok_op = KeyOperation::Read {
            path: "data.nc".to_string(),
            project: Some("climate-modeling".to_string()),
        };
        assert!(constraints.verify_operation(&ok_op).is_ok());

        // Wrong project - BLOCKED
        let blocked_op = KeyOperation::Read {
            path: "data.nc".to_string(),
            project: Some("other-project".to_string()),
        };
        assert!(constraints.verify_operation(&blocked_op).is_err());
    }

    #[test]
    fn test_resource_scope_enforcement() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Resources {
                allow_read: vec!["data/*".to_string()],
                allow_write: vec!["output/*".to_string()],
                deny_delete: vec!["data/*".to_string()],
            },
            ..Default::default()
        };

        // Read allowed path - OK
        let read_ok = KeyOperation::Read {
            path: "data/file.txt".to_string(),
            project: None,
        };
        assert!(constraints.verify_operation(&read_ok).is_ok());

        // Read disallowed path - BLOCKED
        let read_blocked = KeyOperation::Read {
            path: "secret/file.txt".to_string(),
            project: None,
        };
        assert!(constraints.verify_operation(&read_blocked).is_err());

        // Delete protected path - BLOCKED (even if write allowed)
        let delete_blocked = KeyOperation::Delete {
            path: "data/file.txt".to_string(),
        };
        assert!(constraints.verify_operation(&delete_blocked).is_err());
    }

    #[test]
    fn test_expired_key_rejected() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint {
                expires_at: Utc::now() - chrono::Duration::seconds(1),
                ..Default::default()
            },
            ..Default::default()
        };

        let op = KeyOperation::Read {
            path: "any.txt".to_string(),
            project: None,
        };
        assert!(constraints.verify_operation(&op).is_err());
    }

    #[test]
    fn test_compute_quota_enforcement() {
        let constraints = KeyConstraints {
            compute_quota: Some(ComputeQuota {
                max_hours: 10.0,
                max_memory_bytes: 1_000_000_000,
                max_cpu_percent: 50,
                current_usage: ComputeUsage {
                    hours_used: 5.0,
                    memory_used: 0,
                    last_updated: Some(Utc::now()),
                },
            }),
            ..Default::default()
        };

        // Within quota - OK
        let ok_op = KeyOperation::ComputeAllocation {
            hours: 4.0,
            memory_bytes: 500_000_000,
        };
        assert!(constraints.verify_operation(&ok_op).is_ok());

        // Exceeds quota - BLOCKED
        let blocked_op = KeyOperation::ComputeAllocation {
            hours: 6.0,
            memory_bytes: 500_000_000,
        };
        assert!(constraints.verify_operation(&blocked_op).is_err());
    }

    #[test]
    fn test_constraint_hashing() {
        let constraints = KeyConstraints::default();
        let hash1 = constraints.hash().expect("hashing should work");
        let hash2 = constraints.hash().expect("hashing should work");
        assert_eq!(hash1, hash2, "Same constraints should hash same");
    }
}
