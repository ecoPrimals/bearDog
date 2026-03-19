// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for constraint enforcement

use super::enforcer::ConstraintEnforcer;
use super::errors::ConstraintViolationError;
use crate::constraints::types::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashSet;

// === Scope Tests ===

#[test]
fn test_scope_unrestricted() {
    let scope = ScopeConstraint::Unrestricted;
    let op = KeyOperation::Sign {
        domain: Some("any_domain".to_string()),
    };
    assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
}

#[test]
fn test_scope_limited_allowed() {
    let scope = ScopeConstraint::Limited {
        domains: vec!["climate_modeling".to_string()],
    };
    let op = KeyOperation::Sign {
        domain: Some("climate_modeling".to_string()),
    };
    assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
}

#[test]
fn test_scope_limited_no_domain() {
    let scope = ScopeConstraint::Limited {
        domains: vec!["climate_modeling".to_string()],
    };
    let op = KeyOperation::Sign { domain: None };
    assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
}

#[test]
fn test_scope_violation() {
    let scope = ScopeConstraint::Limited {
        domains: vec!["climate_modeling".to_string()],
    };
    let op = KeyOperation::Sign {
        domain: Some("medical_research".to_string()),
    };
    let result = ConstraintEnforcer::check_scope(&scope, &op);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::ScopeViolation { .. }
    ));
}

#[test]
fn test_scope_forbidden_blocked() {
    let scope = ScopeConstraint::Forbidden {
        domains: vec!["forbidden_zone".to_string()],
    };
    let op = KeyOperation::Sign {
        domain: Some("forbidden_zone".to_string()),
    };
    let result = ConstraintEnforcer::check_scope(&scope, &op);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::ForbiddenDomain { .. }
    ));
}

#[test]
fn test_scope_forbidden_allowed() {
    let scope = ScopeConstraint::Forbidden {
        domains: vec!["forbidden_zone".to_string()],
    };
    let op = KeyOperation::Sign {
        domain: Some("ok_domain".to_string()),
    };
    assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
}

#[test]
fn test_scope_forbidden_no_domain() {
    let scope = ScopeConstraint::Forbidden {
        domains: vec!["forbidden_zone".to_string()],
    };
    let op = KeyOperation::Sign { domain: None };
    assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
}

#[test]
fn test_scope_operation_specific_allowed() {
    let scope = ScopeConstraint::OperationSpecific {
        allowed_operations: HashSet::from([OperationType::Sign, OperationType::Read]),
    };
    let op = KeyOperation::Sign { domain: None };
    assert!(ConstraintEnforcer::check_scope(&scope, &op).is_ok());
}

#[test]
fn test_scope_operation_specific_denied() {
    let scope = ScopeConstraint::OperationSpecific {
        allowed_operations: HashSet::from([OperationType::Read]),
    };
    let op = KeyOperation::Sign { domain: None };
    let result = ConstraintEnforcer::check_scope(&scope, &op);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::OperationNotAllowed { .. }
    ));
}

// === Lifetime Tests ===

#[test]
fn test_lifetime_permanent() {
    let lifetime = LifetimeConstraint::Permanent;
    assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
}

#[test]
fn test_expired_key() {
    use chrono::Duration;
    let past = Utc::now() - Duration::days(1);
    let lifetime = LifetimeConstraint::ExpiresAt { timestamp: past };
    let result = ConstraintEnforcer::check_lifetime(&lifetime);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::KeyExpired { .. }
    ));
}

#[test]
fn test_lifetime_not_expired() {
    use chrono::Duration;
    let future = Utc::now() + Duration::days(365);
    let lifetime = LifetimeConstraint::ExpiresAt { timestamp: future };
    assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
}

#[test]
fn test_lifetime_duration() {
    let lifetime = LifetimeConstraint::Duration {
        months: 12,
        evolution_trigger: Some(6),
    };
    assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
}

#[test]
fn test_lifetime_use_count_ok() {
    let lifetime = LifetimeConstraint::UseCount {
        max_uses: 100,
        current_uses: 50,
    };
    assert!(ConstraintEnforcer::check_lifetime(&lifetime).is_ok());
}

#[test]
fn test_lifetime_use_count_exceeded() {
    let lifetime = LifetimeConstraint::UseCount {
        max_uses: 100,
        current_uses: 100,
    };
    let result = ConstraintEnforcer::check_lifetime(&lifetime);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::UseCountExceeded {
            max_uses: 100,
            current_uses: 100
        }
    ));
}

// === Data Access Tests ===

#[test]
fn test_data_access_denial() {
    let data_access = DataAccessConstraint {
        cannot_delete: vec!["raw_data/*".to_string()],
        ..Default::default()
    };
    let op = KeyOperation::Delete {
        path: "raw_data/temperature.nc".to_string(),
    };
    let result = ConstraintEnforcer::check_data_access(&data_access, &op);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::DataAccessDenied { .. }
    ));
}

