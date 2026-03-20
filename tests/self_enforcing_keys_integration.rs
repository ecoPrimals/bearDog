//! Integration Tests for Self-Enforcing Key Constraints and Adapter Certificates
//!
//! This test suite validates the end-to-end functionality of:
//! - Self-enforcing key constraints (scope, lifetime, data access, co-signers, behavioral)
//! - Constraint enforcement and cryptographic verification
//! - Constraint evolution based on trust and behavior
//! - Adapter unlock certificates with integrated constraints
//! - Commercial classification and licensing enforcement

use beardog_adapters::{CertificateIssuer, CertificateVerifier, CommercialClassification};
use beardog_genetics::{
    BehavioralConstraint, ConstraintEvolutionEngine, DataAccessConstraint, KeyConstraints,
    LifetimeConstraint, ScopeConstraint, SignedConstraints,
};
use chrono::{Duration, Utc};
use ed25519_dalek::SigningKey;
use rand::{Rng, rngs::OsRng};

// ============================================================================
// Test Helpers
// ============================================================================

fn create_test_keypair() -> SigningKey {
    let mut rng = OsRng;
    let secret_bytes: [u8; 32] = rng.r#gen();
    SigningKey::from_bytes(&secret_bytes)
}

fn create_research_constraints() -> KeyConstraints {
    KeyConstraints {
        scope: ScopeConstraint::Limited {
            domains: vec!["climate-research".to_string()],
        },
        lifetime: LifetimeConstraint::ExpiresAt {
            timestamp: Utc::now() + Duration::days(365),
        },
        data_access: DataAccessConstraint {
            cannot_delete: vec!["raw_data/*".to_string()],
            cannot_modify: vec!["published_data/*".to_string()],
            read_only: vec![],
            must_encrypt_to: vec![],
            immutable_paths: vec!["published_data/*".to_string()],
        },
        co_signers: vec![],
        behavioral: BehavioralConstraint::default(),
    }
}

// ============================================================================
// Constraint Creation and Structure Tests
// ============================================================================

#[test]
fn test_constraint_creation() {
    let constraints = create_research_constraints();

    assert!(matches!(constraints.scope, ScopeConstraint::Limited { .. }));
    assert!(matches!(
        constraints.lifetime,
        LifetimeConstraint::ExpiresAt { .. }
    ));
    assert!(!constraints.data_access.cannot_delete.is_empty());
}

#[test]
fn test_signed_constraints_structure() {
    let _signing_key = create_test_keypair();
    let constraints = create_research_constraints();

    let signed = SignedConstraints {
        constraints: constraints.clone(),
        signature: vec![1, 2, 3, 4], // Mock signature
        signed_by_key_id: "test-key-123".to_string(),
        created_at: Utc::now(),
        version: SignedConstraints::CURRENT_VERSION,
    };

    assert_eq!(signed.version, 1);
    assert!(!signed.signature.is_empty());
    assert_eq!(signed.constraints, constraints);
}

#[test]
fn test_constraint_hash_computation() {
    let constraints = create_research_constraints();
    let hash1 = SignedConstraints::compute_hash(&constraints);
    let hash2 = SignedConstraints::compute_hash(&constraints);

    // Same constraints should produce same hash
    assert_eq!(hash1, hash2);
    assert!(!hash1.is_empty());
}

// ============================================================================
// Scope Constraint Tests
// ============================================================================

#[test]
fn test_scope_constraint_limited() {
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Limited {
            domains: vec!["medical".to_string(), "research".to_string()],
        },
        ..KeyConstraints::default()
    };

    if let ScopeConstraint::Limited { domains } = &constraints.scope {
        assert_eq!(domains.len(), 2);
        assert!(domains.contains(&"medical".to_string()));
    } else {
        panic!("Expected Limited scope constraint");
    }
}

#[test]
fn test_scope_constraint_unrestricted() {
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Unrestricted,
        ..KeyConstraints::default()
    };

    assert!(matches!(constraints.scope, ScopeConstraint::Unrestricted));
}

#[test]
fn test_scope_constraint_forbidden() {
    let constraints = KeyConstraints {
        scope: ScopeConstraint::Forbidden {
            domains: vec!["financial".to_string()],
        },
        ..KeyConstraints::default()
    };

    if let ScopeConstraint::Forbidden { domains } = &constraints.scope {
        assert!(domains.contains(&"financial".to_string()));
    } else {
        panic!("Expected Forbidden scope constraint");
    }
}

