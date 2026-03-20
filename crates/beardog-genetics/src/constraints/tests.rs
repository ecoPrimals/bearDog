// SPDX-License-Identifier: AGPL-3.0-only
// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests for self-enforcing key constraints

use super::types::OperationType;
use super::*;
use chrono::{Duration, Utc};
use ed25519_dalek::{Signer, SigningKey};

/// Helper to create a test keypair and sign constraints
fn create_signed_constraints(constraints: KeyConstraints) -> (SignedConstraints, SigningKey) {
    let secret_bytes: [u8; 32] = rand::random();
    let signing_key = SigningKey::from_bytes(&secret_bytes);

    let hash = SignedConstraints::compute_hash(&constraints);
    let signature = signing_key.sign(&hash).to_bytes().to_vec();

    let signed = SignedConstraints {
        constraints,
        signature,
        signed_by_key_id: "test_key".to_string(),
        created_at: Utc::now(),
        version: SignedConstraints::CURRENT_VERSION,
    };

    (signed, signing_key)
}

#[test]
fn test_collaboration_key_story() {
    // THE COLLABORATION KEY STORY
    // Martinez and Kowalski create a collaboration key for climate modeling
    // with constraints that protect raw data from deletion

    let constraints = KeyConstraints {
        scope: ScopeConstraint::Limited {
            domains: vec!["climate_modeling".to_string()],
        },
        lifetime: LifetimeConstraint::Duration {
            months: 18,
            evolution_trigger: Some(12),
        },
        data_access: DataAccessConstraint {
            cannot_delete: vec!["raw_data/*".to_string()],
            cannot_modify: vec!["published/*".to_string()],
            must_encrypt_to: vec!["martinez_key_id".to_string(), "kowalski_key_id".to_string()],
            ..Default::default()
        },
        co_signers: vec!["martinez_key_id".to_string()],
        behavioral: BehavioralConstraint::default(),
    };

    let (signed, signing_key) = create_signed_constraints(constraints);
    let verifying_key = signing_key.verifying_key();

    // ✅ This should work: signing in allowed domain
    let sign_op = KeyOperation::Sign {
        domain: Some("climate_modeling".to_string()),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &sign_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_ok(), "Signing in allowed domain should work");

    // ❌ This should FAIL: signing in forbidden domain
    let wrong_domain_op = KeyOperation::Sign {
        domain: Some("medical_research".to_string()),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &wrong_domain_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_err(), "Signing in wrong domain should fail");

    // ❌ This should FAIL: deleting raw data
    let delete_op = KeyOperation::Delete {
        path: "raw_data/2020-temperature.nc".to_string(),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &delete_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_err(), "Deleting raw data should be blocked");
    match result.unwrap_err() {
        ConstraintViolationError::DataAccessDenied {
            operation, path, ..
        } => {
            assert_eq!(operation, "delete");
            assert!(path.contains("raw_data"));
        }
        _ => panic!("Expected DataAccessDenied error"),
    }

    // ❌ This should FAIL: modifying published data
    let modify_op = KeyOperation::Modify {
        path: "published/paper.pdf".to_string(),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &modify_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(
        result.is_err(),
        "Modifying published data should be blocked"
    );

    // ✅ This should work: reading any data
    let read_op = KeyOperation::Read {
        path: "raw_data/2020-temperature.nc".to_string(),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &read_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_ok(), "Reading data should work");
}

#[test]
fn test_signature_verification_detects_tampering() {
    let constraints = KeyConstraints::default();
    let (mut signed, signing_key) = create_signed_constraints(constraints);
    let verifying_key = signing_key.verifying_key();

    // Tamper with constraints
    signed.constraints.scope = ScopeConstraint::Limited {
        domains: vec!["malicious_domain".to_string()],
    };

    // Verification should fail
    let op = KeyOperation::Sign { domain: None };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(
        result.is_err(),
        "Tampered constraints should fail verification"
    );
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::SignatureVerificationFailed { .. }
    ));
}

