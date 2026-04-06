// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage: constraint types (`KeyOperation`, `SignedConstraints`, path matching).

// ═══════════════════════════════════════════════════════════════════
// constraints/types.rs - Additional KeyOperation coverage
// ═══════════════════════════════════════════════════════════════════

mod types_tests {
    use crate::constraints::types::OperationType;
    use crate::constraints::*;

    #[test]
    fn test_key_operation_types() {
        assert_eq!(
            KeyOperation::Sign { domain: None }.operation_type(),
            OperationType::Sign
        );
        assert_eq!(
            KeyOperation::Decrypt { domain: None }.operation_type(),
            OperationType::Decrypt
        );
        assert_eq!(
            KeyOperation::Encrypt { recipients: vec![] }.operation_type(),
            OperationType::Encrypt
        );
        assert_eq!(
            KeyOperation::Delete {
                path: "x".to_string()
            }
            .operation_type(),
            OperationType::Delete
        );
        assert_eq!(
            KeyOperation::Modify {
                path: "x".to_string()
            }
            .operation_type(),
            OperationType::Modify
        );
        assert_eq!(
            KeyOperation::Read {
                path: "x".to_string()
            }
            .operation_type(),
            OperationType::Read
        );
        assert_eq!(
            KeyOperation::Delegate {
                to_key_id: "k".to_string(),
                authority: Box::new(KeyOperation::Sign { domain: None })
            }
            .operation_type(),
            OperationType::Delegate
        );
        assert_eq!(
            KeyOperation::Mix {
                with_key_ids: vec![]
            }
            .operation_type(),
            OperationType::Mix
        );
    }

    #[test]
    fn test_key_operation_domain() {
        assert_eq!(
            KeyOperation::Sign {
                domain: Some("test".to_string())
            }
            .domain(),
            Some("test")
        );
        assert_eq!(
            KeyOperation::Decrypt {
                domain: Some("dec".to_string())
            }
            .domain(),
            Some("dec")
        );
        assert_eq!(KeyOperation::Sign { domain: None }.domain(), None);
        assert_eq!(
            KeyOperation::Delete {
                path: "x".to_string()
            }
            .domain(),
            None
        );
    }

    #[test]
    fn test_key_operation_path() {
        assert_eq!(
            KeyOperation::Delete {
                path: "a".to_string()
            }
            .path(),
            Some("a")
        );
        assert_eq!(
            KeyOperation::Modify {
                path: "b".to_string()
            }
            .path(),
            Some("b")
        );
        assert_eq!(
            KeyOperation::Read {
                path: "c".to_string()
            }
            .path(),
            Some("c")
        );
        assert_eq!(KeyOperation::Sign { domain: None }.path(), None);
    }

    #[test]
    fn test_signed_constraints_compute_hash() {
        let constraints = KeyConstraints::default();
        let hash1 = SignedConstraints::compute_hash(&constraints);
        let hash2 = SignedConstraints::compute_hash(&constraints);
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_signed_constraints_different_hash() {
        let c1 = KeyConstraints::default();
        let c2 = KeyConstraints {
            scope: ScopeConstraint::Unrestricted,
            lifetime: LifetimeConstraint::Permanent,
            data_access: DataAccessConstraint {
                cannot_delete: vec!["test".to_string()],
                ..Default::default()
            },
            co_signers: vec![],
            behavioral: BehavioralConstraint::default(),
        };
        let hash1 = SignedConstraints::compute_hash(&c1);
        let hash2 = SignedConstraints::compute_hash(&c2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_data_access_constraint_path_matching() {
        // Exact match
        assert!(DataAccessConstraint::path_matches(
            "secrets/api_key.txt",
            &["secrets/api_key.txt".to_string()]
        ));

        // Wildcard match
        assert!(DataAccessConstraint::path_matches(
            "raw_data/temperature.nc",
            &["raw_data/*".to_string()]
        ));

        // No match
        assert!(!DataAccessConstraint::path_matches(
            "other/file.txt",
            &["raw_data/*".to_string()]
        ));

        // Multiple patterns
        assert!(DataAccessConstraint::path_matches(
            "secrets/api_key.txt",
            &["raw_data/*".to_string(), "secrets/api_key.txt".to_string()]
        ));
    }
}
