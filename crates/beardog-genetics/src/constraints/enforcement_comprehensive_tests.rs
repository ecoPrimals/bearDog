// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for constraint enforcement
//!
//! These tests cover edge cases, error paths, and complex scenarios
//! to increase test coverage in the genetics crate.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::enforcement::*;
use super::types::*;
use chrono::{Duration, Utc};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::collections::HashSet;

/// Helper to create a valid key pair for testing
fn create_test_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::from_bytes(&[1u8; 32]);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Helper to create valid constraints
fn create_test_constraints() -> KeyConstraints {
    let mut allowed_ops = HashSet::new();
    allowed_ops.insert(OperationType::Encrypt);
    allowed_ops.insert(OperationType::Decrypt);

    KeyConstraints {
        scope: ScopeConstraint::OperationSpecific {
            allowed_operations: allowed_ops,
        },
        lifetime: LifetimeConstraint::Duration {
            months: 24,
            evolution_trigger: Some(12),
        },
        data_access: DataAccessConstraint {
            cannot_delete: vec!["critical/*".to_string()],
            cannot_modify: vec!["immutable/*".to_string()],
            read_only: vec!["secret/*".to_string()],
            must_encrypt_to: vec![],
            immutable_paths: vec![],
        },
        co_signers: vec![],
        behavioral: BehavioralConstraint::default(),
    }
}

/// Helper to sign constraints
fn sign_constraints(constraints: KeyConstraints, signing_key: &SigningKey) -> SignedConstraints {
    let hash = SignedConstraints::compute_hash(&constraints);
    let signature = signing_key.sign(&hash);

    SignedConstraints {
        constraints,
        signature: signature.to_bytes().to_vec(),
        signed_by_key_id: "test-signer-key-id".to_string(),
        created_at: Utc::now(),
        version: SignedConstraints::CURRENT_VERSION,
    }
}

/// Helper to create a basic key operation
fn create_test_operation(op_type: OperationType, path: Option<&str>) -> KeyOperation {
    match op_type {
        OperationType::Encrypt => KeyOperation::Encrypt { recipients: vec![] },
        OperationType::Decrypt => KeyOperation::Decrypt { domain: None },
        OperationType::Sign => KeyOperation::Sign { domain: None },
        OperationType::Delete => KeyOperation::Delete {
            path: path.unwrap_or("default.txt").to_string(),
        },
        OperationType::Modify => KeyOperation::Modify {
            path: path.unwrap_or("default.txt").to_string(),
        },
        OperationType::Read => KeyOperation::Read {
            path: path.unwrap_or("default.txt").to_string(),
        },
        OperationType::Delegate => KeyOperation::Delegate {
            to_key_id: "test-key".to_string(),
            authority: Box::new(KeyOperation::Sign { domain: None }),
        },
        OperationType::Mix => KeyOperation::Mix {
            with_key_ids: vec![],
        },
    }
}

#[test]
fn test_valid_operation_passes() {
    let (signing_key, verifying_key) = create_test_keypair();
    let constraints = create_test_constraints();
    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_ok());
}

#[test]
fn test_invalid_signature_fails() {
    let (signing_key, _) = create_test_keypair();
    // Create a completely different key pair
    let wrong_signing_key = SigningKey::from_bytes(&[255u8; 32]);
    let wrong_verifying_key = wrong_signing_key.verifying_key();

    let constraints = create_test_constraints();
    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        wrong_verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err(), "Should fail with wrong key");
    match result {
        Err(ConstraintViolationError::SignatureVerificationFailed { .. }) => (),
        Err(e) => panic!("Expected SignatureVerificationFailed, got: {e:?}"),
        Ok(()) => panic!("Expected error, got Ok"),
    }
}

#[test]
fn test_use_count_exceeded_fails() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.lifetime = LifetimeConstraint::UseCount {
        max_uses: 10,
        current_uses: 10, // Already at limit
    };

    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err());
    match result {
        Err(ConstraintViolationError::UseCountExceeded {
            max_uses,
            current_uses,
        }) => {
            assert_eq!(max_uses, 10);
            assert_eq!(current_uses, 10);
        }
        _ => panic!("Expected UseCountExceeded"),
    }
}

#[test]
fn test_forbidden_operation_fails() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    let mut allowed_ops = HashSet::new();
    allowed_ops.insert(OperationType::Encrypt);
    // Sign operation not allowed
    constraints.scope = ScopeConstraint::OperationSpecific {
        allowed_operations: allowed_ops,
    };

    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Sign, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err());
    match result {
        Err(ConstraintViolationError::OperationNotAllowed { .. }) => (),
        _ => panic!("Expected OperationNotAllowed"),
    }
}

