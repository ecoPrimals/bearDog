// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage gap tests Part 9: HSM config, HSM keys, HSM capabilities, monitoring core/health/metrics,
//! crypto, config/domains/system, config/hsm/mod, config/unified/simplified

// ===========================================================================
// canonical/config/hsm/mod.rs - 159 uncov (12 Default impls)
// ===========================================================================
mod config_hsm_mod_tests {
    use crate::canonical::config::hsm::*;

    #[test]
    fn test_unified_hsm_config_default() {
        let c = UnifiedHsmConfig::default();
        let _ = format!("{c:?}");
        let c2 = c;
        let _ = format!("{c2:?}");
    }

    #[test]
    fn test_hsm_retry_policy_default() {
        let c = HsmRetryPolicy::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_hsm_tier_management_default() {
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
// canonical/monitoring/core.rs - 152 uncov (5 Default impls)
// ===========================================================================
mod monitoring_core_tests {
    use crate::canonical::monitoring::core::*;

    #[test]
    fn test_core_monitoring_config_default() {
        let c = CoreMonitoringConfig::default();
        let _ = format!("{c:?}");
        let c2 = c;
        let _ = format!("{c2:?}");
    }

    #[test]
    fn test_storage_backend_default() {
        let s = StorageBackend::default();
        let _ = format!("{s:?}");
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

    #[test]
    fn test_metric_filter_manual() {
        let f = MetricFilter {
            name: "test".to_string(),
            pattern: "test.*".to_string(),
            conditions: vec![],
            action: FilterAction::Include,
        };
        let _ = format!("{f:?}");
    }

    #[test]
    fn test_filter_action_variants() {
        let _ = FilterAction::Include;
        let _ = FilterAction::Exclude;
    }

    #[test]
    fn test_comparison_operator_variants() {
        let ops = [
            ComparisonOperator::Equals,
            ComparisonOperator::NotEquals,
            ComparisonOperator::GreaterThan,
            ComparisonOperator::LessThan,
            ComparisonOperator::GreaterThanOrEqual,
            ComparisonOperator::LessThanOrEqual,
        ];
        for op in &ops {
            let _ = format!("{op:?}");
        }
    }

    #[test]
    fn test_compression_type_variants() {
        let types = [
            CompressionType::None,
            CompressionType::Gzip,
            CompressionType::Lz4,
            CompressionType::Zstd,
        ];
        for t in &types {
            let _ = format!("{t:?}");
        }
    }
}

// ===========================================================================
// canonical/monitoring/health.rs - 93 uncov (8 Default impls)
// ===========================================================================
mod monitoring_health_tests {
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
// canonical/hsm/keys.rs - 139 uncov (7 Default impls)
// ===========================================================================
mod hsm_keys_tests_extra {
    use crate::canonical::hsm::keys::*;

    #[test]
    fn test_hsm_key_default() {
        let k = HsmKey::default();
        let _ = format!("{k:?}");
        let k2 = k;
        let _ = format!("{k2:?}");
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
        let i = EncryptionInfo::default();
        let _ = format!("{i:?}");
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
// canonical/hsm/config.rs - 130 uncov (10 Default impls)
// ===========================================================================
mod hsm_config_tests_extra {
    use crate::canonical::hsm::config::*;

    #[test]
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    fn test_legacy_hsm_provider_type_default() {
        let t = LegacyHsmProviderType::default();
        let _ = format!("{t:?}");
    }

    #[test]
    fn test_connection_config_default() {
        let c = ConnectionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_config_default() {
        let c = SecurityConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_auth_method_default() {
        let a = AuthMethod::default();
        let _ = format!("{a:?}");
    }

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
}

// ===========================================================================
// canonical/hsm/capabilities.rs - 110 uncov (9 Default impls)
// ===========================================================================
mod hsm_capabilities_tests_extra {
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
// canonical/config/unified/simplified.rs - 104 uncov (6 Default impls)
// ===========================================================================
mod simplified_config_tests {
    use crate::canonical::config::unified::simplified::*;

    #[test]
    fn test_network_settings_default() {
        let c = NetworkSettings::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_security_settings_default() {
        let c = SecuritySettings::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_database_settings_default() {
        let c = DatabaseSettings::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_monitoring_settings_default() {
        let c = MonitoringSettings::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_performance_settings_default() {
        let c = PerformanceSettings::default();
        let _ = format!("{c:?}");
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
mod system_domain_config_tests {
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
// canonical/crypto.rs - 63 uncov (13 Default impls)
// ===========================================================================
mod crypto_types_tests {
    use crate::canonical::crypto::*;

    #[test]
    fn test_crypto_algorithm_default() {
        let a = CryptoAlgorithm::default();
        let _ = format!("{a:?}");
    }

    #[test]
    fn test_key_derivation_function_default() {
        let k = KeyDerivationFunction::default();
        let _ = format!("{k:?}");
    }

    #[test]
    fn test_key_config_default() {
        let c = KeyConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_usage_default() {
        let u = KeyUsage::default();
        let _ = format!("{u:?}");
    }

    #[test]
    fn test_encryption_mode_default() {
        let m = EncryptionMode::default();
        let _ = format!("{m:?}");
    }

    #[test]
    fn test_padding_scheme_default() {
        let p = PaddingScheme::default();
        let _ = format!("{p:?}");
    }

    #[test]
    fn test_hash_algorithm_default() {
        let h = HashAlgorithm::default();
        let _ = format!("{h:?}");
    }

    #[test]
    fn test_signature_config_default() {
        let c = SignatureConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_encryption_config_default() {
        let c = EncryptionConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_management_config_default() {
        let c = KeyManagementConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_rotation_policy_default() {
        let p = RotationPolicy::default();
        let _ = format!("{p:?}");
    }

    #[test]
    fn test_rng_config_default() {
        let c = RngConfig::default();
        let _ = format!("{c:?}");
    }

    #[test]
    fn test_key_pair_algorithm_default() {
        let a = KeyPairAlgorithm::default();
        let _ = format!("{a:?}");
    }
}

// ===========================================================================
// canonical/monitoring/metrics.rs - 50 uncov (14 Default impls)
// ===========================================================================
mod monitoring_metrics_tests {
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

    #[test]
    fn test_metric_export_config_default() {
        let c = MetricExportConfig::default();
        let _ = format!("{c:?}");
    }
}

// ===========================================================================
// canonical/monitoring/alerting.rs - 34 uncov (3 Default impls)
// ===========================================================================
mod monitoring_alerting_extra_tests {
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
// canonical/monitoring/mod.rs - 35 uncov (6 Default impls)
// ===========================================================================
mod monitoring_mod_extra_tests {
    use crate::canonical::monitoring::*;

    #[test]
    fn test_monitoring_config_default() {
        let c = MonitoringConfig::default();
        let _ = format!("{c:?}");
    }

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
