// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use beardog_types::genetics_constraints::{
    BehavioralConstraint, ComputeQuota, DataAccessConstraint, KeyConstraints, LifetimeConstraint,
    ScopeConstraint,
};
use chrono::{Duration, Utc};
use std::collections::HashMap;

#[test]
fn test_generate_with_constraints() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw_data/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let result = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]);

    assert!(result.is_ok());
    let key = result.unwrap();
    assert!(key.constraints.is_some());
    assert!(key.constraint_signature.is_some());
    assert!(key.public_key.is_some());
}

#[test]
fn test_verify_operation_blocks_protected_delete() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw_data/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Try to delete protected path
    let delete_op = KeyOperation::Delete {
        path: "raw_data/temperature.nc".to_string(),
    };

    let result = key.verify_operation(&delete_op);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("cannot delete protected")
    );
}

#[test]
fn test_verify_operation_allows_unprotected_delete() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw_data/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Try to delete unprotected path
    let delete_op = KeyOperation::Delete {
        path: "processed/temperature.nc".to_string(),
    };

    let result = key.verify_operation(&delete_op);
    assert!(result.is_ok());
}

#[test]
fn test_constraint_integrity_detects_tampering() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw_data/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let mut key =
        BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify integrity is OK initially
    assert!(key.verify_constraint_integrity().is_ok());

    // Tamper with constraints
    if let Some(ref mut constraints) = key.constraints {
        constraints.data_access.immutable_paths.clear(); // Remove protection
    }

    // Integrity check should now fail
    let result = key.verify_constraint_integrity();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("tampered"));
}

#[test]
fn test_expired_key_rejected() {
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint {
            expires_at: Utc::now() - Duration::seconds(1), // Already expired
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    assert!(key.is_expired());

    // Any operation should fail
    let read_op = KeyOperation::Read {
        path: "any/file.txt".to_string(),
        project: None,
    };

    let result = key.verify_operation(&read_op);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("expired"));
}

#[test]
fn test_key_without_constraints_allows_all() {
    let key = BearDogGenetics::default(); // No constraints

    let delete_op = KeyOperation::Delete {
        path: "raw_data/critical.nc".to_string(),
    };

    // Should allow (backward compatible)
    assert!(key.verify_operation(&delete_op).is_ok());
    assert!(!key.is_expired());
}

// ========================================================================
// COMPREHENSIVE TESTS - Scope Constraints
// ========================================================================

#[test]
fn test_project_scope_enforcement() {
    let project_hash = [42u8; 32]; // Mock project hash
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Project {
            name: "climate-model".to_string(),
            project_hash,
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Allowed: Correct project
    let allowed_read = KeyOperation::Read {
        path: "sensor_data/temp.csv".to_string(),
        project: Some("climate-model".to_string()),
    };
    assert!(key.verify_operation(&allowed_read).is_ok());

    // Denied: Wrong project
    let denied_project = KeyOperation::Read {
        path: "sensor_data/temp.csv".to_string(),
        project: Some("other-project".to_string()),
    };
    assert!(key.verify_operation(&denied_project).is_err());
}

#[test]
fn test_resource_scope_enforcement() {
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Resources {
            allow_read: vec!["data/*.csv".to_string()],
            allow_write: vec!["logs/*.log".to_string()],
            deny_delete: vec!["archive/*".to_string()],
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Allowed: Read matching pattern
    let allowed_read = KeyOperation::Read {
        path: "data/measurements.csv".to_string(),
        project: None,
    };
    assert!(key.verify_operation(&allowed_read).is_ok());

    // Allowed: Write matching pattern
    let allowed_write = KeyOperation::Write {
        path: "logs/system.log".to_string(),
        size_bytes: 1024,
        project: None,
    };
    assert!(key.verify_operation(&allowed_write).is_ok());

    // Denied: Delete protected path
    let denied_delete = KeyOperation::Delete {
        path: "archive/2023.tar.gz".to_string(),
    };
    assert!(key.verify_operation(&denied_delete).is_err());
}

#[test]
fn test_operations_scope_enforcement() {
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Operations {
            allowed_operations: vec!["analyze".to_string(), "process".to_string()],
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Allowed: Allowed operation
    let allowed_rpc = KeyOperation::RpcCall {
        target_service: "compute".to_string(),
        method: "analyze".to_string(),
        project: None,
    };
    assert!(key.verify_operation(&allowed_rpc).is_ok());

    // Denied: Disallowed operation
    let denied_rpc = KeyOperation::RpcCall {
        target_service: "compute".to_string(),
        method: "delete_all".to_string(),
        project: None,
    };
    assert!(key.verify_operation(&denied_rpc).is_err());
}

// ========================================================================
// COMPREHENSIVE TESTS - Data Access Constraints
// ========================================================================

#[test]
fn test_multiple_immutable_paths() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec![
                "raw_data/*".to_string(),
                "archive/*".to_string(),
                "provenance/*.json".to_string(),
            ],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // All protected paths should be blocked
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "raw_data/sensor.csv".to_string(),
        })
        .is_err()
    );

    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "archive/2023/data.tar.gz".to_string(),
        })
        .is_err()
    );

    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "provenance/lineage.json".to_string(),
        })
        .is_err()
    );

    // Unprotected path should be allowed
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "tmp/cache.dat".to_string(),
        })
        .is_ok()
    );
}