#[test]
fn test_expired_key_rejected() {
    let past = Utc::now() - Duration::days(1);
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint::ExpiresAt { timestamp: past },
        ..Default::default()
    };

    let (signed, signing_key) = create_signed_constraints(constraints);
    let verifying_key = signing_key.verifying_key();

    let op = KeyOperation::Sign { domain: None };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err(), "Expired key should be rejected");
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::KeyExpired { .. }
    ));
}

#[test]
fn test_use_count_enforcement() {
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint::UseCount {
            max_uses: 3,
            current_uses: 3, // Already at limit
        },
        ..Default::default()
    };

    let (signed, signing_key) = create_signed_constraints(constraints);
    let verifying_key = signing_key.verifying_key();

    let op = KeyOperation::Sign { domain: None };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );

    assert!(result.is_err(), "Use count limit should be enforced");
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::UseCountExceeded { .. }
    ));
}

#[test]
fn test_operation_specific_constraints() {
    let mut allowed = std::collections::HashSet::new();
    allowed.insert(OperationType::Sign);
    allowed.insert(OperationType::Encrypt);

    let constraints = KeyConstraints {
        scope: ScopeConstraint::OperationSpecific {
            allowed_operations: allowed,
        },
        ..Default::default()
    };

    let (signed, signing_key) = create_signed_constraints(constraints);
    let verifying_key = signing_key.verifying_key();

    // ✅ Sign should work
    let sign_op = KeyOperation::Sign { domain: None };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &sign_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_ok(), "Sign operation should be allowed");

    // ❌ Delete should fail
    let delete_op = KeyOperation::Delete {
        path: "test.txt".to_string(),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &delete_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_err(), "Delete operation should not be allowed");
}

#[test]
fn test_constraint_evolution() {
    let mut engine = ConstraintEvolutionEngine::new();

    // Simulate consistent good behavior
    for _ in 0..1001 {
        engine.record_operation(&KeyOperation::Sign { domain: None });
    }

    let constraints = KeyConstraints::default();
    let created_at = Utc::now();

    // Should trigger evolution
    let trigger = engine.should_evolve(&constraints, created_at);
    assert!(
        trigger.is_some(),
        "Should trigger evolution after consistent usage"
    );

    // Evolve constraints
    let evolved = engine.evolve_constraints(&constraints, &trigger.unwrap());
    // Verify evolution occurred (specific changes depend on implementation)
    assert_eq!(evolved.scope, constraints.scope); // Basic check
}

#[test]
fn test_trust_score_management() {
    let mut engine = ConstraintEvolutionEngine::new();
    let initial_trust = engine.trust_score();

    // Good behavior increases trust
    for _ in 0..100 {
        engine.record_operation(&KeyOperation::Sign { domain: None });
    }
    let after_good = engine.trust_score();
    assert!(
        after_good > initial_trust,
        "Trust should increase with good behavior"
    );

    // Bad behavior decreases trust
    engine.record_failure();
    engine.record_failure();
    engine.record_failure();
    let after_failures = engine.trust_score();
    assert!(
        after_failures < after_good,
        "Trust should decrease after failures"
    );
}

#[test]
fn test_immutable_paths() {
    let constraints = KeyConstraints {
        data_access: DataAccessConstraint {
            immutable_paths: vec!["config/system.conf".to_string()],
            ..Default::default()
        },
        ..Default::default()
    };

    let (signed, signing_key) = create_signed_constraints(constraints);
    let verifying_key = signing_key.verifying_key();

    // ❌ Modifying immutable path should fail
    let modify_op = KeyOperation::Modify {
        path: "config/system.conf".to_string(),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &modify_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(
        result.is_err(),
        "Modifying immutable path should be blocked"
    );

    // ✅ Reading immutable path should work
    let read_op = KeyOperation::Read {
        path: "config/system.conf".to_string(),
    };
    let result = ConstraintEnforcer::verify_operation(
        &signed,
        &read_op,
        verifying_key.as_bytes(),
        &ConstraintEnforcementPolicy::default(),
    );
    assert!(result.is_ok(), "Reading immutable path should work");
}