#[test]
fn test_read_only_path_write_fails() {
    let (signing_key, verifying_key) = create_test_keypair();

    let constraints = create_test_constraints();
    let signed = sign_constraints(constraints, &signing_key);

    // Try to modify a path in read_only list
    let operation = create_test_operation(OperationType::Modify, Some("secret/password.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err());
}

#[test]
fn test_cannot_delete_path_fails() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    // Need to allow Delete operation in scope first
    let mut allowed_ops = HashSet::new();
    allowed_ops.insert(OperationType::Delete);
    constraints.scope = ScopeConstraint::OperationSpecific {
        allowed_operations: allowed_ops,
    };

    let signed = sign_constraints(constraints, &signing_key);

    // Try to delete a path in cannot_delete list
    let operation = create_test_operation(OperationType::Delete, Some("critical/data.bin"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err(), "Should fail to delete protected path");
    match result {
        Err(ConstraintViolationError::DataAccessDenied { operation: op, .. }) => {
            assert_eq!(op, "delete");
        }
        Err(e) => panic!("Expected DataAccessDenied, got: {e:?}"),
        Ok(()) => panic!("Expected error, got Ok"),
    }
}

#[test]
fn test_short_public_key_fails() {
    let (signing_key, _) = create_test_keypair();
    let constraints = create_test_constraints();
    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    // Public key too short
    let short_key = vec![1u8; 16]; // Only 16 bytes instead of 32

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        &short_key,
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err());
    match result {
        Err(ConstraintViolationError::SignatureVerificationFailed { reason }) => {
            assert!(reason.contains("32 bytes"));
        }
        _ => panic!("Expected SignatureVerificationFailed"),
    }
}

#[test]
fn test_tampered_constraints_fail() {
    let (signing_key, verifying_key) = create_test_keypair();
    let constraints = create_test_constraints();
    let mut signed = sign_constraints(constraints, &signing_key);

    // Tamper with constraints after signing
    signed.constraints.lifetime = LifetimeConstraint::Permanent; // Changed!

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err());
    match result {
        Err(ConstraintViolationError::SignatureVerificationFailed { .. }) => (),
        _ => panic!("Expected SignatureVerificationFailed due to tampering"),
    }
}

#[test]
fn test_multiple_operations_tracking() {
    let (signing_key, verifying_key) = create_test_keypair();

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    // First operation - should pass
    let constraints1 = KeyConstraints {
        lifetime: LifetimeConstraint::UseCount {
            max_uses: 3,
            current_uses: 0,
        },
        ..create_test_constraints()
    };
    let signed1 = sign_constraints(constraints1, &signing_key);
    assert!(
        ConstraintEnforcer::verify_operation(
            &signed1,
            &operation,
            verifying_key.as_bytes(),
            &ConstraintEnforcementPolicy::default()
        )
        .is_ok()
    );

    // Second operation - should pass
    let constraints2 = KeyConstraints {
        lifetime: LifetimeConstraint::UseCount {
            max_uses: 3,
            current_uses: 1,
        },
        ..create_test_constraints()
    };
    let signed2 = sign_constraints(constraints2, &signing_key);
    assert!(
        ConstraintEnforcer::verify_operation(
            &signed2,
            &operation,
            verifying_key.as_bytes(),
            &ConstraintEnforcementPolicy::default()
        )
        .is_ok()
    );

    // Third operation - should pass
    let constraints3 = KeyConstraints {
        lifetime: LifetimeConstraint::UseCount {
            max_uses: 3,
            current_uses: 2,
        },
        ..create_test_constraints()
    };
    let signed3 = sign_constraints(constraints3, &signing_key);
    assert!(
        ConstraintEnforcer::verify_operation(
            &signed3,
            &operation,
            verifying_key.as_bytes(),
            &ConstraintEnforcementPolicy::default()
        )
        .is_ok()
    );

    // Fourth operation - should fail (exceeded)
    let constraints4 = KeyConstraints {
        lifetime: LifetimeConstraint::UseCount {
            max_uses: 3,
            current_uses: 3,
        },
        ..create_test_constraints()
    };
    let signed4 = sign_constraints(constraints4, &signing_key);
    assert!(
        ConstraintEnforcer::verify_operation(
            &signed4,
            &operation,
            verifying_key.as_bytes(),
            &ConstraintEnforcementPolicy::default()
        )
        .is_err()
    );
}

#[test]
fn test_constraint_violation_error_display() {
    let error = ConstraintViolationError::KeyExpired {
        expired_at: Utc::now(),
    };
    let display = format!("{error}");
    assert!(display.contains("expired") || display.contains("Expired"));

    let error = ConstraintViolationError::UseCountExceeded {
        max_uses: 10,
        current_uses: 15,
    };
    let display = format!("{error}");
    assert!(display.contains("10") && display.contains("15"));
}