#[test]
fn test_wildcard_path_matching() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec![
                "*.nc".to_string(),        // All NetCDF files
                "important_*".to_string(), // All files starting with important_
            ],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Protected by *.nc
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "temperature.nc".to_string(),
        })
        .is_err()
    );

    // Protected by important_*
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "important_config.toml".to_string(),
        })
        .is_err()
    );

    // Not protected
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "temp.csv".to_string(),
        })
        .is_ok()
    );
}

#[test]
fn test_nested_path_protection() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["data/raw/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Protected nested path
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "data/raw/sensors/temp.csv".to_string(),
        })
        .is_err()
    );

    // Unprotected sibling path
    assert!(
        key.verify_operation(&KeyOperation::Delete {
            path: "data/processed/temp.csv".to_string(),
        })
        .is_ok()
    );
}

// ========================================================================
// COMPREHENSIVE TESTS - Compute Quotas
// ========================================================================

#[test]
fn test_compute_quota_presence() {
    use beardog_types::genetics_constraints::ComputeUsage;

    let constraints = KeyConstraints {
        compute_quota: Some(ComputeQuota {
            max_hours: 100.0,
            max_memory_bytes: 1024 * 1024 * 100, // 100 MB
            max_cpu_percent: 80,
            current_usage: ComputeUsage::default(),
        }),
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify compute quota is present
    assert!(key.constraints.as_ref().unwrap().compute_quota.is_some());

    // ComputeAllocation operations should work
    let compute_op = KeyOperation::ComputeAllocation {
        hours: 1.5,
        memory_bytes: 1024 * 1024 * 50,
    };
    assert!(key.verify_operation(&compute_op).is_ok());
}

// ========================================================================
// COMPREHENSIVE TESTS - Behavioral & Co-signers
// ========================================================================

#[test]
fn test_behavioral_constraint_specified() {
    let constraints = KeyConstraints {
        behavior: BehavioralConstraint {
            biometric_required: true,
            challenge_on_anomaly: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify behavioral constraints are present
    let behavioral = &key.constraints.as_ref().unwrap().behavior;
    assert!(behavioral.biometric_required);
    assert!(behavioral.challenge_on_anomaly);
}

#[test]
fn test_co_signer_requirement() {
    let constraints = KeyConstraints {
        co_signers: vec![
            "co-signer-key-123".to_string(),
            "co-signer-key-456".to_string(),
        ],
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify co-signers are present in constraints
    assert_eq!(key.constraints.as_ref().unwrap().co_signers.len(), 2);

    // Operations should still verify (co-signature verification is at auth layer)
    let read_op = KeyOperation::Read {
        path: "data.csv".to_string(),
        project: None,
    };
    assert!(key.verify_operation(&read_op).is_ok());
}

// ========================================================================
// COMPREHENSIVE TESTS - Combined Constraints
// ========================================================================

#[test]
fn test_combined_constraints() {
    let project_hash = [42u8; 32];
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Project {
            name: "climate".to_string(),
            project_hash,
        },
        lifetime: LifetimeConstraint {
            expires_at: Utc::now() + Duration::days(30),
            ..Default::default()
        },
        data_access: DataAccessConstraint {
            immutable_paths: vec!["sensor/calibration/*".to_string()],
            audit_required: true,
            ..Default::default()
        },
        compute_quota: Some(ComputeQuota {
            max_hours: 1000.0,
            max_memory_bytes: 1024 * 1024 * 500,
            max_cpu_percent: 90,
            current_usage: Default::default(),
        }),
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Should pass all constraints
    let allowed = KeyOperation::Read {
        path: "sensor/temp.csv".to_string(),
        project: Some("climate".to_string()),
    };
    assert!(key.verify_operation(&allowed).is_ok());

    // Should fail scope constraint (wrong project)
    let wrong_project = KeyOperation::Read {
        path: "sensor/temp.csv".to_string(),
        project: Some("other".to_string()),
    };
    assert!(key.verify_operation(&wrong_project).is_err());

    // Should fail data access constraint (protected path)
    let protected_delete = KeyOperation::Delete {
        path: "sensor/calibration/baseline.dat".to_string(),
    };
    assert!(key.verify_operation(&protected_delete).is_err());
}

// ========================================================================
// COMPREHENSIVE TESTS - Metadata & Audit
// ========================================================================

#[test]
fn test_audit_required_flag() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            audit_required: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify audit flag is present
    assert!(key.constraints.as_ref().unwrap().data_access.audit_required);
}

#[test]
fn test_mandatory_encryption_specified() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            mandatory_encryption: vec!["key1".to_string(), "key2".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify encryption keys are present
    assert_eq!(
        key.constraints
            .as_ref()
            .unwrap()
            .data_access
            .mandatory_encryption
            .len(),
        2
    );
}

#[test]
fn test_constraint_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("purpose".to_string(), "research".to_string());
    metadata.insert("classification".to_string(), "public".to_string());

    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            metadata: metadata.clone(),
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify metadata is preserved
    let stored_metadata = &key.constraints.as_ref().unwrap().data_access.metadata;
    assert_eq!(
        stored_metadata.get("purpose"),
        Some(&"research".to_string())
    );
    assert_eq!(
        stored_metadata.get("classification"),
        Some(&"public".to_string())
    );
}

#[test]
fn test_constraint_signature_verification() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["protected/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify the signature is valid
    assert!(key.verify_constraint_integrity().is_ok());

    // Verify the key has constraints and signature
    assert!(key.constraints.is_some());
    assert!(key.constraint_signature.is_some());
    assert!(key.public_key.is_some());
}

#[test]
fn test_lifecycle_renewal_settings() {
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint {
            renewable: true,
            max_renewals: Some(3),
            renewal_approvers: vec!["admin-key-123".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();

    // Verify renewal settings
    let lifetime = &key.constraints.as_ref().unwrap().lifetime;
    assert!(lifetime.renewable);
    assert_eq!(lifetime.max_renewals, Some(3));
    assert_eq!(lifetime.renewal_approvers.len(), 1);
}

#[test]
fn test_verify_constraint_integrity_missing_signature() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };
    let entropy = vec![1u8; 32];
    let mut key =
        BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();
    key.constraint_signature = None;
    let r = key.verify_constraint_integrity();
    assert!(r.is_err());
}

#[test]
fn test_verify_constraint_integrity_missing_public_key() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };
    let entropy = vec![1u8; 32];
    let mut key =
        BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();
    key.public_key = None;
    let r = key.verify_constraint_integrity();
    assert!(r.is_err());
}

#[test]
fn test_verify_constraint_integrity_invalid_public_key_length() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["raw/*".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };
    let entropy = vec![1u8; 32];
    let mut key =
        BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();
    key.public_key = Some(vec![0u8; 16]);
    let r = key.verify_constraint_integrity();
    assert!(r.is_err());
}

#[test]
fn test_issue_adapter_certificate_rejects_expired_key() {
    use beardog_types::adapter_certificates::AdapterClassification;
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint {
            expires_at: Utc::now() - Duration::seconds(1),
            ..Default::default()
        },
        ..Default::default()
    };
    let entropy = vec![1u8; 32];
    let key = BearDogGenetics::generate_with_constraints(&entropy, constraints, vec![]).unwrap();
    let r = key.issue_adapter_certificate(
        "adapter::test",
        AdapterClassification::Human,
        Duration::hours(1),
    );
    assert!(r.is_err());
}
