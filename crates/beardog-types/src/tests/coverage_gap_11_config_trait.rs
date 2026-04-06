// SPDX-License-Identifier: AGPL-3.0-or-later

//! Split from `coverage_gap_tests_11`: config trait validation/loaders/types, unified & system/security domains, config utils.

mod config_trait_validation {
    use crate::canonical::config::r#trait::validation;

    #[test]
    fn test_validate_range_ok() {
        assert!(validation::validate_range(5, 0, 10, "test").is_ok());
    }

    #[test]
    fn test_validate_range_err() {
        assert!(validation::validate_range(15, 0, 10, "test").is_err());
    }

    #[test]
    fn test_validate_non_empty_string_ok() {
        assert!(validation::validate_non_empty_string("hello", "test").is_ok());
    }

    #[test]
    fn test_validate_non_empty_string_err() {
        assert!(validation::validate_non_empty_string("", "test").is_err());
    }

    #[test]
    fn test_validate_collection_size_ok() {
        let v = vec![1, 2, 3];
        assert!(validation::validate_collection_size(&v, 1, 10, "test").is_ok());
    }

    #[test]
    fn test_validate_collection_size_err() {
        let v: Vec<i32> = vec![];
        assert!(validation::validate_collection_size(&v, 1, 10, "test").is_err());
    }

