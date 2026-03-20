// SPDX-License-Identifier: AGPL-3.0-only

//! Split from coverage_gap_tests_12: config trait, genetics constraints, HSM migration.

#[cfg(test)]
mod config_trait_methods_tests {
    use crate::canonical::config::r#trait::validation;

    #[test]
    fn test_validate_range() {
        assert!(validation::validate_range(5, 0, 10, "test").is_ok());
        assert!(validation::validate_range(11, 0, 10, "test").is_err());
    }

    #[test]
    fn test_validate_non_empty_string() {
        assert!(validation::validate_non_empty_string("hello", "test").is_ok());
        assert!(validation::validate_non_empty_string("", "test").is_err());
    }

    #[test]
    fn test_validate_collection_size() {
        let items = vec![1, 2, 3, 4, 5];
        assert!(validation::validate_collection_size(&items, 0, 10, "test").is_ok());
        let big: Vec<i32> = (0..15).collect();
        assert!(validation::validate_collection_size(&big, 0, 10, "test").is_err());
    }

    #[test]
    fn test_validate_duration() {
        use std::time::Duration;
        assert!(
            validation::validate_duration(
                Duration::from_secs(5),
                Duration::from_secs(1),
                Duration::from_secs(10),
                "test"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_percentage() {
        assert!(validation::validate_percentage(0.5, "test").is_ok());
        assert!(validation::validate_percentage(1.5, "test").is_err());
        assert!(validation::validate_percentage(-0.1, "test").is_err());
    }

    #[test]
    fn test_validate_port() {
        assert!(validation::validate_port(8080, "test").is_ok());
        assert!(validation::validate_port(0, "test").is_err());
    }

    #[test]
    fn test_validate_network_address() {
        assert!(validation::validate_network_address("127.0.0.1", "test").is_ok());
        assert!(validation::validate_network_address("", "test").is_err());
    }

    #[test]
    fn test_validate_non_empty_collection() {
        let v = vec![1, 2, 3];
        assert!(validation::validate_non_empty_collection(&v, "test").is_ok());
        let empty: Vec<i32> = vec![];
        assert!(validation::validate_non_empty_collection(&empty, "test").is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validation::validate_url("https://example.com", "test").is_ok());
        assert!(validation::validate_url("", "test").is_err());
    }

    #[test]
    fn test_validate_field_consistency() {
        assert!(
            validation::validate_field_consistency(
                true,
                "field_a",
                true,
                "field_b",
                |a: &bool, b: &bool| *a == *b,
                "fields must match"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_resource_allocation() {
        assert!(validation::validate_resource_allocation(&[(0.5, "cpu"), (0.3, "mem")]).is_ok());
        assert!(validation::validate_resource_allocation(&[(0.8, "cpu"), (0.5, "mem")]).is_err());
    }

    #[test]
    fn test_validate_environment_compatibility() {
        assert!(
            validation::validate_environment_compatibility(
                "development",
                "debug_mode",
                "true",
                false
            )
            .is_ok()
        );
        assert!(
            validation::validate_environment_compatibility(
                "production",
                "debug_mode",
                "true",
                false
            )
            .is_err()
        );
    }
}

#[cfg(test)]
mod genetics_constraints_methods_tests {
    use crate::genetics_constraints::*;

    #[test]
    fn test_scope_constraint_unrestricted() {
        let _ = ScopeConstraint::Unrestricted;
    }

    #[test]
    fn test_lifetime_constraint_default() {
        let l = LifetimeConstraint::default();
        let _ = &l;
    }

    #[test]
    fn test_data_access_constraint_default() {
        let d = DataAccessConstraint::default();
        let _ = &d;
    }

    #[test]
    fn test_behavioral_constraint_default() {
        let b = BehavioralConstraint::default();
        let _ = &b;
    }

    #[test]
    fn test_usage_pattern() {
        let u = UsagePattern {
            time_pattern: Some("business hours".to_string()),
            location_pattern: None,
            frequency_threshold: Some(100),
        };
        assert_eq!(u.frequency_threshold, Some(100));
    }

    #[test]
    fn test_network_constraint() {
        let n = NetworkConstraint {
            allowed_ssids: vec!["office-wifi".to_string()],
            require_vpn: Some("corporate".to_string()),
            geo_fence: None,
        };
        assert_eq!(n.allowed_ssids.len(), 1);
    }

    #[test]
    fn test_compute_quota() {
        let c = ComputeQuota {
            max_hours: 100.0,
            max_memory_bytes: 1024 * 1024,
            max_cpu_percent: 80,
            current_usage: ComputeUsage {
                hours_used: 0.0,
                memory_used: 0,
                last_updated: None,
            },
        };
        assert_eq!(c.max_cpu_percent, 80);
    }

    #[test]
    fn test_compute_usage() {
        let u = ComputeUsage {
            hours_used: 5.0,
            memory_used: 512,
            last_updated: None,
        };
        assert_eq!(u.hours_used, 5.0);
    }

    #[test]
    fn test_constraint_signature() {
        let s = ConstraintSignature {
            constraints_hash: [0u8; 32],
            signature: vec![1, 2, 3],
            signed_at: chrono::Utc::now(),
            public_key: vec![4, 5, 6],
        };
        assert!(!s.signature.is_empty());
    }

    #[test]
    fn test_key_constraints_default() {
        let k = KeyConstraints::default();
        let _ = k.hash();
        let _ = k.description();
    }

    #[test]
    fn test_key_operation_read() {
        let op = KeyOperation::Read {
            path: "/data/file".to_string(),
            project: None,
        };
        let _ = &op;
    }

    #[test]
    fn test_key_operation_write() {
        let op = KeyOperation::Write {
            path: "/data/file".to_string(),
            size_bytes: 1024,
            project: None,
        };
        let _ = &op;
    }

    #[test]
    fn test_key_operation_delete() {
        let op = KeyOperation::Delete {
            path: "/data/file".to_string(),
        };
        let _ = &op;
    }

    #[test]
    fn test_key_operation_rpc_call() {
        let op = KeyOperation::RpcCall {
            target_service: "service-1".to_string(),
            method: "get".to_string(),
            project: Some("proj-1".to_string()),
        };
        let _ = &op;
    }

    #[test]
    fn test_key_operation_compute_allocation() {
        let op = KeyOperation::ComputeAllocation {
            hours: 10.0,
            memory_bytes: 1024 * 1024,
        };
        let _ = &op;
    }
}

#[cfg(test)]
mod hsm_migration_tests {
    use crate::canonical::hsm_unified::migration::*;
    use std::collections::HashMap;

    #[test]
    fn test_hsm_migration_service_default() {
        let s = HsmMigrationService::default();
        // Can't access private options, just verify it was created
        let _ = &s;
    }

    #[test]
    fn test_hsm_migration_service_new() {
        let opts = MigrationOptions::default();
        let s = HsmMigrationService::new(opts);
        let _ = &s;
    }

    #[test]
    fn test_migrate_tunnel_hsm() {
        let s = HsmMigrationService::default();
        let legacy = vec![LegacyHsmConfig::TunnelHsm {
            hardware_config: None,
            software_config: None,
            mobile_config: None,
        }];
        let result = s.migrate_hsm_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migrate_configuration_hsm() {
        let s = HsmMigrationService::default();
        let legacy = vec![LegacyHsmConfig::ConfigurationHsm {
            providers: vec![],
            monitoring: None,
            performance: None,
        }];
        let result = s.migrate_hsm_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migrate_zero_cost_hsm() {
        let s = HsmMigrationService::default();
        let legacy = vec![LegacyHsmConfig::ZeroCostHsm {
            manager_config: HashMap::new(),
        }];
        let result = s.migrate_hsm_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_migration_summary() {
        let report = MigrationReport {
            legacy_configs_processed: 1,
            successful_migrations: vec!["tunnel".into()],
            warnings: vec![],
            errors: vec![],
            migrated_at: chrono::Utc::now(),
        };
        let summary = HsmMigrationService::create_migration_summary(&report);
        assert!(!summary.is_empty());
    }

    #[test]
    fn test_migration_options_default() {
        let o = MigrationOptions::default();
        assert!(o.create_backup);
        assert!(o.validate_after_migration);
    }

    #[test]
    fn test_migration_report_fields() {
        let r = MigrationReport {
            legacy_configs_processed: 5,
            successful_migrations: vec!["a".into(), "b".into()],
            warnings: vec![MigrationWarning {
                config_type: "tunnel".into(),
                message: "deprecated field".into(),
                recommendation: Some("use new field".into()),
            }],
            errors: vec![MigrationError {
                config_type: "cloud".into(),
                error: "connection failed".into(),
                resolution: "check credentials".into(),
            }],
            migrated_at: chrono::Utc::now(),
        };
        assert_eq!(r.legacy_configs_processed, 5);
        assert_eq!(r.warnings.len(), 1);
        assert_eq!(r.errors.len(), 1);
    }
}
