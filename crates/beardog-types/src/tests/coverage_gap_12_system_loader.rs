// SPDX-License-Identifier: AGPL-3.0-or-later

//! Split from coverage_gap_tests_12: migrations, system/env config, crypto, loaders, tunnel.

#[cfg(test)]
mod monitoring_migration_tests {
    use crate::canonical::config::monitoring_migration::*;
    use std::collections::HashMap;

    #[test]
    fn test_monitoring_migration_service_default() {
        let s = MonitoringMigrationService::default();
        let _ = &s;
    }

    #[test]
    fn test_migrate_configuration_monitoring() {
        let s = MonitoringMigrationService::default();
        let legacy = vec![LegacyMonitoringConfig::ConfigurationMonitoring {
            metrics: None,
            tracing: None,
            logging: None,
            health: None,
            alerting: None,
        }];
        let result = s.migrate_monitoring_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migrate_production_monitoring() {
        let s = MonitoringMigrationService::default();
        let legacy = vec![LegacyMonitoringConfig::ProductionMonitoring {
            observability: HashMap::new(),
            performance: None,
            security: None,
        }];
        let result = s.migrate_monitoring_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migrate_beardog_monitoring() {
        let s = MonitoringMigrationService::default();
        let legacy = vec![LegacyMonitoringConfig::BeardogMonitoring {
            config: HashMap::new(),
        }];
        let result = s.migrate_monitoring_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migrate_provider_monitoring() {
        let s = MonitoringMigrationService::default();
        let legacy = vec![LegacyMonitoringConfig::ProviderMonitoring {
            provider_configs: vec![],
        }];
        let result = s.migrate_monitoring_configs(legacy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_monitoring_migration_summary() {
        let report = MonitoringMigrationReport {
            legacy_configs_processed: 2,
            successful_migrations: vec!["metrics".into()],
            warnings: vec![],
            errors: vec![],
            migrated_at: chrono::Utc::now(),
        };
        let summary = MonitoringMigrationService::create_monitoring_migration_summary(&report);
        assert!(!summary.is_empty());
    }
}

#[cfg(test)]
mod system_config_tests {
    use crate::canonical::config::domains::system::*;
    use crate::canonical::config::r#trait::BearDogConfig;

    #[test]
    fn test_system_domain_config_default() {
        let c = SystemDomainConfig::default();
        let _ = &c.logging;
        let _ = &c.application;
    }

    #[test]
    fn test_system_domain_from_env() {
        let _ = SystemDomainConfig::from_env();
    }

    #[test]
    fn test_system_domain_validate() {
        let c = SystemDomainConfig::default();
        assert!(c.validate().is_ok());
    }

    #[test]
    fn test_logging_config_default() {
        let c = LoggingConfig::default();
        let _ = &c.level;
        let _ = &c.format;
    }

    #[test]
    fn test_log_level_variants() {
        let _ = LogLevel::Trace;
        let _ = LogLevel::Debug;
        let _ = LogLevel::Info;
        let _ = LogLevel::Warn;
        let _ = LogLevel::Error;
    }

    #[test]
    fn test_log_format_variants() {
        let _ = LogFormat::Text;
        let _ = LogFormat::Json;
        let _ = LogFormat::Compact;
    }

    #[test]
    fn test_application_config_default() {
        let c = ApplicationConfig::default();
        let _ = &*c.name;
    }

    #[test]
    fn test_threading_config_default() {
        let c = ThreadingConfig::default();
        assert!(c.worker_threads > 0);
    }

    #[test]
    fn test_resource_config_default() {
        let c = ResourceConfig::default();
        assert!(c.max_connections > 0);
    }
}

#[cfg(test)]
mod environment_config_tests {
    use crate::canonical::config::production::environment::*;

    #[test]
    fn test_environment_config_default() {
        let c = EnvironmentConfig::default();
        let _ = &c;
    }

    #[test]
    fn test_environment_validation_from_env() {
        let c = EnvironmentValidation::from_env();
        let _ = &c;
    }

    #[test]
    fn test_environment_validation_with_defaults() {
        let c = EnvironmentValidation::with_defaults();
        let _ = &c;
    }

    #[test]
    fn test_modern_secrets_config_default() {
        let c = ModernSecretsConfig::default();
        let _ = &c;
    }
}

#[cfg(test)]
mod config_utils_methods_tests {
    use crate::canonical::config::utils::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_standard_config_paths() {
        let paths = UnifiedConfigUtils::get_standard_config_paths("beardog");
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_find_config_file() {
        // May or may not find a file, just test the method runs
        let _ = UnifiedConfigUtils::find_config_file("beardog");
    }

    #[test]
    fn test_create_default_config() {
        use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;
        let config = ConsolidatedMonitoringConfig::default();
        let dir = std::env::temp_dir().join("beardog_test_create_default");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("default.json");
        let result = UnifiedConfigUtils::create_default_config(config, &path);
        let _ = result;
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_and_load_config() {
        use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;
        let config = ConsolidatedMonitoringConfig::default();
        let dir = std::env::temp_dir().join("beardog_test_config_utils");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_config.json");

        let save_result = UnifiedConfigUtils::save_to_file(&config, &path);
        if save_result.is_ok() {
            let load_result =
                UnifiedConfigUtils::load_from_file::<ConsolidatedMonitoringConfig, _>(&path);
            let _ = load_result;
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_validate_config_file() {
        use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;
        let config = ConsolidatedMonitoringConfig::default();
        let dir = std::env::temp_dir().join("beardog_test_validate_cfg");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("validate.json");
        if UnifiedConfigUtils::save_to_file(&config, &path).is_ok() {
            let result = UnifiedConfigUtils::validate_config_file(&path);
            let _ = result;
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_merge_configs() {
        use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;
        let c1 = ConsolidatedMonitoringConfig::default();
        let c2 = ConsolidatedMonitoringConfig::default();
        let merged = UnifiedConfigUtils::merge_configs(c1, c2);
        assert!(merged.is_ok());
    }

    #[test]
    fn test_get_performance_metrics() {
        let metrics = UnifiedConfigUtils::get_performance_metrics();
        assert!(metrics.consolidation_benefit > 0.0);
    }
}

#[cfg(test)]
mod crypto_config_tests {
    use crate::canonical::crypto::*;

    #[test]
    fn test_crypto_key_pair_new() {
        let kp = CryptoKeyPair::new(vec![1, 2, 3], vec![4, 5, 6], KeyPairAlgorithm::Ed25519);
        assert_eq!(kp.public_key, vec![1, 2, 3]);
        assert_eq!(kp.private_key, vec![4, 5, 6]);
        assert!(matches!(kp.algorithm, KeyPairAlgorithm::Ed25519));
    }

    #[test]
    fn test_key_pair_algorithm_variants() {
        let _ = KeyPairAlgorithm::Ed25519;
        let _ = KeyPairAlgorithm::Rsa { bits: 2048 };
        let _ = KeyPairAlgorithm::Rsa { bits: 4096 };
        let _ = KeyPairAlgorithm::Ec {
            curve: "P-256".to_string(),
        };
        let _ = KeyPairAlgorithm::Ec {
            curve: "secp256k1".to_string(),
        };
    }

    #[test]
    fn test_crypto_config_default() {
        let c = CryptoConfig::default();
        let _ = &c;
    }

    #[test]
    fn test_crypto_config_fields() {
        let c = CryptoConfig::default();
        let _ = &c;
    }
}

#[cfg(test)]
mod config_loader_tests {
    use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;
    use crate::canonical::config::r#trait::*;

    #[test]
    fn test_config_loader_from_env_with_prefix() {
        let result =
            ConfigLoader::from_env_with_prefix::<ConsolidatedMonitoringConfig>("BEARDOG_TEST_");
        let _ = result;
    }

    #[test]
    fn test_config_loader_from_toml_file() {
        let path = std::env::temp_dir().join("beardog_test_cfg.toml");
        let result =
            ConfigLoader::from_toml_file::<ConsolidatedMonitoringConfig>(path.to_str().unwrap());
        let _ = result;
    }

    #[test]
    fn test_config_loader_from_json_file() {
        let path = std::env::temp_dir().join("beardog_test_cfg.json");
        let result =
            ConfigLoader::from_json_file::<ConsolidatedMonitoringConfig>(path.to_str().unwrap());
        let _ = result;
    }

    #[test]
    fn test_config_loader_merge_configs() {
        let c1 = ConsolidatedMonitoringConfig::default();
        let c2 = ConsolidatedMonitoringConfig::default();
        let merged = ConfigLoader::merge_configs(vec![c1, c2]);
        assert!(merged.is_ok());
    }

    #[test]
    fn test_config_metadata_creation() {
        let m = ConfigMetadata {
            source: ConfigSource::Default,
            created_at: std::time::SystemTime::now(),
            domain: "test".to_string(),
            version: 1,
            validation_status: ValidationStatus::Unknown,
        };
        assert_eq!(m.domain, "test");
    }

    #[test]
    fn test_config_source_variants() {
        let _ = ConfigSource::Default;
        let _ = ConfigSource::File("test.toml".to_string());
        let _ = ConfigSource::Environment;
        let _ = ConfigSource::Merged;
    }

    #[test]
    fn test_validation_status_variants() {
        let _ = ValidationStatus::Unknown;
        let _ = ValidationStatus::Valid;
        let _ = ValidationStatus::Invalid(vec!["error".to_string()]);
    }
}

#[cfg(test)]
mod performance_config_tests {
    use crate::canonical::config::performance::*;

    #[test]
    fn test_canonical_performance_config_default() {
        let c = CanonicalPerformanceConfig::default();
        let _ = &c;
    }

    #[test]
    fn test_optimization_level_variants() {
        let _ = OptimizationLevel::default();
    }

    #[test]
    fn test_resource_limits_default() {
        let c = ResourceLimits::default();
        let _ = &c;
    }
}

#[cfg(test)]
mod android_config_tests {
    use crate::canonical::hsm::android::*;

    #[test]
    fn test_android_device_info_default() {
        let d = AndroidDeviceInfo::default();
        let _ = &d;
    }

    #[test]
    fn test_device_integrity_default() {
        let d = DeviceIntegrity::default();
        let _ = &d;
    }

    #[test]
    fn test_android_keystore_config_default() {
        let c = AndroidKeystoreConfig::default();
        let _ = &c;
    }

    #[test]
    fn test_strongbox_config_default() {
        let c = StrongBoxConfig::default();
        let _ = &c;
    }

    #[test]
    fn test_biometric_config_default() {
        let c = BiometricConfig::default();
        let _ = &c;
    }
}

#[cfg(test)]
mod create_tunnel_legacy_tests {
    use crate::canonical::hsm_unified::migration::*;

    #[test]
    fn test_create_tunnel_legacy_config_none() {
        let config = create_tunnel_legacy_config(None, None, None);
        match config {
            LegacyHsmConfig::TunnelHsm {
                hardware_config,
                software_config,
                mobile_config,
            } => {
                assert!(hardware_config.is_none());
                assert!(software_config.is_none());
                assert!(mobile_config.is_none());
            }
            _ => panic!("Expected TunnelHsm variant"),
        }
    }

    #[test]
    fn test_create_tunnel_legacy_config_with_values() {
        let hw = serde_json::json!({"type": "hardware"});
        let sw = serde_json::json!({"type": "software"});
        let config = create_tunnel_legacy_config(Some(hw), Some(sw), None);
        match config {
            LegacyHsmConfig::TunnelHsm {
                hardware_config,
                software_config,
                ..
            } => {
                assert!(hardware_config.is_some());
                assert!(software_config.is_some());
            }
            _ => panic!("Expected TunnelHsm variant"),
        }
    }
}