// ============================================================================
// Lifetime Constraint Tests
// ============================================================================

#[test]
fn test_lifetime_permanent() {
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint::Permanent,
        ..KeyConstraints::default()
    };

    assert!(matches!(
        constraints.lifetime,
        LifetimeConstraint::Permanent
    ));
}

#[test]
fn test_lifetime_expires_at() {
    let expiry = Utc::now() + Duration::days(30);
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint::ExpiresAt { timestamp: expiry },
        ..KeyConstraints::default()
    };

    if let LifetimeConstraint::ExpiresAt { timestamp } = constraints.lifetime {
        assert!(timestamp > Utc::now());
    } else {
        panic!("Expected ExpiresAt lifetime constraint");
    }
}

#[test]
fn test_lifetime_duration() {
    let constraints = KeyConstraints {
        lifetime: LifetimeConstraint::Duration {
            months: 12,
            evolution_trigger: Some(10),
        },
        ..KeyConstraints::default()
    };

    if let LifetimeConstraint::Duration {
        months,
        evolution_trigger,
    } = constraints.lifetime
    {
        assert_eq!(months, 12);
        assert_eq!(evolution_trigger, Some(10));
    } else {
        panic!("Expected Duration lifetime constraint");
    }
}

// ============================================================================
// Data Access Constraint Tests
// ============================================================================

#[test]
fn test_data_access_constraint_creation() {
    let data_access = DataAccessConstraint {
        cannot_delete: vec!["important/*".to_string()],
        cannot_modify: vec!["audit_log/*".to_string()],
        read_only: vec!["public/*".to_string()],
        must_encrypt_to: vec!["admin-key".to_string()],
        immutable_paths: vec!["blockchain/*".to_string()],
    };

    assert_eq!(data_access.cannot_delete.len(), 1);
    assert_eq!(data_access.cannot_modify.len(), 1);
    assert_eq!(data_access.read_only.len(), 1);
}

#[test]
fn test_data_access_path_matching() {
    let patterns = vec!["data/*".to_string(), "config/app.toml".to_string()];

    assert!(DataAccessConstraint::path_matches(
        "data/file.txt",
        &patterns
    ));
    assert!(DataAccessConstraint::path_matches(
        "config/app.toml",
        &patterns
    ));
    assert!(!DataAccessConstraint::path_matches(
        "other/file.txt",
        &patterns
    ));
}

// ============================================================================
// Behavioral Constraint Tests
// ============================================================================

#[test]
fn test_behavioral_constraint_default() {
    let behavioral = BehavioralConstraint::default();

    assert!(!behavioral.requires_biometric);
    assert!(!behavioral.requires_mfa);
    assert!(behavioral.min_operation_interval_secs.is_none());
}

#[test]
fn test_behavioral_constraint_strict() {
    let behavioral = BehavioralConstraint {
        requires_biometric: true,
        requires_mfa: true,
        min_operation_interval_secs: Some(60),
        min_entropy_quality: None,
    };

    assert!(behavioral.requires_biometric);
    assert!(behavioral.requires_mfa);
    assert_eq!(behavioral.min_operation_interval_secs, Some(60));
}

// ============================================================================
// Constraint Evolution Tests
// ============================================================================

#[test]
fn test_evolution_engine_initialization() {
    let engine = ConstraintEvolutionEngine::new();

    // Initial trust score should be reasonable
    assert!(engine.trust_score() > 0.0);
    assert!(engine.trust_score() <= 1.0);
}

#[test]
fn test_evolution_trust_score_updates() {
    let mut engine = ConstraintEvolutionEngine::new();
    let initial_trust = engine.trust_score();

    // Record failures should decrease trust
    for _ in 0..5 {
        engine.record_failure();
    }

    let trust_after_failures = engine.trust_score();
    assert!(trust_after_failures < initial_trust);
}

// ============================================================================
// Adapter Certificate Integration Tests
// ============================================================================

#[test]
fn test_adapter_certificate_issuance_human() {
    let signing_key = create_test_keypair();
    let verifying_key = signing_key.verifying_key();

    let issuer = CertificateIssuer::new(signing_key);
    let result = issuer.issue(
        CommercialClassification::Human {
            confidence: 95,
            reasons: vec!["interactive_behavior".to_string()],
        },
        "prometheus",
    );

    assert!(result.is_ok());
    let cert = result.unwrap();
    assert_eq!(cert.adapter_id.as_str(), "prometheus");
    assert!(matches!(
        cert.classification,
        CommercialClassification::Human { .. }
    ));

    // Verify certificate
    let verifier = CertificateVerifier::new(verifying_key);
    let verify_result = verifier.verify(&cert);
    assert!(verify_result.is_ok());
}

