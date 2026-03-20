// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage: constraint enforcement (`verify_operation`, violation errors).

use beardog_errors::BearDogError;
use chrono::Utc;

// ═══════════════════════════════════════════════════════════════════
// constraints/enforcement.rs - Test via verify_operation public API
// ═══════════════════════════════════════════════════════════════════

mod enforcement_tests {
    use super::{BearDogError, Utc};
    use crate::constraints::types::OperationType;
    use crate::constraints::*;
    use ed25519_dalek::{Signer, SigningKey};
    use std::collections::HashSet;

    /// Helper to create properly signed constraints
    fn sign_constraints(constraints: KeyConstraints) -> (SignedConstraints, Vec<u8>) {
        let signing_key = SigningKey::from_bytes(&[99u8; 32]);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let hash = SignedConstraints::compute_hash(&constraints);
        let signature = signing_key.sign(&hash).to_bytes().to_vec();

        let signed = SignedConstraints {
            constraints,
            signature,
            signed_by_key_id: "test-key-001".to_string(),
            created_at: Utc::now(),
            version: SignedConstraints::CURRENT_VERSION,
        };

        (signed, public_key)
    }

    #[test]
    fn test_verify_operation_scope_unrestricted() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Unrestricted,
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("any_domain".to_string()),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_limited_allowed() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["climate".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("climate".to_string()),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_limited_denied() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["climate".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("military".to_string()),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::ScopeViolation { .. }
        ));
    }

    #[test]
    fn test_verify_operation_scope_forbidden() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Forbidden {
                domains: vec!["military".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("military".to_string()),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::ForbiddenDomain { .. }
        ));
    }

    #[test]
    fn test_verify_operation_scope_forbidden_allowed() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Forbidden {
                domains: vec!["military".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("climate".to_string()),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_operation_specific() {
        let mut allowed = HashSet::new();
        allowed.insert(OperationType::Sign);
        allowed.insert(OperationType::Encrypt);
        let constraints = KeyConstraints {
            scope: ScopeConstraint::OperationSpecific {
                allowed_operations: allowed,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);

        // Allowed operation
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_operation_specific_denied() {
        let mut allowed = HashSet::new();
        allowed.insert(OperationType::Sign);
        let constraints = KeyConstraints {
            scope: ScopeConstraint::OperationSpecific {
                allowed_operations: allowed,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Delete {
            path: "test.txt".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::OperationNotAllowed { .. }
        ));
    }

    #[test]
    fn test_verify_operation_lifetime_permanent() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Permanent,
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_not_expired() {
        let future = Utc::now() + chrono::Duration::days(30);
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::ExpiresAt { timestamp: future },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_expired() {
        let past = Utc::now() - chrono::Duration::days(1);
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::ExpiresAt { timestamp: past },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::KeyExpired { .. }
        ));
    }

    #[test]
    fn test_verify_operation_lifetime_duration() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 12,
                evolution_trigger: Some(6),
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_use_count_ok() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::UseCount {
                max_uses: 100,
                current_uses: 50,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_use_count_exceeded() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::UseCount {
                max_uses: 10,
                current_uses: 10,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::UseCountExceeded { .. }
        ));
    }

    #[test]
    fn test_verify_operation_data_access_delete_denied() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                cannot_delete: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Delete {
            path: "raw_data/file.nc".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_operation_data_access_modify_immutable() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["config/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Modify {
            path: "config/settings.toml".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_operation_data_access_modify_cannot_modify() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                cannot_modify: vec!["audit/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Modify {
            path: "audit/log.txt".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_operation_data_access_read_ok() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint::default(),
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Read {
            path: "data.txt".to_string(),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_co_signers_empty() {
        let constraints = KeyConstraints {
            co_signers: vec![],
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_invalid_signature() {
        let constraints = KeyConstraints::default();
        let (mut signed, pk) = sign_constraints(constraints);
        // Tamper with signature
        signed.signature = vec![0xFF; 64];
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::SignatureVerificationFailed { .. }
        ));
    }

    #[test]
    fn test_verify_operation_short_public_key() {
        let constraints = KeyConstraints::default();
        let (signed, _pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &[0u8; 16]);
        assert!(result.is_err());
    }

    // ConstraintViolationError Display and From coverage
    #[test]
    fn test_violation_error_display() {
        let err = ConstraintViolationError::SignatureVerificationFailed {
            reason: "bad sig".to_string(),
        };
        assert!(format!("{err}").contains("bad sig"));

        let err = ConstraintViolationError::KeyExpired {
            expired_at: Utc::now(),
        };
        assert!(format!("{err}").contains("expired"));

        let err = ConstraintViolationError::UseCountExceeded {
            max_uses: 10,
            current_uses: 15,
        };
        assert!(format!("{err}").contains("15/10"));

        let err = ConstraintViolationError::ScopeViolation {
            allowed: vec!["a".to_string()],
            attempted: "b".to_string(),
        };
        assert!(format!("{err}").contains("Scope violation"));

        let err = ConstraintViolationError::ForbiddenDomain {
            domain: "military".to_string(),
        };
        assert!(format!("{err}").contains("military"));

        let err = ConstraintViolationError::OperationNotAllowed {
            operation: OperationType::Delete,
            allowed: vec![OperationType::Sign],
        };
        assert!(format!("{err}").contains("not allowed"));

        let err = ConstraintViolationError::DataAccessDenied {
            operation: "delete".to_string(),
            path: "/test".to_string(),
            reason: "forbidden".to_string(),
        };
        assert!(format!("{err}").contains("Data access denied"));

        let err = ConstraintViolationError::CoSignerRequired {
            required: vec!["alice".to_string()],
            present: vec![],
        };
        assert!(format!("{err}").contains("Co-signer"));

        let err = ConstraintViolationError::BehavioralRequirementNotMet {
            requirement: "biometric".to_string(),
        };
        assert!(format!("{err}").contains("biometric"));
    }

    #[test]
    fn test_violation_error_into_beardog_error() {
        let err = ConstraintViolationError::KeyExpired {
            expired_at: Utc::now(),
        };
        let beardog_err: BearDogError = err.into();
        let msg = format!("{beardog_err}");
        assert!(msg.contains("Constraint violation"));
    }
}