#[test]
fn test_empty_allowed_operations() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.scope = ScopeConstraint::OperationSpecific {
        allowed_operations: HashSet::new(), // No operations allowed
    };

    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    // Should fail because no operations are explicitly allowed
    assert!(result.is_err());
}

#[test]
fn test_permanent_lifetime_passes() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.lifetime = LifetimeConstraint::Permanent; // No expiration

    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_ok());
}

#[test]
fn test_unrestricted_scope_allows_all() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.scope = ScopeConstraint::Unrestricted;

    let signed = sign_constraints(constraints, &signing_key);

    // Try multiple operation types - all should pass
    for op_type in [
        OperationType::Encrypt,
        OperationType::Decrypt,
        OperationType::Sign,
        OperationType::Read,
    ] {
        let operation = create_test_operation(op_type, Some("data/test.txt"));
        let result = ConstraintEnforcer::verify_operation(
            &signed,
            &operation,
            verifying_key.as_bytes(),
            &ConstraintEnforcementPolicy::default(),
        );
        assert!(result.is_ok(), "Operation {op_type:?} should be allowed");
    }
}

#[test]
fn test_limited_scope_domain_check() {
    let (signing_key, _verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.scope = ScopeConstraint::Limited {
        domains: vec!["climate".to_string(), "research".to_string()],
    };

    let signed = sign_constraints(constraints, &signing_key);

    // Verify the constraints were set correctly
    assert!(matches!(
        signed.constraints.scope,
        ScopeConstraint::Limited { .. }
    ));
}

#[test]
fn test_forbidden_scope_domain() {
    let (signing_key, _verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.scope = ScopeConstraint::Forbidden {
        domains: vec!["financial".to_string(), "medical".to_string()],
    };

    let signed = sign_constraints(constraints, &signing_key);

    // Verify the constraints were set correctly
    assert!(matches!(
        signed.constraints.scope,
        ScopeConstraint::Forbidden { .. }
    ));
}

#[test]
fn test_expires_at_constraint() {
    let (signing_key, verifying_key) = create_test_keypair();

    let mut constraints = create_test_constraints();
    constraints.lifetime = LifetimeConstraint::ExpiresAt {
        timestamp: Utc::now() - Duration::hours(1), // Already expired
    };

    let signed = sign_constraints(constraints, &signing_key);

    let operation = create_test_operation(OperationType::Encrypt, Some("data/test.txt"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    // Should fail - key has expired
    assert!(result.is_err());
    match result {
        Err(ConstraintViolationError::KeyExpired { .. }) => (),
        _ => panic!("Expected KeyExpired"),
    }
}

#[test]
fn test_path_pattern_matching() {
    // Test the path matching logic
    assert!(DataAccessConstraint::path_matches(
        "critical/data.bin",
        &["critical/*".to_string()]
    ));

    assert!(!DataAccessConstraint::path_matches(
        "public/data.bin",
        &["critical/*".to_string()]
    ));

    assert!(DataAccessConstraint::path_matches(
        "exact/path",
        &["exact/path".to_string()]
    ));

    assert!(!DataAccessConstraint::path_matches(
        "exact/path2",
        &["exact/path".to_string()]
    ));
}

#[test]
fn test_behavioral_constraints_default() {
    let behavioral = BehavioralConstraint::default();
    assert!(!behavioral.requires_biometric);
    assert!(!behavioral.requires_mfa);
    assert!(behavioral.min_operation_interval_secs.is_none());
    assert!(behavioral.min_entropy_quality.is_none());
}

#[test]
fn test_entropy_quality_ordering() {
    assert!(EntropyQuality::Software < EntropyQuality::Hardware);
    assert!(EntropyQuality::Hardware < EntropyQuality::SecureHardware);
    assert!(EntropyQuality::SecureHardware < EntropyQuality::Hsm);
}

#[test]
fn test_signed_constraints_version() {
    let constraints = create_test_constraints();
    let (signing_key, _) = create_test_keypair();
    let signed = sign_constraints(constraints, &signing_key);

    assert_eq!(signed.version, SignedConstraints::CURRENT_VERSION);
}

#[test]
fn test_cannot_modify_path_fails() {
    let (signing_key, verifying_key) = create_test_keypair();

    let constraints = create_test_constraints();
    let signed = sign_constraints(constraints, &signing_key);

    // Try to modify a path in cannot_modify list
    let operation = create_test_operation(OperationType::Modify, Some("immutable/config.toml"));

    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &operation,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err());
}
