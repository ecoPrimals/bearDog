//! Coverage gap tests 11 - HSM, monitoring, and config subsystems
//! Targets top uncovered files: hsm/mod, hsm/keys, hsm/capabilities,
//! monitoring/core, monitoring/health, config/trait validation,
//! config/unified/simplified, config/domains/system, config/domains/security/mod

// ===========================================================================
// canonical/config/hsm/mod.rs - 159 uncov (12 Default impls)
// ===========================================================================
mod hsm_mod_defaults {
    use crate::canonical::config::hsm::*;

    #[test]
    fn test_unified_hsm_config_default() {
        let c = UnifiedHsmConfig::default();
        let _ = format!("{c:?}");
        // Test HsmConfigValidation trait
        let _ = c.validate();
        assert!(c.is_compatible_with(1));
    }

    #[test]
    fn test_hsm_retry_policy_default() {
        use crate::canonical::traits::RetryStrategy;
        let c = HsmRetryPolicy::default();
        let _ = format!("{c:?}");
        // Test RetryStrategy trait methods
        assert!(c.max_attempts() > 0);
        let delay = c.delay_for_attempt(0);
        assert!(delay.as_millis() > 0);
        assert!(c.backoff_multiplier() > 1.0);
        let total = c.total_delay(3);
        assert!(total.as_millis() > 0);
        assert!(!c.is_limit_reached(0));
        assert!(c.is_limit_reached(100));
    }

