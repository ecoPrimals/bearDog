// SPDX-License-Identifier: AGPL-3.0-or-later

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

mod types;
mod verification;

pub use types::*;

pub use types::{ComputeQuota, KeyConstraints, KeyOperation, ScopeConstraint};

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
                expires_at: chrono::Utc::now() - chrono::Duration::seconds(1),
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
                    last_updated: Some(chrono::Utc::now()),
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