#[test]
fn test_data_access_modify_cannot_modify() {
    let data_access = DataAccessConstraint {
        cannot_modify: vec!["config/*".to_string()],
        ..Default::default()
    };
    let op = KeyOperation::Modify {
        path: "config/settings.toml".to_string(),
    };
    let result = ConstraintEnforcer::check_data_access(&data_access, &op);
    assert!(result.is_err());
}

#[test]
fn test_data_access_modify_immutable_path() {
    let data_access = DataAccessConstraint {
        immutable_paths: vec!["genesis/*".to_string()],
        ..Default::default()
    };
    let op = KeyOperation::Modify {
        path: "genesis/root.key".to_string(),
    };
    let result = ConstraintEnforcer::check_data_access(&data_access, &op);
    assert!(result.is_err());
}

#[test]
fn test_data_access_no_path_operation() {
    let data_access = DataAccessConstraint {
        cannot_delete: vec!["raw_data/*".to_string()],
        ..Default::default()
    };
    let op = KeyOperation::Sign { domain: None };
    assert!(ConstraintEnforcer::check_data_access(&data_access, &op).is_ok());
}

#[test]
fn test_data_access_allowed_path() {
    let data_access = DataAccessConstraint {
        cannot_delete: vec!["raw_data/*".to_string()],
        ..Default::default()
    };
    let op = KeyOperation::Delete {
        path: "temp/file.txt".to_string(),
    };
    assert!(ConstraintEnforcer::check_data_access(&data_access, &op).is_ok());
}

// === Co-Signer Tests ===

#[test]
fn test_co_signers_empty() {
    let op = KeyOperation::Sign { domain: None };
    assert!(ConstraintEnforcer::check_co_signers(&[], &op).is_ok());
}

#[test]
fn test_co_signers_permissionless() {
    std::env::set_var("BEARDOG_MULTISIG_MODE", "permissionless");
    let op = KeyOperation::Sign { domain: None };
    let co_signers = vec!["alice".to_string(), "bob".to_string()];
    let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
    std::env::remove_var("BEARDOG_MULTISIG_MODE");
    assert!(result.is_ok());
}

#[test]
fn test_co_signers_full_mode() {
    std::env::set_var("BEARDOG_MULTISIG_MODE", "full");
    let op = KeyOperation::Sign { domain: None };
    let co_signers = vec!["alice".to_string(), "bob".to_string()];
    let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
    std::env::remove_var("BEARDOG_MULTISIG_MODE");
    assert!(result.is_ok());
}

#[test]
fn test_co_signers_threshold_mode() {
    std::env::set_var("BEARDOG_MULTISIG_MODE", "threshold");
    std::env::set_var("BEARDOG_MULTISIG_THRESHOLD", "2");
    let op = KeyOperation::Sign { domain: None };
    let co_signers = vec!["alice".to_string(), "bob".to_string()];
    let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
    std::env::remove_var("BEARDOG_MULTISIG_MODE");
    std::env::remove_var("BEARDOG_MULTISIG_THRESHOLD");
    assert!(result.is_ok());
}

#[test]
fn test_co_signers_threshold_insufficient() {
    std::env::set_var("BEARDOG_MULTISIG_MODE", "threshold");
    std::env::set_var("BEARDOG_MULTISIG_THRESHOLD", "5");
    let op = KeyOperation::Sign { domain: None };
    let co_signers = vec!["alice".to_string(), "bob".to_string()];
    let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
    std::env::remove_var("BEARDOG_MULTISIG_MODE");
    std::env::remove_var("BEARDOG_MULTISIG_THRESHOLD");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ConstraintViolationError::CoSignerRequired { .. }
    ));
}

#[test]
fn test_co_signers_unknown_mode() {
    std::env::set_var("BEARDOG_MULTISIG_MODE", "unknown_mode");
    let op = KeyOperation::Sign { domain: None };
    let co_signers = vec!["alice".to_string()];
    let result = ConstraintEnforcer::check_co_signers(&co_signers, &op);
    std::env::remove_var("BEARDOG_MULTISIG_MODE");
    assert!(result.is_ok());
}

// === Behavioral Tests ===

#[test]
fn test_behavioral_permissionless() {
    std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "permissionless");
    let behavioral = BehavioralConstraint {
        requires_biometric: true,
        requires_mfa: true,
        min_operation_interval_secs: Some(60),
        min_entropy_quality: None,
    };
    let result = ConstraintEnforcer::check_behavioral(&behavioral);
    std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
    assert!(result.is_ok());
}