    #[test]
    fn test_validate_duration_ok() {
        let d = std::time::Duration::from_secs(5);
        assert!(
            validation::validate_duration(
                d,
                std::time::Duration::from_secs(1),
                std::time::Duration::from_secs(60),
                "test"
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_duration_err() {
        let d = std::time::Duration::from_secs(120);
        assert!(
            validation::validate_duration(
                d,
                std::time::Duration::from_secs(1),
                std::time::Duration::from_secs(60),
                "test"
            )
            .is_err()
        );
    }

    #[test]
    fn test_validate_percentage_ok() {
        assert!(validation::validate_percentage(0.5, "test").is_ok());
    }

    #[test]
    fn test_validate_percentage_err() {
        assert!(validation::validate_percentage(1.5, "test").is_err());
        assert!(validation::validate_percentage(-0.1, "test").is_err());
    }

    #[test]
    fn test_validate_port_ok() {
        assert!(validation::validate_port(8080, "test").is_ok());
    }

    #[test]
    fn test_validate_port_err() {
        assert!(validation::validate_port(0, "test").is_err());
    }

    #[test]
    fn test_validate_network_address_ok() {
        assert!(validation::validate_network_address("127.0.0.1", "test").is_ok());
        assert!(validation::validate_network_address("localhost", "test").is_ok());
    }

    #[test]
    fn test_validate_network_address_err() {
        assert!(validation::validate_network_address("", "test").is_err());
    }

    #[test]
    fn test_validate_non_empty_collection_ok() {
        let v = vec![1];
        assert!(validation::validate_non_empty_collection(&v, "test").is_ok());
    }

    #[test]
    fn test_validate_non_empty_collection_err() {
        let v: Vec<i32> = vec![];
        assert!(validation::validate_non_empty_collection(&v, "test").is_err());
    }

    #[test]
    fn test_validate_url_ok() {
        assert!(validation::validate_url("https://example.com", "test").is_ok());
    }

    #[test]
    fn test_validate_url_err() {
        assert!(validation::validate_url("", "test").is_err());
    }

    #[test]
    fn test_validate_resource_allocation_ok() {
        let allocs = [(0.3, "cpu"), (0.5, "mem"), (0.2, "io")];
        assert!(validation::validate_resource_allocation(&allocs).is_ok());
    }

    #[test]
    fn test_validate_resource_allocation_err() {
        let allocs = [(0.6, "cpu"), (0.6, "mem")];
        assert!(validation::validate_resource_allocation(&allocs).is_err());
    }

    #[test]
    fn test_validate_environment_compatibility_ok() {
        assert!(
            validation::validate_environment_compatibility(
                "development",
                "debug_mode",
                "true",
                false,
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_environment_compatibility_err() {
        assert!(
            validation::validate_environment_compatibility(
                "production",
                "debug_mode",
                "true",
                false,
            )
            .is_err()
        );
    }
}

mod config_trait_loader {
    use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;
    use crate::canonical::config::r#trait::ConfigLoader;

    #[test]
    fn test_config_loader_from_env() {
        let result = ConfigLoader::from_env_with_prefix::<ConsolidatedMonitoringConfig>(
            "BEARDOG_TEST_NONEXIST_",
        );
        let _ = result;
    }

    #[test]
    fn test_config_loader_from_toml_nonexistent() {
        let result =
            ConfigLoader::from_toml_file::<ConsolidatedMonitoringConfig>("/nonexistent/path.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_loader_from_json_nonexistent() {
        let result =
            ConfigLoader::from_json_file::<ConsolidatedMonitoringConfig>("/nonexistent/path.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_loader_merge_configs() {
        let c1 = ConsolidatedMonitoringConfig::default();
        let c2 = ConsolidatedMonitoringConfig::default();
        let result = ConfigLoader::merge_configs(vec![c1, c2]);
        assert!(result.is_ok());
    }
}

mod config_trait_types {
    use crate::canonical::config::r#trait::*;

    #[test]
    fn test_config_metadata() {
        let m = ConfigMetadata {
            domain: "test".to_string(),
            version: 1,
            created_at: std::time::SystemTime::now(),
            source: ConfigSource::Default,
            validation_status: ValidationStatus::Valid,
        };
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_config_source_variants() {
        let sources = [
            ConfigSource::Default,
            ConfigSource::File("test.toml".to_string()),
            ConfigSource::Environment,
            ConfigSource::Programmatic,
        ];
        for s in &sources {
            let _ = format!("{s:?}");
        }
    }

    #[test]
    fn test_validation_status_variants() {
        let statuses = [
            ValidationStatus::Valid,
            ValidationStatus::Invalid(vec!["test".to_string()]),
            ValidationStatus::Unknown,
        ];
        for s in &statuses {
            let _ = format!("{s:?}");
        }
    }
}

// ===========================================================================
// canonical/config/unified/simplified.rs - 104 uncov (6 Default impls)
// ===========================================================================
mod unified_simplified_defaults {
    use crate::canonical::config::unified::simplified::*;

    #[test]
    fn test_network_settings_default() {
        let s = NetworkSettings::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_security_settings_default() {
        let s = SecuritySettings::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_database_settings_default() {
        let s = DatabaseSettings::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_monitoring_settings_default() {
        let s = MonitoringSettings::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_performance_settings_default() {
        let s = PerformanceSettings::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_simplified_beardog_config_default() {
        let c = SimplifiedBearDogConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/system.rs - 81 uncov (9 Default impls)
// ===========================================================================
mod system_domain_defaults {
    use crate::canonical::config::domains::system::*;

    #[test]
    fn test_logging_config_default() {
        let c = LoggingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_log_level_default() {
        let l = LogLevel::default();
        let _ = format!("{l:?}");
    }

    #[test]
    fn test_log_format_default() {
        let f = LogFormat::default();
        let _ = format!("{f:?}");
    }

    #[test]
    fn test_log_rotation_config_default() {
        let c = LogRotationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_system_domain_config_default() {
        let c = SystemDomainConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_application_config_default() {
        let c = ApplicationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_threading_config_default() {
        let c = ThreadingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_resource_config_default() {
        let c = ResourceConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_environment_config_default() {
        let c = EnvironmentConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/security/mod.rs - 64 uncov
// ===========================================================================
mod security_domain_mod_methods {
    use crate::canonical::config::domains::security::*;

    #[test]
    fn test_consolidated_security_config_validate() {
        let c = ConsolidatedSecurityConfiguration::default();
        let _ = c.validate();
    }

    #[test]
    fn test_consolidated_security_config_development() {
        let c = ConsolidatedSecurityConfiguration::development();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_consolidated_security_config_production() {
        let c = ConsolidatedSecurityConfiguration::production();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/utils.rs - 239 uncov (more method tests)
// ===========================================================================
mod config_utils_extra_methods {
    use crate::canonical::config::utils::*;

    #[test]
    fn test_validate_config_file_nonexistent() {
        assert!(!UnifiedConfigUtils::validate_config_file(
            "/nonexistent/file.toml"
        ));
    }

    #[test]
    fn test_find_config_file() {
        // Will likely return None since no standard config files exist
        let result = UnifiedConfigUtils::find_config_file("beardog_test_nonexist_xyz");
        let _ = result;
    }

    #[test]
    fn test_auto_load_config() {
        // Will likely fail since no config files exist
        let result =
            UnifiedConfigUtils::auto_load_config::<serde_json::Value>("beardog_test_nonexist_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_default_config() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let dir = std::env::temp_dir().join(format!("beardog_cdc_test_{ts}"));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("default.toml");

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct Cfg {
            name: String,
        }

        let cfg = Cfg {
            name: "default".to_string(),
        };
        let result = UnifiedConfigUtils::create_default_config(cfg, &path);
        assert!(result.is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_with_fallback() {
        let result = UnifiedConfigUtils::load_with_fallback::<serde_json::Value>(
            "/nonexistent/primary.toml",
            &["/nonexistent/fallback.toml"],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_shared_config_via_utils() {
        let val = UnifiedConfigUtils::get_shared_config::<String, _>("test_scm_key_11", || {
            "value11".to_string()
        });
        assert_eq!(*val, "value11");
        let stats = UnifiedConfigUtils::get_shared_config_stats();
        let _ = format!("{stats:?}");
    }

    #[test]
    fn test_serialize_deserialize_arc_str() {
        use serde::{Deserialize, Serialize};
        use std::sync::Arc;

        #[derive(Serialize, Deserialize)]
        struct Wrapper {
            #[serde(
                serialize_with = "serialize_arc_str",
                deserialize_with = "deserialize_arc_str"
            )]
            value: Arc<str>,
        }

        let w = Wrapper {
            value: Arc::from("test"),
        };
        let json = serde_json::to_string(&w).unwrap();
        let w2: Wrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(&*w2.value, "test");
    }
}

// ===========================================================================
// canonical/monitoring/metrics.rs - 50 uncov
// ===========================================================================