    #[test]
    fn test_hsm_retry_policy_should_retry_error() {
        use crate::canonical::traits::RetryStrategy;
        let c = HsmRetryPolicy::default();
        // Timeout errors should be retried
        let timeout_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "connection timed out");
        assert!(c.should_retry_error(&timeout_err));
        // Connection errors should be retried
        let conn_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "connection refused");
        assert!(c.should_retry_error(&conn_err));
        // Auth errors should not be retried
        let auth_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "unauthorized access");
        assert!(!c.should_retry_error(&auth_err));
    }

    #[test]
    fn test_hsm_retry_policy_disabled() {
        use crate::canonical::traits::RetryStrategy;
        let mut c = HsmRetryPolicy::default();
        c.enabled = false;
        // When disabled, total_delay should be 0
        assert_eq!(c.total_delay(5).as_millis(), 0);
        // When disabled, is_limit_reached should be true
        assert!(c.is_limit_reached(0));
        // When disabled, should_retry_error should be false
        let err = std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout");
        assert!(!c.should_retry_error(&err));
    }

    #[test]
    fn test_hsm_tier_management_config_default() {
        let c = HsmTierManagementConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_compliance_config_default() {
        let c = HsmComplianceConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_monitoring_config_default() {
        let c = HsmMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_alerting_config_default() {
        let c = HsmAlertingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_health_check_config_default() {
        let c = HsmHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_integration_config_default() {
        let c = HsmIntegrationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_load_balancing_config_default() {
        let c = LoadBalancingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_failover_config_default() {
        let c = FailoverConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_session_management_config_default() {
        let c = SessionManagementConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_backup_config_default() {
        let c = HsmBackupConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/hsm/keys.rs - 136 uncov (7 Default impls)
// ===========================================================================
mod hsm_keys_defaults {
    use crate::canonical::hsm::keys::*;

    #[test]
    fn test_hsm_key_default() {
        let k = HsmKey::default();
        let _ = format!("{k:?}");
    }

    #[test]
    fn test_key_material_default() {
        let m = KeyMaterial::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_key_metadata_default() {
        let m = KeyMetadata::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_key_health_default() {
        let h = KeyHealth::default();
        let _ = format!("{h:?}");
    }

    #[test]
    fn test_encryption_info_default() {
        let e = EncryptionInfo::default();
        let _ = format!("{e:?}");
    }

    #[test]
    fn test_backup_info_default() {
        let b = BackupInfo::default();
        let _ = format!("{b:?}");
    }

    #[test]
    fn test_key_lifecycle_state_default() {
        let s = KeyLifecycleState::default();
        let _ = format!("{s:?}");
    }
}

// ===========================================================================
// canonical/hsm/capabilities.rs - 107 uncov (9 Default impls)
// ===========================================================================
mod hsm_capabilities_defaults {
    use crate::canonical::hsm::capabilities::*;

    #[test]
    fn test_hsm_capabilities_default() {
        let c = HsmCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_generation_capabilities_default() {
        let c = KeyGenerationCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_management_capabilities_default() {
        let c = KeyManagementCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_cryptographic_capabilities_default() {
        let c = CryptographicCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_capabilities_default() {
        let c = SecurityCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_performance_capabilities_default() {
        let c = PerformanceCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_advanced_feature_capabilities_default() {
        let c = AdvancedFeatureCapabilities::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_performance_requirements_default() {
        let c = PerformanceRequirements::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_api_support_capabilities_default() {
        let c = ApiSupportCapabilities::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/core.rs - 152 uncov (5 Default impls)
// ===========================================================================
mod monitoring_core_defaults {
    use crate::canonical::monitoring::core::*;

    #[test]
    fn test_core_monitoring_config_default() {
        let c = CoreMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_storage_backend_default() {
        let b = StorageBackend::default();
        let _ = format!("{b:?}");
    }

    #[test]
    fn test_retention_policy_default() {
        let p = RetentionPolicy::default();
        let _ = format!("{p:?}");
    }

    #[test]
    fn test_batch_config_default() {
        let c = BatchConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_retry_policy_default() {
        let p = RetryPolicy::default();
        let _ = format!("{p:?}");
    }
}

// ===========================================================================
// canonical/monitoring/health.rs - 93 uncov (8 Default impls)
// ===========================================================================
mod monitoring_health_defaults {
    use crate::canonical::monitoring::health::*;

    #[test]
    fn test_unified_health_config_default() {
        let c = UnifiedHealthConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_http_health_check_config_default() {
        let c = HttpHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_tcp_health_check_config_default() {
        let c = TcpHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_database_health_check_config_default() {
        let c = DatabaseHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_service_health_check_config_default() {
        let c = ServiceHealthCheckConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_monitoring_config_default() {
        let c = HealthMonitoringConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_alerting_config_default() {
        let c = HealthAlertingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_health_recovery_config_default() {
        let c = HealthRecoveryConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/trait.rs - 111 uncov (validation functions)
// ===========================================================================
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
        assert!(validation::validate_duration(
            d,
            std::time::Duration::from_secs(1),
            std::time::Duration::from_secs(60),
            "test"
        ).is_ok());
    }

    #[test]
    fn test_validate_duration_err() {
        let d = std::time::Duration::from_secs(120);
        assert!(validation::validate_duration(
            d,
            std::time::Duration::from_secs(1),
            std::time::Duration::from_secs(60),
            "test"
        ).is_err());
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
        assert!(validation::validate_environment_compatibility(
            "development",
            "debug_mode",
            "true",
            false,
        ).is_ok());
    }

    #[test]
    fn test_validate_environment_compatibility_err() {
        assert!(validation::validate_environment_compatibility(
            "production",
            "debug_mode",
            "true",
            false,
        ).is_err());
    }
}

mod config_trait_loader {
    use crate::canonical::config::r#trait::ConfigLoader;
    use crate::canonical::config::domains::monitoring_config::ConsolidatedMonitoringConfig;

    #[test]
    fn test_config_loader_from_env() {
        let result = ConfigLoader::from_env_with_prefix::<ConsolidatedMonitoringConfig>("BEARDOG_TEST_NONEXIST_");
        let _ = result;
    }

    #[test]
    fn test_config_loader_from_toml_nonexistent() {
        let result = ConfigLoader::from_toml_file::<ConsolidatedMonitoringConfig>("/nonexistent/path.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_loader_from_json_nonexistent() {
        let result = ConfigLoader::from_json_file::<ConsolidatedMonitoringConfig>("/nonexistent/path.json");
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
        assert!(!UnifiedConfigUtils::validate_config_file("/nonexistent/file.toml"));
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
        let result = UnifiedConfigUtils::auto_load_config::<serde_json::Value>("beardog_test_nonexist_xyz");
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
        struct Cfg { name: String }

        let cfg = Cfg { name: "default".to_string() };
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
        let val = UnifiedConfigUtils::get_shared_config::<String, _>("test_scm_key_11", || "value11".to_string());
        assert_eq!(*val, "value11");
        let stats = UnifiedConfigUtils::get_shared_config_stats();
        let _ = format!("{stats:?}");
    }

    #[test]
    fn test_serialize_deserialize_arc_str() {
        use serde::{Serialize, Deserialize};
        use std::sync::Arc;

        #[derive(Serialize, Deserialize)]
        struct Wrapper {
            #[serde(serialize_with = "serialize_arc_str", deserialize_with = "deserialize_arc_str")]
            value: Arc<str>,
        }

        let w = Wrapper { value: Arc::from("test") };
        let json = serde_json::to_string(&w).unwrap();
        let w2: Wrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(&*w2.value, "test");
    }
}

// ===========================================================================
// canonical/monitoring/metrics.rs - 50 uncov
// ===========================================================================
mod monitoring_metrics_defaults {
    use crate::canonical::monitoring::metrics::*;

    #[test]
    fn test_unified_metrics_config_default() {
        let c = UnifiedMetricsConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_counter_config_default() {
        let c = CounterConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_gauge_config_default() {
        let c = GaugeConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_histogram_config_default() {
        let c = HistogramConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_summary_config_default() {
        let c = SummaryConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_metric_collection_config_default() {
        let c = MetricCollectionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_metric_aggregation_config_default() {
        let c = MetricAggregationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_metric_analysis_config_default() {
        let c = MetricAnalysisConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_metric_storage_config_default() {
        let c = MetricStorageConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_metric_export_config_default() {
        let c = MetricExportConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/alerting.rs - 45 uncov
// ===========================================================================
mod monitoring_alerting_defaults {
    use crate::canonical::monitoring::alerting::*;

    #[test]
    fn test_unified_alerting_config_default() {
        let c = UnifiedAlertingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_alert_notification_config_default() {
        let c = AlertNotificationConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_alert_suppression_config_default() {
        let c = AlertSuppressionConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/hsm/status.rs - 57 uncov
// ===========================================================================
mod hsm_status_defaults {
    use crate::canonical::hsm::status::*;

    #[test]
    fn test_hsm_status_default() {
        let s = HsmStatus::default();
        let _ = format!("{s:?}");
    }

    #[test]
    fn test_hsm_status_clone() {
        let s = HsmStatus::default();
        let s2 = s.clone();
        let _ = format!("{s2:?}");
    }
}

// ===========================================================================
// canonical/hsm/android.rs - 50 uncov
// ===========================================================================
mod hsm_android_defaults {
    use crate::canonical::hsm::android::*;

    #[test]
    fn test_android_hsm_config_default() {
        let c = AndroidHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_android_hsm_config_clone() {
        let c = AndroidHsmConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/production/environment.rs - 56 uncov
// ===========================================================================
mod prod_environment_defaults {
    use crate::canonical::config::production::environment::*;

    #[test]
    fn test_environment_config_default() {
        let c = EnvironmentConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_environment_validation_default() {
        let v = EnvironmentValidation::default();
        let _ = format!("{v:?}");
    }
}

mod prod_resources_defaults {
    use crate::canonical::config::production::resources::*;

    #[test]
    fn test_gc_tuning_config_default() {
        let c = GcTuningConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/workflow/engine.rs - 60 uncov
// ===========================================================================
mod workflow_engine_defaults {
    use crate::canonical::config::domains::workflow::engine::*;

    #[test]
    fn test_workflow_engine_config_default() {
        let c = WorkflowEngineConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_queue_config_default() {
        let c = QueueConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/network/client.rs - 57 uncov
// ===========================================================================
mod network_client_defaults {
    use crate::canonical::config::domains::network::client::*;

    #[test]
    fn test_client_configuration_default() {
        let c = ClientConfiguration::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_client_configuration_clone() {
        let c = ClientConfiguration::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/domains/network/mod.rs - 51 uncov
// ===========================================================================
mod network_domain_mod_defaults {
    use crate::canonical::config::domains::network::*;

    #[test]
    fn test_consolidated_network_config_default() {
        let c = ConsolidatedNetworkConfiguration::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/network/connection.rs - 45 uncov
// ===========================================================================
mod network_connection_defaults {
    use crate::canonical::config::domains::network::connection::*;

    #[test]
    fn test_connection_pool_config_default() {
        let c = ConnectionPoolConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_timeout_configuration_default() {
        let c = TimeoutConfiguration::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_load_balancer_configuration_default() {
        let c = LoadBalancerConfiguration::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/retry.rs - 41 uncov
// ===========================================================================
mod retry_domain_defaults {
    use crate::canonical::config::domains::retry::*;

    #[test]
    fn test_canonical_retry_config_default() {
        let c = CanonicalRetryConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_canonical_retry_config_clone() {
        let c = CanonicalRetryConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/domains/monitoring_config.rs - 40 uncov
// ===========================================================================
mod monitoring_config_domain_defaults {
    use crate::canonical::config::domains::monitoring_config::*;

    #[test]
    fn test_consolidated_monitoring_config_default() {
        let c = ConsolidatedMonitoringConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/adapter/mod.rs - 60 uncov
// ===========================================================================
mod adapter_domain_mod_defaults {
    use crate::canonical::config::domains::adapter::*;

    #[test]
    fn test_unified_adapter_config_default() {
        let c = UnifiedAdapterConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/adapter/chain.rs - 74 uncov
// ===========================================================================
mod adapter_chain_defaults {
    use crate::canonical::config::domains::adapter::chain::*;

    #[test]
    fn test_chain_config_default() {
        let c = ChainConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_step_config_default() {
        let c = StepConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_retry_config_default() {
        let c = RetryConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/network.rs - 70 uncov
// ===========================================================================
mod canonical_network_defaults {
    use crate::canonical::network::*;
    use crate::canonical::traits::timeout::TimeoutPolicy;

    #[test]
    fn test_network_config_default() {
        let c = NetworkConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_timeout_config_default_and_validate() {
        let c = TimeoutConfig::default();
        let _ = format!("{c:?}");
        let _ = c.validate();
    }

    #[test]
    fn test_network_config_methods() {
        let c = NetworkConfig::default();
        let addr = c.bind_address();
        assert!(!addr.is_empty());
        let _ = c.is_tls_configured();
        let _ = c.validate();
    }
}

// ===========================================================================
// canonical/capabilities.rs - 70 uncov
// ===========================================================================
mod canonical_capabilities_more {
    use crate::canonical::capabilities::*;

    #[test]
    fn test_capability_type_name() {
        let types = [
            CapabilityType::Security,
            CapabilityType::KeyManagement,
            CapabilityType::HardwareSecurityModule,
            CapabilityType::SecretsManagement,
            CapabilityType::Authentication,
            CapabilityType::CloudStorage,
            CapabilityType::DatabaseService,
            CapabilityType::LoadBalancing,
            CapabilityType::ContentDeliveryNetwork,
            CapabilityType::ServiceMesh,
            CapabilityType::ComputeIntelligence,
            CapabilityType::DataStorage,
            CapabilityType::DistributedIntelligence,
            CapabilityType::ContainerOrchestration,
            CapabilityType::Monitoring,
            CapabilityType::Logging,
            CapabilityType::Metrics,
            CapabilityType::Storage,
            CapabilityType::Compute,
            CapabilityType::Network,
        ];
        for t in &types {
            let name = t.name();
            assert!(!name.is_empty());
            let id = t.as_capability_id();
            assert!(!id.is_empty());
        }
    }
}

// ===========================================================================
// canonical/hsm_unified/migration.rs - 146 uncov (more methods)
// ===========================================================================
mod hsm_unified_migration_methods {
    use crate::canonical::hsm_unified::migration::*;

    #[test]
    fn test_migrate_multiple_configs() {
        let service = HsmMigrationService::default();
        let configs = vec![
            LegacyHsmConfig::TunnelHsm {
                hardware_config: None,
                software_config: Some(std::collections::HashMap::new()),
                mobile_config: None,
            },
            LegacyHsmConfig::ConfigurationHsm {
                providers: vec![],
                monitoring: None,
                performance: None,
            },
            LegacyHsmConfig::ZeroCostHsm {
                manager_config: Default::default(),
            },
        ];
        let result = service.migrate_hsm_configs(configs);
        match result {
            Ok(r) => {
                assert!(r.migration_report.legacy_configs_processed > 0);
            }
            Err(e) => {
                let _ = format!("{e:?}");
            }
        }
    }

    #[test]
    fn test_create_tunnel_legacy_config_with_values() {
        let hw = serde_json::json!({"slot": 1});
        let sw = serde_json::json!({"path": "/tmp"});
        let c = create_tunnel_legacy_config(Some(hw), Some(sw), None);
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/monitoring_migration.rs - 145 uncov
// ===========================================================================
mod monitoring_migration_methods {
    use crate::canonical::config::monitoring_migration::*;

    #[test]
    fn test_monitoring_migration_service_default() {
        let _s = MonitoringMigrationService::default();
    }

    #[test]
    fn test_monitoring_migration_report() {
        let r = MonitoringMigrationReport {
            legacy_configs_processed: 2,
            successful_migrations: vec!["a".to_string()],
            warnings: vec![],
            errors: vec![],
            migrated_at: chrono::Utc::now(),
        };
        let _ = format!("{r:?}");
    }
}

// ===========================================================================
// canonical/providers_unified/resilience.rs - 147 uncov
// ===========================================================================
mod providers_resilience_defaults {
    use crate::canonical::providers_unified::resilience::*;
    use crate::canonical::traits::timeout::TimeoutPolicy;

    #[test]
    fn test_timeout_config_default() {
        let c = TimeoutConfig::default();
        let _ = format!("{c:?}");
        let _ = c.validate();
    }

    #[test]
    fn test_circuit_breaker_config_default() {
        let c = CircuitBreakerConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_retry_config_default() {
        let c = RetryConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_bulkhead_config_default() {
        let c = BulkheadConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/providers_unified/migration.rs - 100 uncov
// ===========================================================================
mod providers_migration_defaults {
    use crate::canonical::providers_unified::migration::*;

    #[test]
    fn test_service_health() {
        let h = ServiceHealth {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            response_time_ms: 5.0,
            error_message: None,
        };
        let _ = format!("{h:?}");
        // Test From<ServiceHealth> for ProviderHealth
        let ph: crate::canonical::providers_unified::traits::ProviderHealth = h.into();
        let _ = format!("{ph:?}");
    }

    #[test]
    fn test_cache_stats() {
        let s = CacheStats {
            hit_count: 100,
            miss_count: 10,
            size: 50,
            eviction_count: 5,
        };
        let _ = format!("{s:?}");
        // Test From<CacheStats> for ProviderMetrics
        let pm: crate::canonical::providers_unified::traits::ProviderMetrics = s.into();
        let _ = format!("{pm:?}");
    }
}

// ===========================================================================
// canonical/providers_unified/performance.rs - 65 uncov
// ===========================================================================
mod providers_performance_defaults {
    use crate::canonical::providers_unified::performance::*;

    #[test]
    fn test_performance_config_default() {
        let c = PerformanceConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_caching_config_default() {
        let c = CachingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_compression_config_default() {
        let c = CompressionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_buffer_config_default() {
        let c = BufferConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// genetics_constraints.rs - 79 uncov
// ===========================================================================
mod genetics_constraints_extra {
    use crate::genetics_constraints::*;

    #[test]
    fn test_key_operation_variants() {
        let ops: Vec<KeyOperation> = vec![
            KeyOperation::Read { path: "test".to_string(), project: None },
            KeyOperation::Write { path: "test".to_string(), size_bytes: 100, project: None },
            KeyOperation::Delete { path: "test".to_string() },
            KeyOperation::RpcCall { target_service: "svc".to_string(), method: "call".to_string(), project: None },
        ];
        for op in &ops {
            let _ = format!("{op:?}");
        }
    }
}

// ===========================================================================
// canonical/hsm_unified/cloud.rs - 71 uncov
// ===========================================================================
mod hsm_unified_cloud_defaults {
    use crate::canonical::hsm_unified::cloud::*;

    #[test]
    fn test_cloud_provider_variants() {
        let providers = [
            CloudProvider::Aws,
            CloudProvider::Azure,
            CloudProvider::Gcp,
        ];
        for p in &providers {
            let name = p.default_hsm_service_name();
            assert!(!name.is_empty());
            let _ = format!("{p:?}");
        }
    }
}

// ===========================================================================
// canonical/crypto.rs - 60 uncov
// ===========================================================================
mod canonical_crypto_defaults {
    use crate::canonical::config::domains::security::crypto::*;

    #[test]
    fn test_safe_crypto_configuration_default() {
        let c = SafeCryptoConfiguration::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_safe_crypto_configuration_clone() {
        let c = SafeCryptoConfiguration::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }

    #[test]
    fn test_genetic_renewal_configuration_default() {
        let c = GeneticRenewalConfiguration::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/bootstrap.rs - 37 uncov
// ===========================================================================
mod bootstrap_methods {
    use crate::canonical::config::domains::bootstrap::*;

    #[test]
    fn test_unified_bootstrap_config_validate() {
        use crate::canonical::config::r#trait::BearDogConfig;
        let c = UnifiedBootstrapConfig::default();
        let _ = c.validate();
    }
}

// ===========================================================================
// canonical/config/domains/ai_config/mod.rs - 29 uncov
// ===========================================================================
mod ai_config_mod_defaults {
    use crate::canonical::config::domains::ai_config::*;

    #[test]
    fn test_consolidated_ai_config_default() {
        let c = ConsolidatedAiConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/ai_config/training.rs - 24 uncov
// ===========================================================================
mod ai_training_defaults {
    use crate::canonical::config::domains::ai_config::training::*;

    #[test]
    fn test_training_config_default() {
        let c = TrainingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_training_config_clone() {
        let c = TrainingConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// canonical/config/domains/compliance.rs - 22 uncov
// ===========================================================================
mod compliance_defaults {
    use crate::canonical::config::domains::compliance::*;

    #[test]
    fn test_compliance_config_default() {
        let c = ConsolidatedComplianceConfiguration::default();
        let _ = format!("{c:?}");
        let _ = c.validate();
    }
}

// ===========================================================================
// canonical/config/domains/discovery_unified_builder.rs - 33 uncov
// ===========================================================================
mod discovery_builder_methods {
    use crate::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfigBuilder;

    #[test]
    fn test_builder_full() {
        let config = UnifiedDiscoveryConfigBuilder::new()
            .enabled(true)
            .service_id("svc-1")
            .from_env()
            .build();
        let _ = format!("{config:?}");
    }
}

// ===========================================================================
// canonical/hsm/config.rs - 124 uncov (more types)
// ===========================================================================
mod hsm_config_extra_defaults {
    use crate::canonical::hsm::config::*;

    #[test]
    fn test_hsm_config_default() {
        let c = HsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_software_hsm_config_default() {
        let c = SoftwareHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hardware_hsm_config_default() {
        let c = HardwareHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_network_hsm_config_default() {
        let c = NetworkHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_cloud_hsm_config_default() {
        let c = CloudHsmConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_provider_config_default() {
        let c = HsmProviderConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_auth_method_variants() {
        let methods = [
            AuthMethod::None,
            AuthMethod::Password { password: "test".to_string() },
            AuthMethod::Token { token_id: 1, pin: None },
            AuthMethod::Certificate { cert_path: "/tmp/cert.pem".to_string(), key_path: "/tmp/key.pem".to_string() },
        ];
        for m in &methods {
            let _ = format!("{m:?}");
        }
    }
}

// ===========================================================================
// canonical/config/domains/workflow/retry.rs - 49 uncov
// ===========================================================================
mod workflow_retry_defaults {
    use crate::canonical::config::domains::workflow::retry::*;

    #[test]
    fn test_retry_config_default() {
        let c = RetryConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_retry_config_clone() {
        let c = RetryConfig::default();
        let c2 = c.clone();
        let _ = format!("{c2:?}");
    }
}

// ===========================================================================
// production/types.rs - 42 uncov
// ===========================================================================
mod production_types_extra {
    use crate::production::types::*;

    #[test]
    fn test_environment_level_variants() {
        let levels = [
            EnvironmentLevel::Development,
            EnvironmentLevel::Staging,
            EnvironmentLevel::Production,
        ];
        for l in &levels {
            let _ = format!("{l:?}");
        }
    }

    #[test]
    fn test_operational_status_variants() {
        let statuses = [
            OperationalStatus::Initializing,
            OperationalStatus::Healthy,
            OperationalStatus::Degraded,
            OperationalStatus::Unhealthy,
            OperationalStatus::Critical,
            OperationalStatus::Shutdown,
        ];
        for s in &statuses {
            let _ = format!("{s:?}");
        }
    }
}

// ===========================================================================
// constants/domains/math.rs
// ===========================================================================
mod constants_math_tests {
    #[test]
    fn test_math_constants() {
        use crate::constants::domains::math;
        assert!(math::common::PI > 3.14);
        assert!(math::common::E > 2.71);
        assert!(math::common::TAU > 6.28);
        assert!(math::common::SQRT_2 > 1.41);
    }
}

// ===========================================================================
// constants/domains/system.rs
// ===========================================================================
mod constants_system_tests {
    #[test]
    fn test_system_constants() {
        use crate::constants::domains::system;
        assert!(system::defaults::DEFAULT_THREAD_POOL_SIZE > 0);
        assert!(system::defaults::DEFAULT_BUFFER_SIZE > 0);
        assert!(system::defaults::DEFAULT_CACHE_SIZE > 0);
        assert!(!system::versions::MIN_RUST_VERSION.is_empty());
        assert!(!system::versions::BEARDOG_VERSION.is_empty());
    }
}

// ===========================================================================
// constants/domains/pkcs11.rs
// ===========================================================================
mod constants_pkcs11_tests {
    #[test]
    fn test_pkcs11_constants() {
        use crate::constants::domains::pkcs11;
        assert_eq!(pkcs11::return_codes::CKR_OK, 0);
        let _ = pkcs11::return_codes::CKR_GENERAL_ERROR;
        let _ = pkcs11::object_classes::CKO_SECRET_KEY;
        let _ = pkcs11::object_classes::CKO_PUBLIC_KEY;
        let _ = pkcs11::object_classes::CKO_PRIVATE_KEY;
    }
}

// ===========================================================================
// canonical/config/hsm/discovery.rs - 33 uncov
// ===========================================================================
mod hsm_discovery_defaults {
    use crate::canonical::config::hsm::discovery::*;

    #[test]
    fn test_hsm_discovery_config_default() {
        let c = UnifiedHsmDiscoveryConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/hsm/performance.rs - 33 uncov
// ===========================================================================
mod hsm_performance_defaults {
    use crate::canonical::config::hsm::performance::*;

    #[test]
    fn test_hsm_performance_config_default() {
        let c = UnifiedHsmPerformanceConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/hsm/security.rs - 33 uncov
// ===========================================================================
mod hsm_security_defaults {
    use crate::canonical::config::hsm::security::*;

    #[test]
    fn test_hsm_security_config_default() {
        let c = UnifiedHsmSecurityConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/hsm/software.rs - 33 uncov
// ===========================================================================
mod hsm_software_defaults {
    use crate::canonical::config::hsm::software::*;

    #[test]
    fn test_unified_software_hsm_config_default() {
        let c = UnifiedSoftwareHsmConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/security/mfa.rs - 33 uncov
// ===========================================================================
mod security_mfa_defaults {
    use crate::canonical::config::security::mfa::*;

    #[test]
    fn test_canonical_mfa_config_default() {
        let c = CanonicalMfaConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_totp_config_default() {
        let c = TotpConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_sms_config_default() {
        let c = SmsConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_backup_codes_config_default() {
        let c = BackupCodesConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/analytics.rs - 33 uncov
// ===========================================================================
mod monitoring_analytics_defaults {
    use crate::canonical::monitoring::analytics::*;

    #[test]
    fn test_unified_analytics_config_default() {
        let c = UnifiedAnalyticsConfig::default();
        let _ = format!("{c:?}");
    }
}

mod monitoring_metrics_forecasting {
    use crate::canonical::monitoring::metrics::*;

    #[test]
    fn test_forecasting_config_default() {
        let c = ForecastingConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_anomaly_detection_config_default() {
        let c = AnomalyDetectionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_trend_analysis_config_default() {
        let c = TrendAnalysisConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/integration.rs - 33 uncov
// ===========================================================================
mod monitoring_integration_defaults {
    use crate::canonical::monitoring::integration::*;

    #[test]
    fn test_unified_integration_monitoring_config_default() {
        let c = UnifiedIntegrationMonitoringConfig::default();
        let _ = format!("{c:?}");
    }
}

mod monitoring_mod_exporter_defaults {
    use crate::canonical::monitoring::*;

    #[test]
    fn test_prometheus_exporter_config_default() {
        let c = PrometheusExporterConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_grafana_exporter_config_default() {
        let c = GrafanaExporterConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_jaeger_exporter_config_default() {
        let c = JaegerExporterConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_dashboard_config_default() {
        let c = DashboardConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_notification_config_default() {
        let c = NotificationConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/performance.rs - 33 uncov
// ===========================================================================
mod monitoring_performance_defaults {
    use crate::canonical::monitoring::performance::*;

    #[test]
    fn test_unified_performance_monitoring_config_default() {
        let c = UnifiedPerformanceMonitoringConfig::default();
        let _ = format!("{c:?}");
    }
}

mod monitoring_metrics_storage {
    use crate::canonical::monitoring::metrics::*;

    #[test]
    fn test_metric_storage_config_default() {
        let c = MetricStorageConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_partitioning_config_default() {
        let c = PartitioningConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/security.rs - 33 uncov
// ===========================================================================
mod monitoring_security_defaults {
    use crate::canonical::monitoring::security::*;

    #[test]
    fn test_unified_security_monitoring_config_default() {
        let c = UnifiedSecurityMonitoringConfig::default();
        let _ = format!("{c:?}");
    }
}

mod prod_secrets_config {
    use crate::canonical::config::production::environment::*;

    #[test]
    fn test_modern_secrets_config_default() {
        let c = ModernSecretsConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/tracing.rs - 21 uncov
// ===========================================================================
mod monitoring_tracing_defaults {
    use crate::canonical::monitoring::tracing::*;

    #[test]
    fn test_tracing_config_default() {
        let c = UnifiedTracingConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/security/encryption.rs - 40 uncov
// ===========================================================================
mod security_encryption_defaults {
    use crate::canonical::config::security::encryption::*;

    #[test]
    fn test_encryption_config_default() {
        let c = CanonicalEncryptionConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/security/session.rs - 40 uncov
// ===========================================================================
mod security_session_defaults {
    use crate::canonical::config::security::session::*;

    #[test]
    fn test_session_config_default() {
        let c = CanonicalSessionConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/unified/implementations.rs - 37 uncov
// ===========================================================================
mod unified_implementations_defaults {
    use crate::canonical::config::unified::UnifiedBearDogConfig;

    #[test]
    fn test_unified_beardog_config_default() {
        let c = UnifiedBearDogConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_unified_beardog_config_development() {
        let c = UnifiedBearDogConfig::development();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_unified_beardog_config_production() {
        let c = UnifiedBearDogConfig::production();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_unified_beardog_config_validate() {
        let c = UnifiedBearDogConfig::default();
        let _ = c.validate();
    }
}

// ===========================================================================
// canonical/config/security/mod.rs - 52 uncov
// ===========================================================================
mod security_mod_methods {
    use crate::canonical::config::security::*;

    #[test]
    fn test_canonical_security_config_validate() {
        let c = CanonicalSecurityConfig::default();
        let _ = c.validate();
    }
}

// ===========================================================================
// canonical/hsm_unified/providers.rs - 46 uncov
// ===========================================================================
mod hsm_unified_providers_defaults {
    use crate::canonical::hsm_unified::providers::*;

    #[test]
    fn test_hsm_provider_type_variants() {
        let types = [
            HsmProviderType::Software,
            HsmProviderType::Hardware { capabilities: vec![] },
            HsmProviderType::Network { capabilities: vec![] },
            HsmProviderType::Cloud { capabilities: vec![] },
        ];
        for t in &types {
            let _ = format!("{t:?}");
        }
        // Test Default
        let d = HsmProviderType::default();
        assert!(matches!(d, HsmProviderType::Software));
    }
}

// ===========================================================================
// workflow.rs - 41 uncov
// ===========================================================================
mod workflow_extra_tests {
    use crate::workflow::*;

    #[test]
    fn test_workflow_default() {
        let w = Workflow::default();
        let _ = format!("{w:?}");
    }

    #[test]
    fn test_workflow_step_manual() {
        let s = WorkflowStep {
            id: "step-1".to_string(),
            name: "test".to_string(),
            step_type: StepType::Action,
            configuration: Default::default(),
            dependencies: vec![],
            timeout_seconds: Some(30),
            retry_count: 3,
        };
        let _ = format!("{s:?}");
    }
}

// ===========================================================================
// canonical/config/compliance.rs - 21 uncov
// ===========================================================================
mod config_compliance_defaults {
    use crate::canonical::config::compliance::*;

    #[test]
    fn test_canonical_compliance_config_default() {
        let c = CanonicalComplianceConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/config/domains/security/advanced.rs - 17 uncov
// ===========================================================================
mod security_advanced_defaults {
    use crate::canonical::config::domains::security::advanced::*;

    #[test]
    fn test_genetic_security_configuration() {
        let c = GeneticSecurityConfiguration {
            enable_genetic_security: true,
            genetic_parameters: std::collections::HashMap::new(),
            evolution_strategies: vec!["strategy1".to_string()],
            fitness_criteria: vec!["criteria1".to_string()],
        };
        let _ = format!("{c:?}");
    }
}