#[test]
fn test_behavioral_strict() {
    std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "strict");
    let behavioral = BehavioralConstraint {
        requires_biometric: true,
        requires_mfa: true,
        min_operation_interval_secs: Some(30),
        min_entropy_quality: None,
    };
    let result = ConstraintEnforcer::check_behavioral(&behavioral);
    std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
    assert!(result.is_ok());
}

#[test]
fn test_behavioral_relaxed() {
    std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "relaxed");
    let behavioral = BehavioralConstraint {
        requires_biometric: true,
        requires_mfa: true,
        min_operation_interval_secs: Some(10),
        min_entropy_quality: None,
    };
    let result = ConstraintEnforcer::check_behavioral(&behavioral);
    std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
    assert!(result.is_ok());
}

#[test]
fn test_behavioral_unknown_mode() {
    std::env::set_var("BEARDOG_BEHAVIORAL_MODE", "unknown_mode");
    let behavioral = BehavioralConstraint {
        requires_biometric: true,
        requires_mfa: true,
        min_operation_interval_secs: Some(5),
        min_entropy_quality: None,
    };
    let result = ConstraintEnforcer::check_behavioral(&behavioral);
    std::env::remove_var("BEARDOG_BEHAVIORAL_MODE");
    assert!(result.is_ok());
}

#[test]
fn test_behavioral_no_requirements() {
    let behavioral = BehavioralConstraint {
        requires_biometric: false,
        requires_mfa: false,
        min_operation_interval_secs: None,
        min_entropy_quality: None,
    };
    assert!(ConstraintEnforcer::check_behavioral(&behavioral).is_ok());
}

// === Display Tests ===

#[test]
fn test_display_signature_verification_failed() {
    let err = ConstraintViolationError::SignatureVerificationFailed {
        reason: "tampered".to_string(),
    };
    assert!(err.to_string().contains("tampered"));
}

#[test]
fn test_display_key_expired() {
    let err = ConstraintViolationError::KeyExpired {
        expired_at: Utc::now(),
    };
    assert!(err.to_string().contains("expired"));
}

#[test]
fn test_display_use_count_exceeded() {
    let err = ConstraintViolationError::UseCountExceeded {
        max_uses: 100,
        current_uses: 101,
    };
    let s = err.to_string();
    assert!(s.contains("101") && s.contains("100"));
}

#[test]
fn test_display_scope_violation() {
    let err = ConstraintViolationError::ScopeViolation {
        allowed: vec!["a".into()],
        attempted: "b".into(),
    };
    assert!(err.to_string().contains("b"));
}

#[test]
fn test_display_forbidden_domain() {
    let err = ConstraintViolationError::ForbiddenDomain {
        domain: "evil".into(),
    };
    assert!(err.to_string().contains("evil"));
}

#[test]
fn test_display_operation_not_allowed() {
    let err = ConstraintViolationError::OperationNotAllowed {
        operation: OperationType::Sign,
        allowed: vec![OperationType::Read],
    };
    assert!(err.to_string().contains("not allowed"));
}

#[test]
fn test_display_data_access_denied() {
    let err = ConstraintViolationError::DataAccessDenied {
        operation: "delete".into(),
        path: "/data".into(),
        reason: "immutable".into(),
    };
    assert!(err.to_string().contains("immutable"));
}

#[test]
fn test_display_co_signer_required() {
    let err = ConstraintViolationError::CoSignerRequired {
        required: vec!["alice".into()],
        present: vec![],
    };
    assert!(err.to_string().contains("alice"));
}

#[test]
fn test_display_behavioral_requirement() {
    let err = ConstraintViolationError::BehavioralRequirementNotMet {
        requirement: "biometric".into(),
    };
    assert!(err.to_string().contains("biometric"));
}

#[test]
fn test_error_conversion_to_beardog_error() {
    let err = ConstraintViolationError::ForbiddenDomain {
        domain: "test".into(),
    };
    let beardog_err: BearDogError = err.into();
    assert!(beardog_err.to_string().contains("Constraint violation"));
}

// === Signature Verification Tests ===

#[test]
fn test_verify_signature_short_pubkey() {
    let signed = SignedConstraints {
        constraints: KeyConstraints::default(),
        signature: vec![0u8; 64],
        signed_by_key_id: "test".into(),
        created_at: Utc::now(),
        version: 1,
    };
    let result = ConstraintEnforcer::verify_signature(&signed, &[0u8; 16]);
    assert!(result.is_err());
}

#[test]
fn test_verify_signature_short_signature() {
    let signed = SignedConstraints {
        constraints: KeyConstraints::default(),
        signature: vec![0u8; 32],
        signed_by_key_id: "test".into(),
        created_at: Utc::now(),
        version: 1,
    };
    let result = ConstraintEnforcer::verify_signature(&signed, &[0u8; 32]);
    assert!(result.is_err());
}