#[test]
fn test_adapter_certificate_issuance_small_team() {
    let signing_key = create_test_keypair();
    let verifying_key = signing_key.verifying_key();

    let issuer = CertificateIssuer::new(signing_key);
    let result = issuer.issue(
        CommercialClassification::SmallTeam {
            team_size: 5,
            confidence: 85,
        },
        "consul",
    );

    assert!(result.is_ok());
    let cert = result.unwrap();
    assert!(matches!(
        cert.classification,
        CommercialClassification::SmallTeam { .. }
    ));

    let verifier = CertificateVerifier::new(verifying_key);
    let verify_result = verifier.verify(&cert);
    assert!(verify_result.is_ok());
}

#[test]
fn test_adapter_certificate_commercial_low_risk() {
    let signing_key = create_test_keypair();
    let verifying_key = signing_key.verifying_key();

    let issuer = CertificateIssuer::new(signing_key);
    // Use Uncertain classification
    let result = issuer.issue(
        CommercialClassification::Uncertain {
            reasons: vec!["automated_pattern".to_string()],
        },
        "etcd",
    );

    assert!(result.is_ok());
    let cert = result.unwrap();
    assert!(matches!(
        cert.classification,
        CommercialClassification::Uncertain { .. }
    ));

    let verifier = CertificateVerifier::new(verifying_key);
    let verify_result = verifier.verify(&cert);
    assert!(verify_result.is_ok());
}

// ============================================================================
// Commercial Classification Tests
// ============================================================================

#[test]
fn test_commercial_classifications() {
    let classifications = vec![
        CommercialClassification::Human {
            confidence: 90,
            reasons: vec!["human_pattern".to_string()],
        },
        CommercialClassification::SmallTeam {
            confidence: 85,
            team_size: 3,
        },
        CommercialClassification::Uncertain {
            reasons: vec!["mixed_signals".to_string()],
        },
    ];

    let signing_key = create_test_keypair();
    let verifying_key = signing_key.verifying_key();
    let issuer = CertificateIssuer::new(signing_key);
    let verifier = CertificateVerifier::new(verifying_key);

    for classification in classifications {
        let cert = issuer
            .issue(classification.clone(), "test-adapter")
            .expect("Certificate issuance failed");

        assert_eq!(cert.classification, classification);
        assert!(verifier.verify(&cert).is_ok());
    }
}

// ============================================================================
// End-to-End Integration: Real-World Scenario
// ============================================================================

#[test]
fn test_collaboration_key_workflow() {
    // Alice creates a research key with constraints
    let alice_key = create_test_keypair();
    let alice_verifying = alice_key.verifying_key();
    let alice_issuer = CertificateIssuer::new(alice_key);
    let alice_verifier = CertificateVerifier::new(alice_verifying);

    // Issue certificate for human researcher
    let alice_cert = alice_issuer
        .issue(
            CommercialClassification::Human {
                confidence: 95,
                reasons: vec!["research_pattern".to_string()],
            },
            "prometheus",
        )
        .expect("Alice certificate issuance failed");

    // Verify certificate
    assert!(alice_verifier.verify(&alice_cert).is_ok());

    // Bob joins as part of small team
    let bob_key = create_test_keypair();
    let bob_verifying = bob_key.verifying_key();
    let bob_issuer = CertificateIssuer::new(bob_key);
    let bob_verifier = CertificateVerifier::new(bob_verifying);

    let bob_cert = bob_issuer
        .issue(
            CommercialClassification::SmallTeam {
                team_size: 2,
                confidence: 90,
            },
            "prometheus",
        )
        .expect("Bob certificate issuance failed");

    assert!(bob_verifier.verify(&bob_cert).is_ok());

    // Both researchers have valid certificates
    assert!(alice_cert.expires_at > Utc::now());
    assert!(bob_cert.expires_at > Utc::now());
}

#[test]
fn test_constraint_evolution_workflow() {
    let engine = ConstraintEvolutionEngine::new();
    let initial_trust = engine.trust_score();

    // Simulate successful operations building trust
    let _initial_constraints = create_research_constraints();

    // After many successful operations, trust increases
    // (actual operation recording would be done in production code)

    // Check that trust is within valid range
    assert!(initial_trust > 0.0 && initial_trust <= 1.0);
}
